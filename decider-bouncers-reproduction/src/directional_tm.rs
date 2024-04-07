#[derive(Debug)]
enum TMError {
    MachineHasHalted,
    OutOfTapeError,
    InvalidConfigurationError,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TMTransition {
    pub write: u8,
    pub direction: Direction,
    pub state_goto: u8,
}

impl TMTransition {
    /// Transforms a standard transition triplet to a TMTransition.
    ///
    /// For instance, "1RA" gives `TMTransition { write: 1, direction: Direction::RIGHT, state_goto: 0 }`.
    ///
    /// ```
    /// use decider_bouncers_reproduction::directional_tm::{TMTransition, Direction};
    /// assert_eq!(TMTransition::from_std_str_triple("1RA"), Some(TMTransition { write: 1, direction: Direction::RIGHT, state_goto: 0 }));
    /// assert_eq!(TMTransition::from_std_str_triple("0LD"), Some(TMTransition { write: 0, direction: Direction::LEFT, state_goto: 3 }));
    /// assert_eq!(TMTransition::from_std_str_triple("---"), None);
    /// ```
    pub fn from_std_str_triple(triple: &str) -> Option<TMTransition> {
        if triple.len() != 3 || triple.chars().nth(0).unwrap() == '-' {
            return Option::None;
        }

        let write = triple.chars().nth(0).unwrap();
        let direction = triple.chars().nth(1).unwrap();
        let state_goto = triple.chars().nth(2).unwrap();

        let direction_enum = match direction {
            'L' => Direction::LEFT,
            'R' => Direction::RIGHT,
            _ => return Option::None,
        };

        Option::Some(TMTransition {
            write: (write as u8) - b'0',
            direction: direction_enum,
            state_goto: (state_goto as u8) - b'A',
        })
    }
}

pub struct TMTransitionTable {
    pub machine_std_format: String,
}

impl TMTransitionTable {
    pub fn new(machine_std_format: &str) -> TMTransitionTable {
        TMTransitionTable {
            machine_std_format: machine_std_format.to_string(),
        }
    }

    /// Returns the transition corresponding to a given state and read symbol.
    ///
    /// ```
    /// use decider_bouncers_reproduction::directional_tm::{TMTransition, Direction, TMTransitionTable};
    /// let machine_str = "1RB1LE_1LC1RD_1LB1RC_1LA0RD_---0LA";
    /// let transition_table = TMTransitionTable::new(machine_str);
    /// assert_eq!(transition_table.get_transition(0, 0), Some(TMTransition { write: 1, direction: Direction::RIGHT, state_goto: 1 }));
    /// assert_eq!(transition_table.get_transition(0, 1), Some(TMTransition { write: 1, direction: Direction::LEFT, state_goto: 4 }));
    /// assert_eq!(transition_table.get_transition(4, 0), None);
    /// assert_eq!(transition_table.get_transition(4, 1), Some(TMTransition { write: 0, direction: Direction::LEFT, state_goto: 0 }));
    /// ```
    pub fn get_transition(&self, state: u8, read: u8) -> Option<TMTransition> {
        let machine_split: Vec<&str> = self.machine_std_format.split("_").collect();
        let read_usize = read as usize;
        return TMTransition::from_std_str_triple(
            &machine_split[state as usize][3 * read_usize..3 * read_usize + 3],
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct TapeHead {
    state: u8,
    pointing_direction: Direction,
}

#[derive(Debug, Clone, Copy)]
enum TapeContent {
    InfiniteZero,
    Symbol(u8),
    Head(TapeHead),
}

/// Directional Turing Machine Configuration, with additional information stored for convenience.
pub struct Configuration {
    machine_transition: TMTransitionTable,
    tape: VecDeque<TapeContent>,
    head_pos: usize,
    step_count: i32,
}

use std::{collections::VecDeque, fmt};

impl fmt::Display for Configuration {
    /// Returns the string representation of a Configuration.
    ///
    /// ```
    /// use decider_bouncers_reproduction::directional_tm::{TMTransition, Direction, TMTransitionTable, Configuration};
    /// let machine_str = "1RB1LE_1LC1RD_1LB1RC_1LA0RD_---0LA";
    /// let configuration = Configuration::new(machine_str);
    /// assert_eq!(format!("{configuration}"), "0∞A>0∞");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.tape.len() {
            match &self.tape[i] {
                TapeContent::InfiniteZero => write!(f, "0∞")?,
                TapeContent::Symbol(x) => write!(f, "{}", x)?,
                TapeContent::Head(head) => {
                    if head.pointing_direction == Direction::RIGHT {
                        write!(f, "{}>", (head.state + b'A') as char)?;
                    } else {
                        write!(f, "<{}", (head.state + b'A') as char)?;
                    }
                }
            }
        }
        write!(f, "")
    }
}

impl Configuration {
    pub fn new(machine_std_format: &str) -> Configuration {
        Configuration {
            machine_transition: TMTransitionTable::new(machine_std_format),
            tape: VecDeque::from(vec![
                TapeContent::InfiniteZero,
                TapeContent::Head(TapeHead {
                    state: 0,
                    pointing_direction: Direction::RIGHT,
                }),
                TapeContent::InfiniteZero,
            ]),
            head_pos: 0,
            step_count: 0,
        }
    }

    fn get_tape_content(&self, pos: i32) -> Result<TapeContent, TMError> {
        if pos < 0 || pos >= self.tape.len().try_into().unwrap() {
            return Err(TMError::OutOfTapeError);
        }

        Ok(self.tape[pos as usize])
    }

    fn get_current_head(&self) -> Result<TapeHead, TMError> {
        match self.get_tape_content(self.head_pos as i32)? {
            TapeContent::Head(head) => Ok(head),
            _ => Err(TMError::InvalidConfigurationError),
        }
    }

    fn get_current_read_pos(&self) -> Result<i32, TMError> {
        return match self.get_current_head()?.pointing_direction {
            Direction::RIGHT => Result::Ok((self.head_pos as i32) + 1),
            Direction::LEFT => Result::Ok((self.head_pos as i32) - 1),
        };
    }

    fn get_current_read_content(&self) -> Result<TapeContent, TMError> {
        return Result::Ok(self.get_tape_content(self.get_current_read_pos()?)?);
    }

    fn get_current_read_symbol(&self) -> Result<u8, TMError> {
        return match self.get_current_read_content()? {
            TapeContent::Symbol(x) => Result::Ok(x),
            TapeContent::InfiniteZero => Result::Ok(0),
            TapeContent::Head(_) => Err(TMError::InvalidConfigurationError),
        };
    }

    fn get_current_transition(&self) -> Result<TMTransition, TMError> {
        let curr_transition = self.machine_transition.get_transition(
            self.get_current_head()?.state,
            self.get_current_read_symbol()?,
        );
        match curr_transition {
            Option::None => return Err(TMError::MachineHasHalted),
            Option::Some(transition) => Result::Ok(transition),
        }
    }

    /// Implements a directional Turing machine step, inplace.
    fn step(&mut self) -> Result<(), TMError> {
        let curr_head = self.get_current_head()?;
        let curr_read_pos = self.get_current_read_pos()?;
        let curr_read_content = self.get_current_read_content()?;
        let curr_transition = self.get_current_transition()?;

        match curr_read_content {
            TapeContent::Symbol(_) => {
                return Ok(());
            }
            TapeContent::InfiniteZero => {
                return Ok(());
            }
            TapeContent::Head(_) => {
                return Err(TMError::InvalidConfigurationError);
            }
        }
    }
}

pub fn hey() {
    println!("Hey from directional_tm!");
}
