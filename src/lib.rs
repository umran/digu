mod card;
mod digu;
pub mod game;
mod stack;

#[cfg(test)]
mod tests {
    use super::digu::eval_hand;

    #[test]
    fn evaluate_preset_hand() {
        let hand = [0, 1, 2, 3, 4, 5, 6, 7, 9, 10];
        let score = eval_hand(&hand);
        assert_eq!(score.digs.len(), 2);
        assert_eq!(score.points, 20);
    }
}
