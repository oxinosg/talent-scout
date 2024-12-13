use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
};

pub type PlayerId = String;

/// PlayerDB is a database of players with their stats
pub struct PlayerDB {
    pub players: HashMap<PlayerId, Player>,
    pub top_scorers: Vec<PlayerId>,
    pub top_assists: Vec<PlayerId>,
}

impl Default for PlayerDB {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerDB {
    /// Create a new [`PlayerDB`]
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            top_scorers: Vec::new(),
            top_assists: Vec::new(),
        }
    }

    /// Add [`Player`] to list
    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.id.clone(), player);
    }

    /// Index top_scorers and top_assists
    pub fn index_players(&mut self) {
        // calculate top scorers and add ids to top_scorers, descending order by
        // goals_scored limited to 10 players
        let mut top_scorers: BTreeSet<PlayerWithScore> = BTreeSet::new();
        for (id, player) in self.players.iter() {
            top_scorers.insert(PlayerWithScore {
                id: id.clone(),
                score: player.goals_scored as u32,
            });
        }

        // take more than 10 if they are tied
        self.top_scorers = top_scorers.into_iter().take(10).map(|p| p.id).collect();

        // calculate top assists and add ids to top_assists, descending order by
        // assists limited to 10 players
        let mut top_assists: BTreeSet<PlayerWithAssists> = BTreeSet::new();
        for (id, player) in self.players.iter() {
            top_assists.insert(PlayerWithAssists {
                id: id.clone(),
                assists: player.assists as u32,
            });
        }
        self.top_assists = top_assists.into_iter().take(10).map(|p| p.id).collect();
    }

    /// Print top scorers
    pub fn print_top_scorers(&self) {
        if self.top_scorers.is_empty() {
            println!("No top scorers found");

            return;
        }

        println!();
        println!();
        println!();
        println!("Top Scorers:");

        let mut position = 1;
        let mut prev_score = 0;
        for (index, id) in self.top_scorers.iter().enumerate() {
            let player = self.players.get(id).unwrap();

            if index == 0 {
                prev_score = player.goals_scored
            }

            if prev_score > player.goals_scored {
                position += 1;
                prev_score = player.goals_scored;
            }

            println!(
                "{}: {} ({} goals)",
                position, player.name, player.goals_scored
            );
        }
    }

    /// Print top assists
    pub fn print_top_assists(&self) {
        if self.top_assists.is_empty() {
            println!("No top assists found");

            return;
        }

        println!();
        println!();
        println!();
        println!("Top Assists:");

        let mut position = 1;
        let mut prev_assists = 0;

        for (index, id) in self.top_assists.iter().enumerate() {
            let player = self.players.get(id).unwrap();

            if index == 0 {
                prev_assists = player.assists;
            }

            if prev_assists > player.assists {
                position += 1;
                prev_assists = player.assists;
            }

            println!("{}: {} ({} assists)", position, player.name, player.assists);
        }
    }
}

/// Player struct
pub struct Player {
    pub id: String,
    pub name: String,
    pub goals_scored: u16,
    pub assists: u16,
}

/// Temporary struct to hold player id and score for sorting purposes in
/// BTreeSet
#[derive(Debug, Eq, PartialEq)]
struct PlayerWithScore {
    id: String,
    score: u32,
}

impl Ord for PlayerWithScore {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for PlayerWithScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PlayerWithAssists {
    id: String,
    assists: u32,
}

impl Ord for PlayerWithAssists {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .assists
            .cmp(&self.assists)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for PlayerWithAssists {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test player db works
    #[test]
    fn test_player_db() {
        let mut player_db = PlayerDB::new();

        let player1 = Player {
            id: "1".to_string(),
            name: "Player 1".to_string(),
            goals_scored: 10,
            assists: 5,
        };

        let player2 = Player {
            id: "2".to_string(),
            name: "Player 2".to_string(),
            goals_scored: 5,
            assists: 10,
        };

        player_db.add_player(player1);
        player_db.add_player(player2);

        player_db.index_players();

        assert_eq!(player_db.top_scorers.len(), 2);
        assert_eq!(player_db.top_assists.len(), 2);

        assert_eq!(player_db.top_scorers[0], "1");
        assert_eq!(player_db.top_assists[0], "2");
    }

    // test player db limits top scorers and top assists to 10
    #[test]
    fn test_player_db_limit() {
        let mut player_db = PlayerDB::new();

        for i in 0..20 {
            let player = Player {
                id: i.to_string(),
                name: format!("Player {}", i),
                goals_scored: i as u16,
                assists: i as u16,
            };

            player_db.add_player(player);
        }

        player_db.index_players();

        assert_eq!(player_db.top_scorers.len(), 10);
        assert_eq!(player_db.top_assists.len(), 10);
    }
}
