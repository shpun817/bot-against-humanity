use crate::game_state::{self, AllSubmittedAnswers};

pub mod generic;

pub trait GameCoreDriver {
    type PlayerName: game_state::PlayerName;
    type Error;
    type RoundStartInfo;
    type RoundEndInfo;

    fn start_game(&mut self) -> Result<(), Self::Error>;
    fn start_round(&mut self) -> Result<Self::RoundStartInfo, Self::Error>;
    fn submit_answers(
        &mut self,
        player_name: Self::PlayerName,
        answer_indices: impl IntoIterator<Item = impl Into<usize>>,
    ) -> Result<AllSubmittedAnswers<Self::PlayerName>, Self::Error>;
    fn end_round(
        &mut self,
        chosen_player: Self::PlayerName,
    ) -> Result<Self::RoundEndInfo, Self::Error>;
    fn end_game(&mut self) -> Result<(), Self::Error>;
}
