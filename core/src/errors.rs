use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum GameCoreError {
    QuestionBlanksAndNumAnswersMismatch,
}

impl Display for GameCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            GameCoreError::QuestionBlanksAndNumAnswersMismatch => {
                "Mismatch in number of question blanks and number of answers."
            }
        };

        write!(f, "GameCoreError: {}", msg)
    }
}
