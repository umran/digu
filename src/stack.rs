use rand::prelude::*;
use rand::rngs::EntropyRng;
use rand::seq::sample_indices;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const DECK: [u8; 52] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51,
];

#[derive(Serialize, Deserialize, Clone)]
pub struct Stack {
    cards: Vec<u8>,
}

impl Stack {
    pub fn new(cards: Vec<u8>) -> Self {
        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut working_set: HashSet<u8> = self.cards.iter().copied().collect();
        let mut rng = StdRng::from_rng(EntropyRng::new()).unwrap();

        for i in 0..self.cards.len() {
            let vectorized: Vec<u8> = working_set.iter().copied().collect();
            let sample_i = sample_indices(&mut rng, self.cards.len() - i, 1);
            let selection = vectorized[sample_i[0]];
            self.cards[i] = selection;

            working_set.remove(&selection);
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.len() == 0
    }

    pub fn top(&self) -> Option<u8> {
        match self.cards.len() > 0 {
            true => Some(self.cards[self.cards.len() - 1]),
            false => None,
        }
    }

    pub fn deal(&mut self) -> Option<u8> {
        self.cards.pop()
    }

    pub fn stack(&mut self, card: u8) {
        self.cards.push(card);
    }

    pub fn dump(&mut self) -> Vec<u8> {
        self.cards.drain(..).collect()
    }
}
