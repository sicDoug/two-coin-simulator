//
// THIS CRATE IS NOT USED
//

use crate::coin::Coin;

pub struct Game {
    pub first: Coin,
    pub second: Coin,
}

impl Game {
    pub fn play() -> Self {
        Self {
            first: Coin::flip(),
            second: Coin::flip(),
        }
    }
}
