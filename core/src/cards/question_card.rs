use std::fmt::Display;

use regex::Regex;

pub(crate) struct QuestionCard {
    tokens: Vec<QuestionToken>,
}

pub(crate) type QuestionToken = Option<String>;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_str() {
        let question_card = QuestionCard::new("How are you?");

        assert_eq!(question_card.tokens.len(), 2);

        assert_eq!(question_card.tokens[0].as_ref().unwrap(), "How are you? ");
        assert!(question_card.tokens[1].is_none());
    }

    #[test]
    fn new_string() {
        let question_card = QuestionCard::new("How are you?".to_owned());

        assert_eq!(question_card.tokens.len(), 2);

        assert_eq!(question_card.tokens[0].as_ref().unwrap(), "How are you? ");
        assert!(question_card.tokens[1].is_none());
    }

    #[test]
    fn new_1_blank_size_1() {
        let question_card = QuestionCard::new("How are _?".to_owned());

        assert_eq!(question_card.tokens.len(), 3);

        assert_eq!(question_card.tokens[0].as_ref().unwrap(), "How are ");

        assert!(question_card.tokens[1].is_none());

        assert_eq!(question_card.tokens[2].as_ref().unwrap(), "?");
    }

    #[test]
    fn new_1_blank_size_2() {
        let question_card = QuestionCard::new("How are __?".to_owned());

        assert_eq!(question_card.tokens.len(), 3);

        assert_eq!(question_card.tokens[0].as_ref().unwrap(), "How are ");

        assert!(question_card.tokens[1].is_none());

        assert_eq!(question_card.tokens[2].as_ref().unwrap(), "?");
    }

    #[test]
    fn new_1_blank_size_3() {
        let question_card = QuestionCard::new("How are ___?".to_owned());

        assert_eq!(question_card.tokens.len(), 3);

        assert_eq!(question_card.tokens[0].as_ref().unwrap(), "How are ");

        assert!(question_card.tokens[1].is_none());

        assert_eq!(question_card.tokens[2].as_ref().unwrap(), "?");
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
}
