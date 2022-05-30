use std::fmt::Display;

use regex::Regex;

use crate::errors::GameCoreError;

use super::{answer_card::AnswerCard, card::Card};

#[derive(Clone, Debug)]
pub(crate) struct QuestionCard {
    tokens: Vec<QuestionToken>,
}

pub(crate) type QuestionToken = Option<String>;

impl Card for QuestionCard {}

impl Display for QuestionCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.tokens
                .iter()
                .map(|token| {
                    let token_str = if let Some(token_str) = token {
                        token_str
                    } else {
                        "_"
                    };
                    token_str.to_owned()
                })
                .collect::<String>()
        )
    }
}

impl From<String> for QuestionCard {
    fn from(question: String) -> Self {
        Self::new(question)
    }
}

impl QuestionCard {
    pub(crate) fn new(question: impl Into<String>) -> Self {
        let underscores_regex = Regex::new("_+").unwrap();
        let question = question.into();

        if underscores_regex.is_match(&question) {
            let tokens = underscores_regex
                .split(&question)
                .flat_map(|s| [Some(s.to_owned()), None])
                .collect::<Vec<_>>();

            let num_tokens_with_extra_none = tokens.len();

            Self {
                tokens: tokens
                    .into_iter()
                    .take(num_tokens_with_extra_none - 1)
                    .collect(),
            }
        } else {
            Self {
                tokens: vec![Some(question + " "), None],
            }
        }
    }

    pub(crate) fn num_blanks(&self) -> usize {
        self.tokens.iter().filter(|token| token.is_none()).count()
    }

    pub(crate) fn combine_with_answer_cards(
        &self,
        answer_cards: &[AnswerCard],
    ) -> Result<String, GameCoreError> {
        if self.num_blanks() != answer_cards.len() {
            return Err(GameCoreError::QuestionBlanksAndNumAnswersMismatch {
                num_blanks: self.num_blanks(),
                num_answers: answer_cards.len(),
            });
        }

        let mut current_answer = 0;

        Ok(self
            .tokens
            .iter()
            .map(|token| {
                let token_str = if let Some(token_str) = token {
                    token_str
                } else {
                    let answer_str = &answer_cards[current_answer].content;
                    current_answer += 1;
                    answer_str
                };
                token_str.to_owned()
            })
            .collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::card_storage::CardStorage;

    use super::*;

    #[test]
    fn new_str() {
        let question_card = QuestionCard::new("How are you?");

        assert_eq!(question_card.tokens.len(), 2);

        assert_eq!(question_card.tokens[0].as_ref().unwrap(), "How are you? ");
        assert!(question_card.tokens[1].is_none());

        assert_eq!(question_card.num_blanks(), 1);
    }

    #[test]
    fn new_string() {
        let question_card = QuestionCard::new("How are you?".to_owned());

        assert_eq!(question_card.tokens.len(), 2);

        assert_eq!(question_card.tokens[0].as_ref().unwrap(), "How are you? ");
        assert!(question_card.tokens[1].is_none());

        assert_eq!(question_card.num_blanks(), 1);
    }

    #[test]
    fn new_1_blank_size_1() {
        let question_card = QuestionCard::new("How are _?".to_owned());

        assert_eq!(question_card.tokens.len(), 3);

        assert_eq!(question_card.tokens[0].as_ref().unwrap(), "How are ");

        assert!(question_card.tokens[1].is_none());

        assert_eq!(question_card.tokens[2].as_ref().unwrap(), "?");

        assert_eq!(question_card.num_blanks(), 1);
    }

    #[test]
    fn new_1_blank_size_2() {
        let question_card = QuestionCard::new("How are __?".to_owned());

        assert_eq!(question_card.tokens.len(), 3);

        assert_eq!(question_card.tokens[0].as_ref().unwrap(), "How are ");

        assert!(question_card.tokens[1].is_none());

        assert_eq!(question_card.tokens[2].as_ref().unwrap(), "?");

        assert_eq!(question_card.num_blanks(), 1);
    }

    #[test]
    fn new_1_blank_size_3() {
        let question_card = QuestionCard::new("How are ___?".to_owned());

        assert_eq!(question_card.tokens.len(), 3);

        assert_eq!(question_card.tokens[0].as_ref().unwrap(), "How are ");

        assert!(question_card.tokens[1].is_none());

        assert_eq!(question_card.tokens[2].as_ref().unwrap(), "?");

        assert_eq!(question_card.num_blanks(), 1);
    }

    #[test]
    fn new_2_blanks() {
        let question_card = QuestionCard::new("How _ are _?".to_owned());

        assert_eq!(question_card.tokens.len(), 5);

        assert_eq!(question_card.tokens[0].as_ref().unwrap(), "How ");

        assert!(question_card.tokens[1].is_none());

        assert_eq!(question_card.tokens[2].as_ref().unwrap(), " are ");

        assert!(question_card.tokens[3].is_none());

        assert_eq!(question_card.tokens[4].as_ref().unwrap(), "?");

        assert_eq!(question_card.num_blanks(), 2);
    }

    #[test]
    fn display_question() {
        let question_card = QuestionCard::new("Who are you?");
        assert_eq!(question_card.to_string(), "Who are you? _");
    }

    #[test]
    fn display_question_with_blanks() {
        let question_card = QuestionCard::new("Who are _, _?");
        assert_eq!(question_card.to_string(), "Who are _, _?");
    }

    #[test]
    fn combine_with_answer_cards_question() {
        let question_card = QuestionCard::new("Who am I?");
        let answer_cards = vec![AnswerCard::new("Your Father")];

        let combine_result = question_card.combine_with_answer_cards(&answer_cards);
        assert_eq!(combine_result.ok().unwrap(), "Who am I? Your Father");
    }

    #[test]
    fn combine_with_answer_cards_fill_single_blank() {
        let question_card = QuestionCard::new("I am _.");
        let answer_cards = vec![AnswerCard::new("Your Father")];

        let combine_result = question_card.combine_with_answer_cards(&answer_cards);
        assert_eq!(combine_result.ok().unwrap(), "I am Your Father.");
    }

    #[test]
    fn combine_with_answer_cards_fill_multiple_blanks() {
        let question_card = QuestionCard::new("I am _, and you are _.");
        let answer_cards = vec![
            AnswerCard::new("Your Father"),
            AnswerCard::new("Luke Skywalker"),
        ];

        let combine_result = question_card.combine_with_answer_cards(&answer_cards);
        assert_eq!(
            combine_result.ok().unwrap(),
            "I am Your Father, and you are Luke Skywalker."
        );
    }

    #[test]
    fn combine_with_wrong_number_of_answer_cards() {
        let question_card = QuestionCard::new("I am _, and you are _.");
        let answer_cards = vec![
            AnswerCard::new("Your Father"),
            AnswerCard::new("My Mother"),
            AnswerCard::new("Luke Skywalker"),
        ];

        let combine_result = question_card.combine_with_answer_cards(&answer_cards);
        assert_eq!(
            combine_result.err().unwrap(),
            GameCoreError::QuestionBlanksAndNumAnswersMismatch {
                num_blanks: 2,
                num_answers: 3
            }
        );
    }

    #[test]
    fn card_storage() {
        let mut card_storage = CardStorage::new();
        let question_card = QuestionCard::new("Who am I?");
        card_storage.add_card_to_deck(question_card);
    }
}
