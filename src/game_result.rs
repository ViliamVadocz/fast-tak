use takparse::{Color, WinReason};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameResult {
    Winner {
        color: Color,
        reason: Reason,
    },
    Draw {
        reason: Reason,
    },
    #[default]
    Ongoing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reason {
    Road,
    ReservesDepleted,
    BoardFill,
    ReversiblePlies,
}

impl From<Reason> for WinReason {
    fn from(reason: Reason) -> Self {
        match reason {
            Reason::Road => Self::Road,
            Reason::BoardFill | Reason::ReservesDepleted => Self::Flat,
            Reason::ReversiblePlies => Self::Other,
        }
    }
}

pub struct Ongoing;

impl TryFrom<GameResult> for takparse::GameResult {
    type Error = Ongoing;

    fn try_from(result: GameResult) -> Result<Self, Self::Error> {
        Ok(match result {
            GameResult::Ongoing => return Err(Ongoing),
            GameResult::Draw { .. } => Self::Draw,
            GameResult::Winner {
                color: Color::White,
                reason,
            } => Self::White(reason.into()),
            GameResult::Winner {
                color: Color::Black,
                reason,
            } => Self::Black(reason.into()),
        })
    }
}
