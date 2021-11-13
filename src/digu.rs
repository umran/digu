use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct Score {
    pub digs: Vec<Vec<u8>>,
    pub bag: Vec<u8>,
    pub points: i32,
    pub winner: bool,
}

pub fn eval_hand(hand: &[u8; 10]) -> Score {
    let working_set: HashSet<u8> = hand.to_vec().iter().copied().collect();

    let mut best_score = Score {
        digs: vec![],
        bag: working_set.iter().copied().collect(),
        points: -working_set.iter().fold(0, |acc, &v| acc + card_points(v)),
        winner: false,
    };

    let ten_c_fours = find_digs(working_set.iter().copied().collect(), 4);
    for (_, i) in ten_c_fours.iter().enumerate() {
        let mut working_set: HashSet<u8> = working_set.iter().copied().collect();
        for (_, card) in i.iter().enumerate() {
            working_set.remove(card);
        }

        let credits = i.iter().fold(0, |acc, &v| acc + card_points(v));

        let six_c_threes = find_digs(working_set.iter().copied().collect(), 3);
        if !six_c_threes.is_empty() {
            for (_, j) in six_c_threes.iter().enumerate() {
                let mut working_set: HashSet<u8> = working_set.iter().copied().collect();
                for (_, card) in j.iter().enumerate() {
                    working_set.remove(card);
                }

                let credits = credits + j.iter().fold(0, |acc, &v| acc + card_points(v));

                // tally the final 3 cards
                let three_c_threes = find_digs(working_set.iter().copied().collect(), 3);
                if !three_c_threes.is_empty() {
                    for (_, k) in three_c_threes.iter().enumerate() {
                        let mut working_set: HashSet<u8> = working_set.iter().copied().collect();
                        for (_, card) in k.iter().enumerate() {
                            working_set.remove(card);
                        }

                        let credits = credits + k.iter().fold(0, |acc, &v| acc + card_points(v));
                        // tally total
                        let debits: i32 =
                            working_set.iter().fold(0, |acc, &v| acc + card_points(v));
                        let points = credits - debits + 100; // plus 100 for completion
                        if points > best_score.points {
                            best_score = Score {
                                digs: vec![i.to_vec(), j.to_vec(), k.to_vec()],
                                bag: working_set.iter().copied().collect(),
                                points: points,
                                winner: true,
                            };
                        }
                    }
                } else {
                    // tally total
                    let debits: i32 = working_set.iter().fold(0, |acc, &v| acc + card_points(v));
                    let points = credits - debits;
                    if points > best_score.points {
                        best_score = Score {
                            digs: vec![i.to_vec(), j.to_vec()],
                            bag: working_set.iter().copied().collect(),
                            points: points,
                            winner: false,
                        };
                    }
                }
            }
        } else {
            // tally total
            let debits: i32 = working_set.iter().fold(0, |acc, &v| acc + card_points(v));
            let points = credits - debits;
            if points > best_score.points {
                best_score = Score {
                    digs: vec![i.to_vec()],
                    bag: working_set.iter().copied().collect(),
                    points: points,
                    winner: false,
                };
            }
        }
    }

    let ten_c_threes = find_digs(working_set.iter().copied().collect(), 3);
    for (_, i) in ten_c_threes.iter().enumerate() {
        let mut working_set: HashSet<u8> = working_set.iter().copied().collect();
        for (_, card) in i.iter().enumerate() {
            working_set.remove(card);
        }

        let credits = i.iter().fold(0, |acc, &v| acc + card_points(v));

        let seven_c_threes = find_digs(working_set.iter().copied().collect(), 3);
        if !seven_c_threes.is_empty() {
            for (_, j) in seven_c_threes.iter().enumerate() {
                let mut working_set: HashSet<u8> = working_set.iter().copied().collect();
                for (_, card) in j.iter().enumerate() {
                    working_set.remove(card);
                }

                let credits = credits + j.iter().fold(0, |acc, &v| acc + card_points(v));

                let four_c_threes = find_digs(working_set.iter().copied().collect(), 3);
                if !four_c_threes.is_empty() {
                    for (_, k) in four_c_threes.iter().enumerate() {
                        let mut working_set: HashSet<u8> = working_set.iter().copied().collect();
                        for (_, card) in k.iter().enumerate() {
                            working_set.remove(card);
                        }

                        let credits = credits + k.iter().fold(0, |acc, &v| acc + card_points(v));

                        // tally total
                        let debits: i32 =
                            working_set.iter().fold(0, |acc, &v| acc + card_points(v));
                        let points = credits - debits;
                        if points > best_score.points {
                            best_score = Score {
                                digs: vec![i.to_vec(), j.to_vec(), k.to_vec()],
                                bag: working_set.iter().copied().collect(),
                                points: points,
                                winner: false,
                            };
                        }
                    }
                } else {
                    // tally total
                    let debits: i32 = working_set.iter().fold(0, |acc, &v| acc + card_points(v));
                    let points = credits - debits;
                    if points > best_score.points {
                        best_score = Score {
                            digs: vec![i.to_vec(), j.to_vec()],
                            bag: working_set.iter().copied().collect(),
                            points: points,
                            winner: false,
                        };
                    }
                }
            }
        } else {
            // tally total
            let debits: i32 = working_set.iter().fold(0, |acc, &v| acc + card_points(v));
            let points = credits - debits;
            if points > best_score.points {
                best_score = Score {
                    digs: vec![i.to_vec()],
                    bag: working_set.iter().copied().collect(),
                    points: points,
                    winner: false,
                };
            }
        }
    }

    best_score
}

fn find_digs(set: Vec<u8>, dig_length: usize) -> Vec<Vec<u8>> {
    let mut digs = vec![];
    let combos = set.into_iter().combinations(dig_length);
    for (_, combo) in combos.enumerate() {
        if is_dig(combo.clone()) {
            digs.push(combo);
        }
    }

    digs
}

fn is_dig(combo: Vec<u8>) -> bool {
    is_par(combo.clone()) || is_seq(combo.clone())
}

fn is_par(combo: Vec<u8>) -> bool {
    for (i, val) in combo.iter().enumerate() {
        if i > 0 {
            let a = val % 13;
            let b = combo[i - 1] % 13;
            if a != b {
                return false;
            }
        }
    }

    true
}

fn is_seq(mut combo: Vec<u8>) -> bool {
    combo.sort();

    for (i, val) in combo.iter().enumerate() {
        if i > 0 {
            let b = val % 13;
            let a = combo[i - 1] % 13;

            if a > b || b - a != 1 {
                return false;
            }
        }
    }

    if combo[combo.len() - 1] > 11 {
        return false;
    }

    true
}

pub fn card_points(card: u8) -> i32 {
    let value_index = card % 13;

    match value_index {
        0..=8 => (value_index + 2).into(),
        9..=11 => 10,
        12 => 15,
        _ => 0,
    }
}
