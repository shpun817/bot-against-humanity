use std::collections::HashMap;

use crate::{player::Player, cards::{CardStorage, QuestionCard, AnswerCard}};

use self::builder::PlayerName;

mod builder;

pub(crate) struct GameState<PN = String>
where
    PN: PlayerName,
{
    pub(crate) players: HashMap<PN, Player>,
    pub(crate) question_card_storage: CardStorage<QuestionCard>,
    pub(crate) answer_card_storage: CardStorage<AnswerCard>,
}
