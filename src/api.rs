use std::env;

use anyhow::Result;
use chrono::NaiveDate;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{
    utils::{deserialize_date, serialize_date},
    ACCOUNT_ACCESS_LEVEL,
    API_BASE_URL,
};

/// API endpoints for the soccer API
pub struct ApiEndpoints {
    base_url: Url,
}

impl ApiEndpoints {
    /// Create a new instance of the API endpoints
    pub fn new() -> Result<Self, url::ParseError> {
        let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| API_BASE_URL.into());
        let access_level =
            env::var("ACCOUNT_ACCESS_LEVEL").unwrap_or_else(|_| ACCOUNT_ACCESS_LEVEL.into());

        Ok(ApiEndpoints {
            base_url: Url::parse(&format!("{}/soccer/{}/v4/en/", base_url, access_level))?,
        })
    }

    /// Get the URL for the competition seasons endpoint
    pub fn competition_seasons(&self, competition_id: &str) -> Result<Url, url::ParseError> {
        self.base_url
            .join(&format!("competitions/{}/seasons.json", competition_id))
    }

    /// Get the URL for the season competitions endpoint
    pub fn season_competitors(&self, season_id: &str) -> Result<Url, url::ParseError> {
        self.base_url
            .join(&format!("seasons/{}/competitors.json", season_id))
    }

    /// Get the URL for the competitor statistics endpoint
    pub fn competitor_statistics(
        &self,
        season_id: &str,
        competition_id: &str,
    ) -> Result<Url, url::ParseError> {
        self.base_url.join(&format!(
            "seasons/{}/competitors/{}/statistics.json",
            season_id, competition_id
        ))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiCompetitionSeasons {
    pub seasons: Vec<ApiCompetitionSeason>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiCompetitionSeason {
    pub id: String,
    pub name: String,
    pub year: String,
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    pub start_date: NaiveDate,
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    pub end_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiSeasonSchedules {
    pub schedules: Vec<ApiSportEvent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiSportEvent {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiSeasonCompetitors {
    pub season_competitors: Vec<ApiSeasonCompetitor>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiSeasonCompetitor {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiSeasonCompetitorStatistics {
    pub competitor: ApiSeasonCompetitorStatistic,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiSeasonCompetitorStatistic {
    pub id: String,
    pub statistics: ApiGameStatistic,
    pub players: Vec<ApiGamePlayers>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiGameStatistic {
    pub goals_scored: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiGamePlayers {
    pub id: String,
    pub name: String,
    pub statistics: ApiPlayerGameStatistic,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiPlayerGameStatistic {
    pub goals_scored: u16,
    pub assists: u16,
}
