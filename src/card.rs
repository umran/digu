use std::convert::TryFrom;

pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl TryFrom<u8> for Value {
    type Error = ();

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(Self::Two),
            1 => Ok(Self::Three),
            2 => Ok(Self::Four),
            3 => Ok(Self::Five),
            4 => Ok(Self::Six),
            5 => Ok(Self::Seven),
            6 => Ok(Self::Eight),
            7 => Ok(Self::Nine),
            8 => Ok(Self::Ten),
            9 => Ok(Self::J),
            10 => Ok(Self::Q),
            11 => Ok(Self::K),
            12 => Ok(Self::A),
            _ => Err(()),
        }
    }
}

impl Value {
    pub fn index(&self) -> u8 {
        match self {
            Self::Two => 0,
            Self::Three => 1,
            Self::Four => 2,
            Self::Five => 3,
            Self::Six => 4,
            Self::Seven => 5,
            Self::Eight => 6,
            Self::Nine => 7,
            Self::Ten => 8,
            Self::J => 9,
            Self::Q => 10,
            Self::K => 11,
            Self::A => 12,
        }
    }
}

pub enum Card {
    Diamonds(Value),
    Hearts(Value),
    Clubs(Value),
    Spades(Value),
}

impl TryFrom<u8> for Card {
    type Error = ();

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        let value_index: u8 = index % 13;
        match index {
            0..=12 => Ok(Self::Diamonds(value_index.try_into().unwrap())),
            13..=25 => Ok(Self::Hearts(value_index.try_into().unwrap())),
            26..=38 => Ok(Self::Clubs(value_index.try_into().unwrap())),
            39..=51 => Ok(Self::Spades(value_index.try_into().unwrap())),
            _ => Err(()),
        }
    }
}

impl Card {
    pub fn suit_index(&self) -> u8 {
        match self {
            Self::Diamonds(_) => 0,
            Self::Hearts(_) => 1,
            Self::Clubs(_) => 2,
            Self::Spades(_) => 3,
        }
    }

    pub fn value_index(&self) -> u8 {
        match self {
            Self::Diamonds(v) | Self::Hearts(v) | Self::Clubs(v) | Self::Spades(v) => v.index(),
        }
    }

    pub fn index(&self) -> u8 {
        match self {
            Self::Diamonds(v) => v.index(),
            Self::Hearts(v) => v.index() + 13,
            Self::Clubs(v) => v.index() + 26,
            Self::Spades(v) => v.index() + 39,
        }
    }
}
