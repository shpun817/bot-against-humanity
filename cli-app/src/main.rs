mod assets;
mod input;

use std::{io::Write, thread, time::Duration};

use assets::{answers, questions};
use bot_against_humanity_core::drivers::{generic::GenericDriverBuilder, GameCoreDriver};
use colored::{ColoredString, Colorize};
use input::CountBase;

use crate::input::{InputManager, PreparationInput};

type Error = String;

pub(crate) fn error_handler(err: impl Into<Error>) {
    eprintln!("{}", err.into());
}

struct Config {
    win_target: usize,
    count_base: CountBase,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            win_target: 3,
            count_base: CountBase::OneBased,
        }
    }
}

fn main() {
    let mut builder = GenericDriverBuilder::new();

    builder.add_new_questions(questions());
    builder.add_new_answers(answers());

    let mut config = Config::default();

    loop {
        // Preparation
        println!("Preparation stage!");
        println!("Add a Player: /add `Player Name`");
        println!("Start the game: /start");
        println!("Set hand size: /handsize `Number >= 1`");
        println!("Set win target: /wintarget `Number >= 1`");
        println!("Set count base: /countbase `0 or 1`");

        let mut driver = loop {
            match InputManager::preparation_input() {
                PreparationInput::Start => match builder.build() {
                    Ok(driver) => break driver,
                    Err(err) => error_handler(err),
                },
                PreparationInput::AddPlayer(player_name) => {
                    if let Err(err) = builder.add_player(&player_name) {
                        error_handler(err)
                    } else {
                        println!("{} joins the game!", color_player_name(player_name));
                    }
                }
                PreparationInput::RemovePlayer(player_name) => {
                    if let Err(err) = builder.remove_player(&player_name) {
                        error_handler(err)
                    } else {
                        println!("{} leaves the game!", color_player_name(player_name));
                    }
                }
                PreparationInput::SetHandSize(hand_size) => builder.set_hand_size(hand_size),
                PreparationInput::SetWinTarget(win_target) => config.win_target = win_target,
                PreparationInput::SetCountBaseOne => config.count_base = CountBase::OneBased,
                PreparationInput::SetCountBaseZero => config.count_base = CountBase::ZeroBased,
            }
        };
        let ordered_players = driver.ordered_players();

        loop {
            let round_information = driver.start_round();
            let non_judge_players =
                find_non_judge_players(&ordered_players, &round_information.judge);

            // Display round information to users
            println!(
                "The Judge for this round is {}!",
                color_player_name(&round_information.judge)
            );
            let submitted_answers = non_judge_players.into_iter().fold(vec![], |_, player| {
                println!("======================================================");
                println!("{}, Your hand is:", color_player_name(player));
                for (i, card_content) in round_information
                    .player_hands
                    .get(player)
                    .unwrap()
                    .iter()
                    .enumerate()
                {
                    println!(
                        "{} - {}",
                        if let CountBase::ZeroBased = config.count_base {
                            i
                        } else {
                            i + 1
                        },
                        card_content
                    );
                }
                println!("\nAnswer this: {}", round_information.question);
                println!("Please submit your answers, {}", color_player_name(player));
                let submitted_answers = loop {
                    let answers = InputManager::submit_answers(config.count_base);
                    match driver.submit_answers(player, answers) {
                        Ok(submitted_answers) => {
                            if let Some(submitted_answers) = submitted_answers {
                                break submitted_answers;
                            } else {
                                break vec![];
                            }
                        }
                        Err(err) => error_handler(err),
                    }
                };
                println!("======================================================");

                submitted_answers
            });

            // Display the submitted answers to everyone
            {
                println!("The following creative answers were collected:");
                for (i, (_, answer)) in submitted_answers.iter().enumerate() {
                    println!(
                        "{} - {}",
                        if let CountBase::ZeroBased = config.count_base {
                            i
                        } else {
                            i + 1
                        },
                        answer
                    );

                    for _ in 0..3 {
                        print!(".");
                        std::io::stdout().flush().unwrap();
                        sleep(1);
                    }
                    println!();
                }
                println!(
                    "Choose your favorite, Judge {}!",
                    color_player_name(round_information.judge)
                );
            }

            let ranking = loop {
                let chosen_index = InputManager::choose_favorite(config.count_base);

                let chosen_player = if let Some((player, _)) = submitted_answers.get(chosen_index) {
                    player
                } else {
                    error_handler("Invalid input");
                    continue;
                };

                match driver.end_round(chosen_player) {
                    Ok(round_end_info) => break round_end_info,
                    Err(err) => error_handler(err),
                }
            };
            let highest = ranking[0].clone();

            // Display the ranking
            {
                let mut rank_tracker = 0;
                let mut last_points = highest.1 + 1;
                println!("======================================================");
                for (name, points) in ranking {
                    if points != last_points {
                        rank_tracker += 1
                    }
                    println!(
                        "{} {} - {}",
                        {
                            if rank_tracker == 1 {
                                rank_tracker.to_string().red().bold()
                            } else {
                                rank_tracker.to_string().as_str().into()
                            }
                        },
                        color_player_name(name),
                        points
                    );
                    last_points = points;
                }
                println!("======================================================");
            }

            if highest.1 >= config.win_target as i32 {
                println!(
                    "🎉 Congratulations, {}, You Have Won! 🎉\n",
                    color_player_name(highest.0)
                );
                driver.end_game();
                break;
            }
        }
    }
}

fn color_player_name(text: impl Into<String>) -> ColoredString {
    text.into().green().bold()
}

fn find_non_judge_players<'a>(
    ordered_players: &'a [String],
    judge_name: &String,
) -> Vec<&'a String> {
    ordered_players
        .iter()
        .filter(|&p| p != judge_name)
        .collect()
}

fn sleep(seconds: u64) {
    thread::sleep(Duration::new(seconds, 0));
}
