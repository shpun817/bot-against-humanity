use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum GameCoreError {
    QuestionBlanksAndNumAnswersMismatch,
    PlayerChoosingPlayCardOutOfHandBounds,
    PlayerChoosingTheSameCardMultipleTimes,
}

impl Display for GameCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            GameCoreError::QuestionBlanksAndNumAnswersMismatch => {
                "Mismatch in number of question blanks and number of answers."
            }
            GameCoreError::PlayerChoosingPlayCardOutOfHandBounds => {
                "Player chose a card index out of hand bounds."
            }
            GameCoreError::PlayerChoosingTheSameCardMultipleTimes => {
                "Player chose the same card index multiple times."
            }
        };

        write!(f, "GameCoreError: {}", msg)
    }
}
