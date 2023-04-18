use crate::tracker::Results;

pub fn display(results: &Results) {

    // Print the final output to the terminal.
    println!("Total valid games:       {}", results.total_valid_games);
    println!("Total heads:             {}", results.total_heads);
    println!("Total tails:             {}", results.total_tails);
    println!("Percent of heads-heads:  {}", results.percent_heads_heads);
}

