use super::card::Card;
use std::collections::{HashMap, HashSet};

struct Outcome {
    digs: Vec<Vec<u8>>,
    bag: Vec<u8>,
    score: i32,
}

fn score_hand(hand: [u8; 10]) -> i32 {
    let mut working_set: HashSet<u8> = hand.to_vec().iter().copied().collect();

    let mut best_outcome = Outcome {
        digs: vec![],
        bag: working_set.iter().copied().collect(),
        score: working_set.iter().fold(0, |acc, &v| acc + card_score(v)),
    };

    let ten_c_fours = choose_fours(working_set.iter().copied().collect());
    for (_, i) in ten_c_fours.iter().enumerate() {
        let mut working_set: HashSet<u8> = working_set.iter().copied().collect();
        working_set.remove(&i[0]);
        working_set.remove(&i[1]);
        working_set.remove(&i[2]);
        working_set.remove(&i[3]);

        let credits = i.to_vec().iter().fold(0, |acc, &v| acc + card_score(v));

        let six_c_threes = choose_threes(working_set.iter().copied().collect());
        if !six_c_threes.is_empty() {
            for (_, j) in six_c_threes.iter().enumerate() {
                let mut working_set: HashSet<u8> = working_set.iter().copied().collect();
                working_set.remove(&j[0]);
                working_set.remove(&j[1]);
                working_set.remove(&j[2]);

                let credits = credits + j.to_vec().iter().fold(0, |acc, &v| acc + card_score(v));

                // tally total
                let debits: i32 = working_set.iter().fold(0, |acc, &v| acc + card_score(v));
                let score = credits - debits;
                if score > best_outcome.score {
                    best_outcome = Outcome {
                        digs: vec![i.to_vec(), j.to_vec()],
                        bag: working_set.iter().copied().collect(),
                        score: score,
                    };
                }
            }
        } else {
            // tally total
            let debits: i32 = working_set.iter().fold(0, |acc, &v| acc + card_score(v));
            let score = credits - debits;
            if score > best_outcome.score {
                best_outcome = Outcome {
                    digs: vec![i.to_vec()],
                    bag: working_set.iter().copied().collect(),
                    score: score,
                };
            }
        }
    }

    let ten_c_threes = choose_threes(working_set.iter().copied().collect());
    for (_, i) in ten_c_threes.iter().enumerate() {
        let mut working_set: HashSet<u8> = working_set.iter().copied().collect();
        working_set.remove(&i[0]);
        working_set.remove(&i[1]);
        working_set.remove(&i[2]);

        let credits = i.to_vec().iter().fold(0, |acc, &v| acc + card_score(v));

        let seven_c_threes = choose_threes(working_set.iter().copied().collect());
        if !seven_c_threes.is_empty() {
            for (_, j) in seven_c_threes.iter().enumerate() {
                let mut working_set: HashSet<u8> = working_set.iter().copied().collect();
                working_set.remove(&j[0]);
                working_set.remove(&j[1]);
                working_set.remove(&j[2]);

                let credits = credits + j.to_vec().iter().fold(0, |acc, &v| acc + card_score(v));

                let four_c_threes = choose_threes(working_set.iter().copied().collect());
                if !four_c_threes.is_empty() {
                    for (_, k) in four_c_threes.iter().enumerate() {
                        let mut working_set: HashSet<u8> = working_set.iter().copied().collect();
                        working_set.remove(&k[0]);
                        working_set.remove(&k[1]);
                        working_set.remove(&k[2]);

                        let credits =
                            credits + k.to_vec().iter().fold(0, |acc, &v| acc + card_score(v));

                        // tally total
                        let debits: i32 = working_set.iter().fold(0, |acc, &v| acc + card_score(v));
                        let score = credits - debits;
                        if score > best_outcome.score {
                            best_outcome = Outcome {
                                digs: vec![i.to_vec(), j.to_vec(), k.to_vec()],
                                bag: working_set.iter().copied().collect(),
                                score: score,
                            };
                        }
                    }
                } else {
                    // tally total
                    let debits: i32 = working_set.iter().fold(0, |acc, &v| acc + card_score(v));
                    let score = credits - debits;
                    if score > best_outcome.score {
                        best_outcome = Outcome {
                            digs: vec![i.to_vec(), j.to_vec()],
                            bag: working_set.iter().copied().collect(),
                            score: score,
                        };
                    }
                }
            }
        } else {
            // tally total
            let debits: i32 = working_set.iter().fold(0, |acc, &v| acc + card_score(v));
            let score = credits - debits;
            if score > best_outcome.score {
                best_outcome = Outcome {
                    digs: vec![i.to_vec()],
                    bag: working_set.iter().copied().collect(),
                    score: score,
                };
            }
        }
    }

    0
}

fn choose_threes(set: Vec<u8>) -> HashSet<[u8; 3]> {
    let mut chosen = HashSet::new();

    for (i, ci) in set.iter().enumerate() {
        for (j, cj) in set.iter().enumerate() {
            for (k, ck) in set.iter().enumerate() {
                if i == j || i == k || j == k {
                    continue;
                }

                if is_three_dig(*ci, *cj, *ck) {
                    chosen.insert([*ci, *cj, *ck]);
                }
            }
        }
    }

    chosen
}

fn choose_fours(set: Vec<u8>) -> HashSet<[u8; 4]> {
    let mut chosen = HashSet::new();

    for (i, ci) in set.iter().enumerate() {
        for (j, cj) in set.iter().enumerate() {
            for (k, ck) in set.iter().enumerate() {
                for (l, cl) in set.iter().enumerate() {
                    if i == j || i == k || i == l || j == k || j == l || k == l {
                        continue;
                    }

                    if is_four_dig(*ci, *cj, *ck, *cl) {
                        chosen.insert([*ci, *cj, *ck, *cl]);
                    }
                }
            }
        }
    }

    chosen
}

fn is_three_dig(a: u8, b: u8, c: u8) -> bool {
    is_par_three(a, b, c) || is_seq_three(a, b, c)
}

fn is_four_dig(a: u8, b: u8, c: u8, d: u8) -> bool {
    is_par_four(a, b, c, d) || is_seq_four(a, b, c, d)
}

fn is_par_three(a: u8, b: u8, c: u8) -> bool {
    let a = a % 13;
    let b = b % 13;
    let c = c % 13;

    a == b && a == c
}

fn is_par_four(a: u8, b: u8, c: u8, d: u8) -> bool {
    let a = a % 13;
    let b = b % 13;
    let c = c % 13;
    let d = d % 13;

    a == b && a == c && a == d
}

fn is_seq_three(a: u8, b: u8, c: u8) -> bool {
    let mut sorted = vec![a, b, c];
    sorted.sort();

    let a = sorted[0] % 13;
    let b = sorted[1] % 13;
    let c = sorted[2] % 13;

    if (a - b).pow(2) != 1 {
        return false;
    }
    if (b - c).pow(2) != 1 {
        return false;
    }

    if c > 11 {
        return false;
    }

    true
}

fn is_seq_four(a: u8, b: u8, c: u8, d: u8) -> bool {
    let mut sorted = vec![a, b, c, d];
    sorted.sort();

    let a = sorted[0] % 13;
    let b = sorted[1] % 13;
    let c = sorted[2] % 13;
    let d = sorted[3] % 13;

    if (a - b).pow(2) != 1 {
        return false;
    }
    if (b - c).pow(2) != 1 {
        return false;
    }
    if (c - d).pow(2) != 1 {
        return false;
    }

    if d > 11 {
        return false;
    }

    true
}

pub fn card_score(card: u8) -> i32 {
    let value_index = card % 13;

    match value_index {
        0..=8 => (value_index + 2).into(),
        9..=11 => 10,
        12 => 15,
        _ => 0,
    }
}
