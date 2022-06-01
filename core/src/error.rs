use std::fmt::Display;

use GameCoreError::*;

#[derive(Debug, PartialEq, Eq)]
pub enum GameCoreError {
    Custom(String),
    QuestionBlanksAndNumAnswersMismatch {
        num_blanks: usize,
        num_answers: usize,
    },
    PlayerChoosingCardOutOfHandBound {
        chosen_ind: usize,
        hand_bound: usize,
    },
    PlayerChoosingTheSameCardMultipleTimes {
        chosen_ind: usize,
    },
    JudgeTryingToSubmitAnswers {
        judge_name: String,
    },
    PlayerAlreadySubmittedAnswers {
        player_name: String,
    },
    InsufficientAnswerCardsToDeal {
        num_players: usize,
        each_deal: usize,
        num_answer_cards: usize,
    },
    NoQuestionCards,
    NoActiveQuestionCard,
    NotEnoughPlayers {
        num_players: usize,
    },
    PlayerAlreadyExists {
        name: String,
    },
    PlayerDoesNotExist {
        name: String,
    },
    GameNotStarted,
    GameAlreadyInProgress,
}

impl Display for GameCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Custom(msg) => {
                format!("(custom) {}", msg)
            }
            QuestionBlanksAndNumAnswersMismatch {
                num_blanks,
                num_answers,
            } => {
                format!(
                    "Mismatch in number of question blanks ({}) and number of answers ({}).",
                    num_blanks, num_answers
                )
            }
            PlayerChoosingCardOutOfHandBound {
                chosen_ind,
                hand_bound,
            } => {
                format!(
                    "Player chose a card index ({}) >= hand size ({}).",
                    chosen_ind, hand_bound
                )
            }
            PlayerChoosingTheSameCardMultipleTimes { chosen_ind } => {
                format!(
                    "Player chose the same card index ({}) multiple times.",
                    chosen_ind
                )
            }
            PlayerAlreadyExists { name } => {
                format!("A Player with the name {} already exists.", name)
            }
            InsufficientAnswerCardsToDeal {
                num_players,
                each_deal,
                num_answer_cards,
            } => {
                format!(
                    "Cannot deal {} cards to {} players when there are only {} cards in total.",
                    each_deal, num_players, num_answer_cards
                )
            }
            NotEnoughPlayers { num_players } => {
                format!("There must be at least 3 players. (Now: {})", num_players)
            }
            NoQuestionCards => "There are no question cards.".to_owned(),
            NoActiveQuestionCard => "There is no active question card.".to_owned(),
            PlayerDoesNotExist { name } => {
                format!("Player with name {} does not exist.", name)
            }
            JudgeTryingToSubmitAnswers { judge_name } => {
                format!("The Judge ({}) cannot submit answers.", judge_name)
            }
            PlayerAlreadySubmittedAnswers { player_name } => {
                format!("Player {} already submitted answers.", player_name)
            }
            GameNotStarted => "The game is not started.".to_owned(),
            GameAlreadyInProgress => "The game is already in progress.".to_owned(),
        };

        write!(f, "GameCoreError: {}", msg)
    }
}

impl From<GameCoreError> for String {
    fn from(val: GameCoreError) -> Self {
        val.to_string()
    }
}

impl From<String> for GameCoreError {
    fn from(msg: String) -> Self {
        Self::Custom(msg)
    }
}

impl From<&str> for GameCoreError {
    fn from(msg: &str) -> Self {
        Self::Custom(msg.to_owned())
    }
}
