use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameId {
    Flux,
    SudoRush,
    Stackfall,
    Diceforge,
}

impl GameId {
    pub fn min_players(self) -> usize {
        match self {
            Self::Flux => 2,
            Self::SudoRush => 1,
            Self::Stackfall => 1,
            Self::Diceforge => 1,
        }
    }

    pub fn max_players(self) -> usize {
        4
    }

    pub fn is_available(self) -> bool {
        matches!(self, Self::Flux)
    }
}
