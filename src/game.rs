use super::card::Card;
use super::digu::score_hand;
use super::stack::{Stack, DECK};
use dialoguer::{theme::ColorfulTheme, Select, Sort};
use std::collections::HashMap;

pub struct Game {
    n_players: u8,
    deck: Stack,
    pile: Stack,
    hands: HashMap<u8, [u8; 10]>,
}

impl Game {
    pub fn new(n_players: u8) -> Self {
        if n_players == 1 || n_players > 4 {
            panic!("Invalid number of players");
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

        Self {
            n_players,
            deck,
            pile,
            hands,
        }
    }

    pub fn run(&mut self) {
        let mut active_player: u8 = 0;
        loop {
            println!("\n  player {}, your move\n", active_player);

            let hand = self.hands.get_mut(&active_player).unwrap();
            let mut hand_labels: Vec<String> = hand
                .to_vec()
                .iter()
                .map(|&index| Card::new_from_index(index).unwrap().name())
                .collect();

            // prompt active player for move
            let mut options = vec![
                "View and sort hand".to_string(),
                "Pick card from deck".to_string(),
            ];

            if !self.pile.is_empty() {
                options.push(format!(
                    "Pick {} from pile",
                    Card::new_from_index(self.pile.top()).unwrap().name()
                ))
            }

            loop {
                let option = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Choose an option")
                    .default(0)
                    .items(&options[..])
                    .interact()
                    .unwrap();

                if option == 0 {
                    let sorted_indices = Sort::with_theme(&ColorfulTheme::default())
                        .with_prompt("View and sort hand")
                        .items(&hand_labels[..])
                        .interact()
                        .unwrap();

                    let hand_copy = hand.to_vec();
                    for (i, &j) in sorted_indices.iter().enumerate() {
                        hand[i] = hand_copy[j];
                    }

                    hand_labels = hand
                        .to_vec()
                        .iter()
                        .map(|&index| Card::new_from_index(index).unwrap().name())
                        .collect();

                    continue;
                }

                if option == 1 || option == 2 {
                    let mut incoming_card_name =
                        Card::new_from_index(self.deck.top()).unwrap().name();
                    if option == 2 {
                        incoming_card_name = Card::new_from_index(self.pile.top()).unwrap().name();
                    }

                    let discarded_index = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt(
                            format!("Choose card to discard in place of {}", incoming_card_name)
                                .as_str(),
                        )
                        .default(0)
                        .items(&hand_labels[..])
                        .interact()
                        .unwrap();

                    if option == 1 {
                        let new_card = self.deck.deal();
                        self.pile.stack(hand[discarded_index]);
                        hand[discarded_index] = new_card;
                    }

                    if option == 2 {
                        let new_card = self.pile.deal();
                        self.pile.stack(hand[discarded_index]);
                        hand[discarded_index] = new_card;
                    }

                    break;
                }
            }

            // do at end of every turn
            // check win condition
            let outcome = score_hand(hand);
            if outcome.complete {
                println!("\n  player {} has won the game\n", active_player);

                for p in 0..self.n_players {
                    let hand = self.hands.get(&p).unwrap();
                    let outcome = score_hand(hand);

                    println!("      player {} score: {}", p, outcome.score);
                }

                break;
            }

            // switch player
            active_player = (active_player + 1) % self.n_players; // switch player

            // if the deck has run out, transfer pile to deck and shuffle
            if self.deck.is_empty() {
                self.deck = Stack::new(self.pile.dump());
                self.deck.shuffle();
            }
        }
    }
}
