use crate::tracker::Tracker;

pub fn display(tracker: &Tracker) {

    // Sum up all Coins that landed Heads up.
    let total_heads = tracker.heads_heads * 2 +
        tracker.heads_tails +
        tracker.tails_heads;

    // Sum up all Coins that landed Tails up
    let total_tails = tracker.heads_tails + tracker.tails_heads;

    // Sum up all valid games.
    // All games are valid except for the Tails-Tails permutation.
    let total_valid_games: u32 =
        tracker.heads_heads +
        tracker.heads_tails +
        tracker.tails_heads;

    // Get percentige of Heads-Heads of all valid games.
    let percent_heads_heads =
        tracker.heads_heads as f32 /
        total_valid_games as f32 *
        100.0;

    println!("Total valid games:       {}", total_valid_games);
    println!("Total Heads:             {}", total_heads);
    println!("Total Tails:             {}", total_tails);
    println!("Total Heads-Heads:       {}", tracker.heads_heads);
    println!("Percent of Heads-Heads:  {}", percent_heads_heads);
}

