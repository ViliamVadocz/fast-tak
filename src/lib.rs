#![warn(clippy::pedantic, clippy::nursery, clippy::style)]
#![allow(clippy::module_name_repetitions)]

mod board;
mod colors;
mod error;
mod game;
mod game_result;
mod move_gen;
mod ptn;
mod reserves;
mod stack;
mod symm;
mod tps;
mod wins;

pub use board::Board;
pub use error::{PlayError, StackError, TakeError};
pub use game::Game;
pub use game_result::GameResult;
pub use move_gen::perf_count;
pub use reserves::Reserves;
pub use symm::Symmetry;
pub use takparse;
pub use wins::MAX_REVERSIBLE_PLIES;
