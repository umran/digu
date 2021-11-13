use super::digu::{eval_hand, Score};
use super::stack::{Stack, DECK};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum Action {
    InitiateDraw,
    FinalizeDraw(usize),
    Swap(usize),
}

#[derive(Serialize, Deserialize)]
pub struct Outcome {
    pub winner: u8,
    pub scores: Vec<Score>,
}

#[derive(Serialize, Deserialize)]
pub struct PublicState {
    pub active_player: u8,
    pub n_players: u8,
    pub n_deck: usize,
    pub n_pile: usize,
    pub pile_top: Option<u8>,
    pub outcome: Option<Outcome>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PrivateState {
    pub hand: [u8; 10],
    pub deck_top: Option<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    completed: bool,
    draw_in_progress: bool,
    active_player: u8,
    n_players: u8,
    deck: Stack,
    pile: Stack,
    hands: HashMap<u8, [u8; 10]>,
}

impl Game {
    pub fn new(n_players: u8) -> Result<(Self, PublicState, Vec<PrivateState>), String> {
        if n_players == 1 || n_players > 4 {
            return Err(String::from("Invalid number of players"));
        }

        let mut deck = Stack::new(DECK.to_vec());
        deck.shuffle();

        let pile = Stack::new(vec![]);

        let mut hands: HashMap<u8, [u8; 10]> = HashMap::new();
        for i in 0..n_players {
            let mut hand: [u8; 10] = [0; 10];
            for i in 0..10 {
                hand[i] = deck.deal();
            }
            hands.insert(i, hand);
        }

        let gme = Self {
            completed: false,
            draw_in_progress: false,
            active_player: 0,
            n_players,
            deck,
            pile,
            hands,
        };

        let public_state = PublicState {
            active_player: gme.active_player,
            n_players: gme.n_players,
            n_deck: gme.deck.len(),
            n_pile: gme.pile.len(),
            pile_top: None,
            outcome: None,
        };

        let mut private_states: Vec<PrivateState> = vec![];
        for i in 0..n_players {
            private_states.push(PrivateState {
                hand: *gme.hands.get(&i).unwrap(),
                deck_top: None,
            });
        }

        Ok((gme, public_state, private_states))
    }

    pub fn step(&mut self, action: Action) -> Result<(PublicState, PrivateState), String> {
        if self.completed {
            return Err(String::from("Game is already over"));
        }

        if self.draw_in_progress {
            if let Action::FinalizeDraw(_) = action {
                // noop
            } else {
                return Err(String::from("Invalid action. Expected FinalizeDraw"));
            }
        }

        let active_player = self.active_player;
        let hand = self.hands.get_mut(&active_player).unwrap();
        match action {
            Action::InitiateDraw => {
                self.draw_in_progress = true;
            }
            Action::FinalizeDraw(discarded_index) => {
                self.pile.stack(hand[discarded_index]);
                hand[discarded_index] = self.deck.deal();

                self.draw_in_progress = false;
            }
            Action::Swap(discarded_index) => {
                if self.pile.is_empty() {
                    return Err(String::from("Pile is empty, Please choose another action"));
                }

                let discarded_card = hand[discarded_index];
                hand[discarded_index] = self.pile.deal();
                self.pile.stack(discarded_card);
            }
        }

        if !self.draw_in_progress {
            // switch player
            self.active_player = (active_player + 1) % self.n_players; // switch player
        }

        // if the deck has run out, transfer pile to deck and shuffle
        if self.deck.is_empty() {
            self.deck = Stack::new(self.pile.dump());
            self.deck.shuffle();
        }

        // check win condition
        let score = eval_hand(hand);
        if score.winner {
            self.completed = true;
        }

        let public_state = PublicState {
            active_player: self.active_player,
            n_players: self.n_players,
            n_deck: self.deck.len(),
            n_pile: self.pile.len(),
            pile_top: match self.pile.is_empty() {
                true => None,
                false => Some(self.pile.top()),
            },
            outcome: match score.winner {
                true => {
                    let mut outcome = Outcome {
                        winner: active_player,
                        scores: vec![],
                    };

                    for p in 0..self.n_players {
                        let hand = self.hands.get(&p).unwrap();
                        let score = eval_hand(hand);
                        outcome.scores.push(score);
                    }

                    Some(outcome)
                }
                false => None,
            },
        };

        let private_state = PrivateState {
            hand: *self.hands.get(&self.active_player).unwrap(),
            deck_top: match self.draw_in_progress {
                true => Some(self.deck.top()),
                false => None,
            },
        };

        Ok((public_state, private_state))
    }
}
