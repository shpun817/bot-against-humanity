use super::card::Card;

#[derive(Clone, Debug)]
pub(crate) struct AnswerCard {
    pub(crate) content: String,
}

impl Card for AnswerCard {}

impl From<String> for AnswerCard {
    fn from(answer: String) -> Self {
        Self::new(answer)
    }
}

impl AnswerCard {
    pub(crate) fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::cards::card_storage::CardStorage;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn new_str() {
        let answer_card = AnswerCard::new("Hello");

        assert_eq!(answer_card.content, "Hello");
    }

    #[test]
    fn new_string() {
        let answer_card = AnswerCard::new("Hello".to_owned());

        assert_eq!(answer_card.content, "Hello");
    }

    #[test]
    fn new_two_words() {
        let answer_card = AnswerCard::new("Hello World");

        assert_eq!(answer_card.content, "Hello World");
    }

    #[test]
    fn card_storage() {
        let mut card_storage = CardStorage::new();
        let answer_card = AnswerCard::new("None of your business");
        card_storage.add_card_to_deck(answer_card);
    }
}
