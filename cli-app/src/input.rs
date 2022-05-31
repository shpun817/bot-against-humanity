use std::str::FromStr;

use crate::error_handler;

pub(crate) struct InputManager;

pub(crate) enum PreparationInput {
    Start,
    AddPlayer(String),
    SetHandSize(usize),  // Must be >= 1
    SetWinTarget(usize), // Must be >= 1
    SetCountBaseOne,
    SetCountBaseZero,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum CountBase {
    ZeroBased,
    OneBased,
}

type Error = String;

impl InputManager {
    /// Valid input:
    /// - /start
    /// - /add `Player Name`
    /// - /handsize `usize >= 1`
    /// - /wintarget `usize >= 1`
    /// - /countbase `0 or 1`
    pub fn preparation_input() -> PreparationInput {
        let tokens = Self::read_line_as_tokens_until_no_error(|tokens: &Vec<String>| {
            tokens[0] == "/start"
                || (tokens[0] == "/add" && tokens.len() >= 2)
                || ((tokens[0] == "/handsize" || tokens[0] == "/wintarget")
                    && tokens.len() == 2
                    && if let Ok(handsize) = tokens[1].parse::<usize>() {
                        handsize >= 1
                    } else {
                        false
                    })
                || (tokens[0] == "/countbase"
                    && tokens.len() == 2
                    && (tokens[1] == "0" || tokens[1] == "1"))
        });

        if tokens.len() == 1 {
            PreparationInput::Start
        } else {
            match tokens[0].as_str() {
                "/add" => PreparationInput::AddPlayer(tokens[1..].join(" ")),
                "/handsize" => PreparationInput::SetHandSize(tokens[1].parse::<usize>().unwrap()),
                "/wintarget" => PreparationInput::SetWinTarget(tokens[1].parse::<usize>().unwrap()),
                "/countbase" => match tokens[1].as_str() {
                    "0" => PreparationInput::SetCountBaseZero,
                    "1" => PreparationInput::SetCountBaseOne,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
    }

    /// Return a vector of indices representing the submitted answers.
    /// `count_base` is used to validate the indices; the offset is applied before returning if necessary.
    pub fn submit_answers(count_base: CountBase) -> Vec<usize> {
        let indices = Self::read_line_as_tokens_until_no_error(|indices: &Vec<usize>| {
            if let CountBase::OneBased = count_base {
                !indices.iter().any(|&ind| ind == 0)
            } else {
                true
            }
        });

        if let CountBase::OneBased = count_base {
            indices.into_iter()
                .map(|ind| ind - 1)
                .collect()
        } else {
            indices
        }
    }

    /// Return an index representing the chosen player.
    /// `count_base` is used to validate the indices; the offset is applied before returning if necessary.
    pub fn choose_favorite(count_base: CountBase) -> usize {
        let index = Self::read_line_as_tokens_until_no_error(|indices: &Vec<usize>| {
            indices.len() == 1 && {
                if let CountBase::OneBased = count_base {
                    indices[0] != 0
                } else {
                    true
                }
            }
        })[0];

        if let CountBase::OneBased = count_base {
            index - 1
        } else {
            index
        }
    }

    // The input must also pass the predicate in order to be returned.
    fn read_line_as_tokens_until_no_error<T>(predicate: impl Fn(&Vec<T>) -> bool) -> Vec<T>
    where
        T: FromStr,
    {
        loop {
            match Self::read_line_as_tokens::<T>() {
                Ok(tokens) => {
                    if predicate(&tokens) {
                        return tokens;
                    }
                    error_handler("Invalid input");
                }
                Err(err) => {
                    error_handler(err);
                }
            }
        }
    }

    fn read_line_as_tokens<T>() -> Result<Vec<T>, Error>
    where
        T: FromStr,
    {
        let line = Self::read_line()?;

        let result = line
            .split_whitespace()
            .into_iter()
            .map(|t| t.parse::<T>())
            .collect::<Result<Vec<T>, _>>();

        if let Ok(tokens) = result {
            if tokens.is_empty() {
                return Err("Empty input".to_owned());
            }

            Ok(tokens)
        } else {
            Err("Invalid input".to_owned())
        }
    }

    fn read_line() -> Result<String, Error> {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;

        if input.is_empty() {
            Err("Empty input".to_owned())
        } else {
            Ok(input)
        }
    }
}
