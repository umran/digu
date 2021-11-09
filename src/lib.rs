mod card;
mod digu;
pub mod game;
mod stack;

#[cfg(test)]
mod tests {
    use super::card::{Card, Value};
    use super::digu::score_hand;

    #[test]
    fn check_first_card() {
        let two_diamonds = Card::Diamonds(Value::Two);
        let index = two_diamonds.index();
        assert_eq!(index, 0);

        let card: Card = 0u8.try_into().unwrap();
        assert_eq!(index, card.index());
    }

    #[test]
    fn check_last_card() {
        let ace_spades = Card::Spades(Value::A);
        let index = ace_spades.index();
        assert_eq!(index, 51);

        let card: Card = 51u8.try_into().unwrap();
        assert_eq!(index, card.index());
    }

    #[test]
    fn check_all_indices() {
        for i in 0u8..52u8 {
            let card: Card = i.try_into().unwrap();
            assert_eq!(card.index(), i);
        }
    }

    #[test]
    fn evaluate_preset_hand() {
        let hand = [0, 1, 2, 3, 4, 5, 6, 7, 9, 10];
        let outcome = score_hand(&hand);
        assert_eq!(outcome.digs.len(), 2);
        assert_eq!(outcome.score, 20);
    }
}
