use std::{collections::HashMap, env, fs};

use anyhow::{Context, Result};
use api::{
    ApiCompetitionSeasons,
    ApiEndpoints,
    ApiSeasonCompetitorStatistics,
    ApiSeasonCompetitors,
};
use player::{Player, PlayerDB};
use reqwest::Client;
use utils::fetch_data;

pub mod api;
pub mod player;
pub mod utils;

// Default values for environment variables
pub const API_BASE_URL: &str = "https://api.sportradar.com";
pub const COMPETITION_ID: &str = "sr:competition:17";
pub const CACHE_LOCATION: &str = "cache";
pub const ACCOUNT_ACCESS_LEVEL: &str = "trial";

#[tokio::main]
async fn main() -> Result<()> {
    // Create an HTTP client
    let client = Client::new();

    let mut player_db: PlayerDB = PlayerDB::new();

    let api_endpoints = ApiEndpoints::new()?;

    // Set up environment variables
    let api_key = env::var("API_KEY").context("API_KEY environment variable not found")?;
    let competition_id = env::var("COMPETITION_ID").unwrap_or_else(|_| COMPETITION_ID.to_string());
    let cache_location = env::var("CACHE_LOCATION").unwrap_or_else(|_| CACHE_LOCATION.to_string());
    let access_level =
        env::var("ACCOUNT_ACCESS_LEVEL").unwrap_or_else(|_| ACCOUNT_ACCESS_LEVEL.into());

    // Define query parameters for the request authorization
    let query_params = HashMap::from([("api_key", api_key.as_str())]);

    // Define headers for the request
    let headers = HashMap::from([("Accept", "application/json"), ("User-Agent", "reqwest")]);

    // Ensure cache directory exists
    fs::create_dir_all(&cache_location).context("Failed to create cache directory")?;

    let competition_seasons_url = api_endpoints.competition_seasons(competition_id.as_str())?;
    let competition_seasons_cache_path = &format!("{}/competition_seasons.json", cache_location);

    // 1. call the API to get the season id from the competition_seasons
    let competition_seasons = fetch_data::<ApiCompetitionSeasons>(
        &client,
        competition_seasons_url.as_str(),
        &query_params,
        &headers,
        competition_seasons_cache_path,
        access_level.as_str(),
    )
    .await
    .context("Failed to fetch competition seasons")?;

    // extract the season id for the Premier League 23/24
    let season = competition_seasons
        .seasons
        .iter()
        .find(|s| s.name == "Premier League 23/24")
        .expect("Season not found");

    let season_competitors_url = api_endpoints.season_competitors(&season.id)?;
    let competitors_cache_path = &format!("{}/season_competitors.json", cache_location);

    // 2. get competitors from for the season
    let competitors = fetch_data::<ApiSeasonCompetitors>(
        &client,
        season_competitors_url.as_str(),
        &query_params,
        &headers,
        competitors_cache_path,
        access_level.as_str(),
    )
    .await
    .context("Failed to fetch season competitors")?;

    // 3. get statistics for each competitor
    for competitor in competitors.season_competitors.iter() {
        let competitor_statistics_url =
            api_endpoints.competitor_statistics(&season.id, &competitor.id)?;

        let stats_cache_path = &format!("{}/stats_{}.json", cache_location, competitor.id);

        // fetch data for each competitor
        match fetch_data::<ApiSeasonCompetitorStatistics>(
            &client,
            competitor_statistics_url.as_str(),
            &query_params,
            &headers,
            stats_cache_path,
            access_level.as_str(),
        )
        .await
        {
            Ok(data) => {
                // skip competitors with no goals
                if data.competitor.players.is_empty() {
                    continue;
                }

                // loop through data to create Player data
                for player in data.competitor.players.iter() {
                    // skip players with 0 goals
                    if player.statistics.goals_scored == 0 {
                        continue;
                    }

                    let player: Player = Player {
                        id: player.id.clone(),
                        name: player.name.clone(),
                        goals_scored: player.statistics.goals_scored,
                        assists: player.statistics.assists,
                    };

                    // add player to player database
                    player_db.add_player(player);
                }

                // index players to keep track of top scorers and assists
                player_db.index_players();

                Ok(())
            },
            Err(err) => Err(err),
        }?;
    }

    // Print top scorers and assists
    player_db.print_top_scorers();
    player_db.print_top_assists();

    Ok(())
}
