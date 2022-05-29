use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum GameCoreError {
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
    PlayerWithThisNameAlreadyExists {
        name: String,
    },
    InsufficientAnswerCardsToDeal {
        num_players: usize,
        each_deal: usize,
        num_answer_cards: usize,
    },
    NotEnoughPlayers {
        num_players: usize,
    },
    NoQuestionCards,
}

impl Display for GameCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            GameCoreError::QuestionBlanksAndNumAnswersMismatch {
                num_blanks,
                num_answers,
            } => {
                format!(
                    "Mismatch in number of question blanks ({}) and number of answers ({}).",
                    num_blanks, num_answers
                )
            }
            GameCoreError::PlayerChoosingCardOutOfHandBound {
                chosen_ind,
                hand_bound,
            } => {
                format!(
                    "Player chose a card index ({}) >= hand size ({}).",
                    chosen_ind, hand_bound
                )
            }
            GameCoreError::PlayerChoosingTheSameCardMultipleTimes { chosen_ind } => {
                format!(
                    "Player chose the same card index ({}) multiple times.",
                    chosen_ind
                )
            }
            GameCoreError::PlayerWithThisNameAlreadyExists { name } => {
                format!("A Player with the name {} already exists.", name)
            }
            GameCoreError::InsufficientAnswerCardsToDeal {
                num_players,
                each_deal,
                num_answer_cards,
            } => {
                format!(
                    "Cannot deal {} cards to {} players when there are only {} cards in total.",
                    each_deal, num_players, num_answer_cards
                )
            }
            GameCoreError::NotEnoughPlayers { num_players } => {
                format!("There must be at least 3 players. (Now: {})", num_players)
            }
            GameCoreError::NoQuestionCards => "There are no question cards.".to_owned(),
        };

        write!(f, "GameCoreError: {}", msg)
    }
}
