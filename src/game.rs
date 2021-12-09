use super::digu::{eval_hand, Score};
use super::stack::{Stack, DECK};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum Action {
    InitiateDraw,
    FinalizeDraw(Option<usize>),
    Swap(usize),
    Forfeit,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct Outcome {
    pub winner: u8,
    pub scores: Vec<Score>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PublicState {
    pub completed: bool,
    pub steps: u32,
    pub draw_in_progress: bool,
    pub active_player: u8,
    pub n_players: u8,
    pub forfeitures: HashSet<u8>,
    pub pile: Vec<u8>,
    pub outcome: Option<Outcome>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct PrivateState {
    pub hand: [u8; 10],
    pub deck_top: Option<u8>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Game {
    completed: bool,
    steps: u32,
    draw_in_progress: bool,
    active_player: u8,
    n_players: u8,
    forfeitures: HashSet<u8>,
    deck: Stack,
    pile: Stack,
    hands: HashMap<u8, [u8; 10]>,
    outcome: Option<Outcome>,
}

impl Game {
    pub fn new(n_players: u8) -> Result<(Self, PublicState, Vec<PrivateState>), String> {
        if n_players == 1 || n_players > 4 {
            return Err(String::from("Invalid number of players"));
        }

        let mut deck = Stack::new(DECK.to_vec());
        deck.shuffle();

        let mut hands: HashMap<u8, [u8; 10]> = HashMap::new();
        for i in 0..n_players {
            let mut hand: [u8; 10] = [0; 10];
            for i in 0..10 {
                hand[i] = deck.deal().unwrap();
            }
            hands.insert(i, hand);
        }

        let forfeitures: HashSet<u8> = HashSet::new();
        let pile = Stack::new(vec![]);

        let gme = Self {
            completed: false,
            steps: 0,
            draw_in_progress: false,
            active_player: 0,
            n_players,
            forfeitures: forfeitures.clone(),
            deck,
            pile: pile.clone(),
            hands,
            outcome: None,
        };

        let public_state = PublicState {
            completed: gme.completed,
            steps: 0,
            draw_in_progress: gme.draw_in_progress,
            active_player: gme.active_player,
            n_players: gme.n_players,
            forfeitures: forfeitures,
            pile: pile.list(),
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

    pub fn step(&mut self, action: Action) -> Result<(PublicState, Vec<PrivateState>), String> {
        if self.completed {
            return Err(String::from("Game is already over"));
        }

        // the only allowed actions while draw is in progress are FinalizeDraw and Forfeit
        if self.draw_in_progress {
            if !(matches!(action, Action::FinalizeDraw(_)) || matches!(action, Action::Forfeit)) {
                return Err(String::from(
                    "Invalid action. Expected FinalizeDraw or Forfeit",
                ));
            }
        }

        let active_player = self.active_player;
        let hand = self.hands.get_mut(&active_player).unwrap();
        match action {
            Action::InitiateDraw => {
                self.draw_in_progress = true;
            }
            Action::FinalizeDraw(possible_discarded_index) => {
                if let Some(discarded_index) = possible_discarded_index {
                    match discarded_index {
                        0..=9 => {
                            self.pile.stack(hand[discarded_index]);
                            hand[discarded_index] = self.deck.deal().unwrap();
                        }
                        _ => {
                            return Err(String::from(
                                "Invalid index, please provide a value between 0 and 9",
                            ));
                        }
                    }
                } else {
                    self.pile.stack(self.deck.deal().unwrap());
                }

                self.draw_in_progress = false;
            }
            Action::Swap(discarded_index) => {
                if self.pile.is_empty() {
                    return Err(String::from("Pile is empty, Please choose another action"));
                }

                match discarded_index {
                    0..=9 => {
                        let discarded_card = hand[discarded_index];
                        hand[discarded_index] = self.pile.deal().unwrap();
                        self.pile.stack(discarded_card);
                    }
                    _ => {
                        return Err(String::from(
                            "Invalid index, please provide a value between 0 and 9",
                        ))
                    }
                }
            }
            Action::Forfeit => {
                for (_, &c) in hand.iter().enumerate() {
                    self.deck.stack(c);
                }
                self.deck.shuffle();
                self.forfeitures.insert(active_player);
                self.draw_in_progress = false;
            }
        }

        self.steps += 1;

        if !self.draw_in_progress {
            loop {
                self.active_player = (self.active_player + 1) % self.n_players;
                if !self.forfeitures.contains(&self.active_player) {
                    break;
                }
                if self.active_player == active_player {
                    break;
                }
            }
        }

        // if the deck has run out, transfer pile to deck and shuffle
        if self.deck.is_empty() {
            self.deck = Stack::new(self.pile.dump());
            self.deck.shuffle();
        }

        // check win condition
        let score = eval_hand(hand);
        if score.winner || self.forfeitures.len() == usize::from(self.n_players - 1) {
            let mut outcome = Outcome {
                winner: active_player,
                scores: vec![],
            };

            for p in 0..self.n_players {
                if p == active_player {
                    outcome.scores.push(score.clone());
                    continue;
                }

                let hand = self.hands.get(&p).unwrap();
                let score = eval_hand(hand);
                outcome.scores.push(score);
            }

            self.outcome = Some(outcome);
            self.completed = true;
        }

        let public_state = PublicState {
            completed: self.completed,
            steps: self.steps,
            draw_in_progress: self.draw_in_progress,
            active_player: self.active_player,
            n_players: self.n_players,
            forfeitures: self.forfeitures.clone(),
            pile: self.pile.list(),
            outcome: self.outcome.clone(),
        };

        let mut private_states: Vec<PrivateState> = vec![];
        for i in 0..self.n_players {
            private_states.push(PrivateState {
                hand: *self.hands.get(&i).unwrap(),
                deck_top: match self.draw_in_progress && i == active_player {
                    true => Some(self.deck.top().unwrap()),
                    false => None,
                },
            });
        }

        Ok((public_state, private_states))
    }
}
