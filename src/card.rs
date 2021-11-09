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
    pub fn name(&self) -> String {
        match self {
            Self::Two => String::from("2"),
            Self::Three => String::from("3"),
            Self::Four => String::from("4"),
            Self::Five => String::from("5"),
            Self::Six => String::from("6"),
            Self::Seven => String::from("7"),
            Self::Eight => String::from("8"),
            Self::Nine => String::from("9"),
            Self::Ten => String::from("10"),
            Self::J => String::from("J"),
            Self::Q => String::from("Q"),
            Self::K => String::from("K"),
            Self::A => String::from("A"),
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
    pub fn new_from_index(index: u8) -> Result<Self, ()> {
        let card: Self = index.try_into().map_err(|_| ())?;
        Ok(card)
    }

    pub fn name(&self) -> String {
        match self {
            Self::Diamonds(v) => format!("{} of Diamonds", v.name()),
            Self::Hearts(v) => format!("{} of Hearts", v.name()),
            Self::Clubs(v) => format!("{} of Clubs", v.name()),
            Self::Spades(v) => format!("{} of Spades", v.name()),
        }
    }
}
