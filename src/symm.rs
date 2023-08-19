use std::array;

use takparse::{Direction, Move, MoveKind, Square};

use crate::{board::Board, game::Game};

pub trait Symmetry<const N: usize>: Sized {
    fn symmetries(&self) -> [Self; 8];
}

impl<const N: usize> Symmetry<N> for Square {
    fn symmetries(&self) -> [Self; 8] {
        let n = u8::try_from(N).unwrap();
        [
            *self,
            self.rotate(n),
            self.rotate(n).rotate(n),
            self.rotate(n).rotate(n).rotate(n),
            self.mirror(n),
            self.mirror(n).rotate(n),
            self.mirror(n).rotate(n).rotate(n),
            self.mirror(n).rotate(n).rotate(n).rotate(n),
        ]
    }
}

impl<const N: usize> Symmetry<N> for Direction {
    fn symmetries(&self) -> [Self; 8] {
        [
            *self,
            self.rotate(),
            self.rotate().rotate(),
            self.rotate().rotate().rotate(),
            self.mirror(),
            self.mirror().rotate(),
            self.mirror().rotate().rotate(),
            self.mirror().rotate().rotate().rotate(),
        ]
    }
}

impl<const N: usize> Symmetry<N> for Move {
    fn symmetries(&self) -> [Self; 8] {
        let square = self.square();
        let kind = self.kind();
        match kind {
            MoveKind::Place(_) => {
                Symmetry::<N>::symmetries(&square).map(|square| Self::new(square, kind))
            }
            MoveKind::Spread(direction, pattern) => zip(
                Symmetry::<N>::symmetries(&square),
                Symmetry::<N>::symmetries(&direction),
            )
            .map(|(square, direction)| Self::new(square, MoveKind::Spread(direction, pattern))),
        }
    }
}

impl<const N: usize> Symmetry<N> for Board<N> {
    fn symmetries(&self) -> [Self; 8] {
        let n = u8::try_from(N).unwrap();
        array::from_fn(|i| {
            let mut board = Self::default();
            for x in 0..n {
                for y in 0..n {
                    let square = Square::new(y, x);
                    let target = Symmetry::<N>::symmetries(&square)[i];
                    *board.get_mut(target).unwrap() = *self.get(square).unwrap();
                }
            }
            board
        })
    }
}

impl<const N: usize, const HALF_KOMI: i8> Symmetry<N> for Game<N, HALF_KOMI> {
    fn symmetries(&self) -> [Self; 8] {
        let mut iter = self.board.symmetries().into_iter();
        array::from_fn(|_| {
            let mut game = self.clone();
            game.board = iter.next().unwrap();
            game
        })
    }
}

#[inline]
fn zip<const N: usize, A: Copy, B: Copy>(a: [A; N], b: [B; N]) -> [(A, B); N] {
    array::from_fn(|i| (a[i], b[i]))
}

impl<const N: usize, const HALF_KOMI: i8> Game<N, HALF_KOMI> {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn canonical(mut self) -> Self {
        self.board = self.board.symmetries().into_iter().min().unwrap();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{reserves::Reserves, symm::Symmetry, Game, GameResult, PlayError};

    fn symmetrical_boards<const N: usize>(seed: usize) -> Result<(), PlayError>
    where
        Reserves<N>: Default,
    {
        let mut game: Game<N, 0> = Game::default();
        let mut moves = Vec::new();
        while matches!(game.result(), GameResult::Ongoing) {
            moves.clear();
            game.possible_moves(&mut moves);
            let my_move = moves[seed % moves.len()];

            let mut game_syms = game.symmetries();
            let move_syms = Symmetry::<N>::symmetries(&my_move);

            game.play(my_move)?;
            game_syms
                .iter_mut()
                .zip(move_syms)
                .try_for_each(|(game, m)| game.play(m))?;

            let result = game.result();
            assert!(game_syms.iter().all(|game| game.result() == result));
        }
        Ok(())
    }

    macro_rules! symmetrical_boards_seeded {
        [$($name:ident $seed:literal),*] => {
            $(
                #[test]
                fn $name() {
                    symmetrical_boards::<3>($seed).unwrap();
                    symmetrical_boards::<4>($seed).unwrap();
                    symmetrical_boards::<5>($seed).unwrap();
                    symmetrical_boards::<6>($seed).unwrap();
                    symmetrical_boards::<7>($seed).unwrap();
                    symmetrical_boards::<8>($seed).unwrap();
                }
            )*
        };
    }

    symmetrical_boards_seeded![
        symmetrical_boards_5915587277 5_915_587_277,
        symmetrical_boards_1500450271 1_500_450_271,
        symmetrical_boards_3267000013 3_267_000_013,
        symmetrical_boards_5754853343 5_754_853_343,
        symmetrical_boards_4093082899 4_093_082_899,
        symmetrical_boards_9576890767 9_576_890_767,
        symmetrical_boards_3628273133 3_628_273_133,
        symmetrical_boards_2860486313 2_860_486_313,
        symmetrical_boards_5463458053 5_463_458_053,
        symmetrical_boards_3367900313 3_367_900_313
    ];
}
