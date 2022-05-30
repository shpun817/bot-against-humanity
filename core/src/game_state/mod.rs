use std::collections::HashMap;

use crate::{player::Player, cards::{CardStorage, QuestionCard, AnswerCard}};

use self::builder::PlayerName;

pub mod builder;

pub struct GameState<PN = String>
where
    PN: PlayerName,
{
    players: HashMap<PN, Player>,
    question_card_storage: CardStorage<QuestionCard>,
    answer_card_storage: CardStorage<AnswerCard>,
}
