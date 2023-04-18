#[derive(PartialEq)]
pub enum Coin {
    Heads,
    Tails,
}

impl Coin {

    // Returns random Coin.
    pub fn flip() -> Coin {

        // Generate random bool and return
        // the matching Heads / Tails.
        return match rand::random() {
            true => Coin::Heads,
            false => Coin::Tails,
        }
    }
}
