use dialoguer::{theme::ColorfulTheme, Select, Sort};
use digu::card::Card;
use digu::game::{Action, Game};
use std::collections::HashMap;

fn main() {
    // prompt user to select number of players
    let possible_player_counts: &[u8] = &[2, 3, 4];

    let player_count_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please choose the number of players")
        .default(0)
        .items(&possible_player_counts[..])
        .interact()
        .unwrap();

    let (mut game, mut public_state, mut private_states) =
        Game::new(possible_player_counts[player_count_index]).unwrap();

    let mut hand_orderings: HashMap<u8, Vec<usize>> = HashMap::new();
    for p in 0..public_state.n_players {
        hand_orderings.insert(p, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    loop {
        println!("\n  player {}, your move\n", public_state.active_player);

        let active_player_index: usize = public_state.active_player.into();
        let private_state = private_states[active_player_index].clone();

        loop {
            let mut options = vec![
                "View and sort hand".to_string(),
                "Pick card from deck".to_string(),
            ];

            if let Some(pile_top) = public_state.pile.top() {
                options.push(format!(
                    "Pick {} from pile",
                    Card::new_from_index(pile_top).unwrap().name()
                ));
            }

            let option = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose an option")
                .default(0)
                .items(&options[..])
                .interact()
                .unwrap();

            let hand_labels: Vec<String> = hand_orderings
                .get(&public_state.active_player)
                .unwrap()
                .iter()
                .map(|&index| {
                    Card::new_from_index(private_state.hand[index])
                        .unwrap()
                        .name()
                })
                .collect();

            if option == 0 {
                let sorted_indices = Sort::with_theme(&ColorfulTheme::default())
                    .with_prompt("View and sort hand")
                    .items(&hand_labels[..])
                    .interact()
                    .unwrap();

                let ordering = hand_orderings.get_mut(&public_state.active_player).unwrap();
                let ordering_copy = ordering.clone();
                for (i, &j) in sorted_indices.iter().enumerate() {
                    ordering[i] = ordering_copy[j];
                }
                continue;
            }

            if option == 1 {
                let (_, next_private_states) = game.step(Action::InitiateDraw).unwrap();
                let next_private_state = next_private_states[active_player_index].clone();

                let incoming_card_name = Card::new_from_index(next_private_state.deck_top.unwrap())
                    .unwrap()
                    .name();

                let should_swap = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        format!("You drew {}. Would you like to swap?", &incoming_card_name)
                            .as_str(),
                    )
                    .default(0)
                    .items(&vec!["Yes".to_string(), "No".to_string()][..])
                    .interact()
                    .unwrap();

                if should_swap == 0 {
                    let discarded_index = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt(
                            format!("Choose card to discard in place of {}", incoming_card_name)
                                .as_str(),
                        )
                        .default(0)
                        .items(&hand_labels[..])
                        .interact()
                        .unwrap();

                    let orderings = hand_orderings.get(&public_state.active_player).unwrap();
                    let discarded_index = orderings[discarded_index];
                    let (next_public_state, next_private_states) = game
                        .step(Action::FinalizeDraw(Some(discarded_index)))
                        .unwrap();

                    public_state = next_public_state;
                    private_states = next_private_states;
                } else {
                    let (next_public_state, next_private_states) =
                        game.step(Action::FinalizeDraw(None)).unwrap();

                    public_state = next_public_state;
                    private_states = next_private_states;
                }
            }

            if option == 2 {
                let incoming_card_name = Card::new_from_index(public_state.pile.top().unwrap())
                    .unwrap()
                    .name();

                let discarded_index = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        format!("Choose card to discard in place of {}", incoming_card_name)
                            .as_str(),
                    )
                    .default(0)
                    .items(&hand_labels[..])
                    .interact()
                    .unwrap();

                let orderings = hand_orderings.get(&public_state.active_player).unwrap();
                let discarded_index = orderings[discarded_index];
                let (next_public_state, next_private_states) =
                    game.step(Action::Swap(discarded_index)).unwrap();

                public_state = next_public_state;
                private_states = next_private_states;
            }

            break;
        }

        if let Some(outcome) = public_state.outcome {
            println!("\n  player {} has won the game\n", outcome.winner);

            for (p, score) in outcome.scores.iter().enumerate() {
                println!("      player {} score: {}", p, score.points);
            }

            break;
        }
    }
}
