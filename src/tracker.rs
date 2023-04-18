use crate::coin::Coin;

// Trackers keep the scores of games.
pub struct Tracker {
    pub heads_heads: u32,
    pub heads_tails: u32,
    pub tails_heads: u32,
    pub tails_tails: u32,
}

pub struct Results {
    pub total_heads:         u32,
    pub total_tails:         u32,
    pub total_valid_games:   u32,
    pub percent_heads_heads: f64,
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            heads_heads: 0,
            heads_tails: 0,
            tails_heads: 0,
            tails_tails: 0,
        }
    }

    // Updates a Tracker with the results of a single game.
    pub fn update(&mut self, first_coin: &Coin, second_coin: &Coin) {
        match (first_coin, second_coin) {
            (Coin::Heads, Coin::Heads) => self.heads_heads += 1,
            (Coin::Heads, Coin::Tails) => self.heads_tails += 1,
            (Coin::Tails, Coin::Heads) => self.tails_heads += 1,
            (Coin::Tails, Coin::Tails) => self.tails_tails += 1,
        }
    }

    // Adds all fields of one Tracker to another.
    pub fn absorb(&mut self, child: Tracker) {
        self.heads_heads += child.heads_heads;
        self.heads_tails += child.heads_tails;
        self.tails_heads += child.tails_heads;
        self.tails_tails += child.tails_tails;
    }

    // Calculate and return Results.
    pub fn get_results(&self) -> Results {
        // Sum up all Coins that landed Heads up.
        let total_heads: u32 =
            self.heads_heads * 2 +
            self.heads_tails +
            self.tails_heads;

        // Sum up all Coins that landed Tails up
        let total_tails =
            self.heads_tails +
            self.tails_heads;

        // Sum up all valid games.
        // All games are valid except for the Tails-Tails permutation.
        let total_valid_games: u32 =
            self.heads_heads +
            self.heads_tails +
            self.tails_heads;

        // Get percentige of Heads-Heads of all valid games.
        let percent_heads_heads: f64 =
            self.heads_heads as f64 /
            total_valid_games as f64 *
            100.0;

        // Return new Results.
        Results {
            total_heads,
            total_tails,
            total_valid_games,
            percent_heads_heads,
        }
    }
}
