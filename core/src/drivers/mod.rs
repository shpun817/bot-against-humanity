use crate::game_state::{self, AllSubmittedAnswers};

pub mod generic;
pub mod wasm;

pub trait GameCoreDriver {
    type PlayerName: game_state::PlayerName;
    type Error;
    type RoundStartInfo;
    type RoundEndInfo;

    fn ordered_players(&self) -> Vec<Self::PlayerName>;

    fn start_round(&mut self) -> Self::RoundStartInfo;

    fn submit_answers(
        &mut self,
        player_name: impl Into<Self::PlayerName>,
        answer_indices: impl IntoIterator<Item = impl Into<usize>>,
    ) -> Result<AllSubmittedAnswers<Self::PlayerName>, Self::Error>;

    fn redraw_hands(
        &mut self,
        player_names: impl IntoIterator<Item = impl Into<Self::PlayerName>>,
    ) -> Result<(), Self::Error>;

    fn end_round(
        &mut self,
        chosen_player: impl Into<Self::PlayerName>,
    ) -> Result<Self::RoundEndInfo, Self::Error>;

    fn end_game(self);
}
