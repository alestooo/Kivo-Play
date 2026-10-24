use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClashColor {
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClashCardColor {
    Red,
    Blue,
    Green,
    Yellow,
    Wild,
}

impl ClashCardColor {
    pub fn normal_color(self) -> Option<ClashColor> {
        match self {
            Self::Red => Some(ClashColor::Red),

            Self::Blue => Some(ClashColor::Blue),

            Self::Green => Some(ClashColor::Green),

            Self::Yellow => Some(ClashColor::Yellow),

            Self::Wild => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FluxSpecial {
    DrawTwo,
    Reverse,
    Wild,
    DrawFour,
    DrawSix,
    Steal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FluxCardKind {
    Number { value: u8 },

    Special { special: FluxSpecial },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FluxCard {
    pub id: Uuid,

    pub color: ClashCardColor,

    #[serde(flatten)]
    pub kind: FluxCardKind,
}
