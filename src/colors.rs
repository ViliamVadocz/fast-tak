use takparse::Color;

type BitVec = u64;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Colors {
    bits: BitVec,
}

impl Default for Colors {
    fn default() -> Self {
        Self { bits: 1 }
    }
}

impl Colors {
    #[inline]
    pub const fn of_one(color: Color) -> Self {
        Self {
            bits: 0b10 + from_color(color),
        }
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.bits == 1
    }

    #[inline]
    pub const fn len(self) -> u32 {
        BitVec::BITS - (self.bits.leading_zeros() + 1)
    }

    #[inline]
    pub const fn top(self) -> Option<Color> {
        if self.is_empty() {
            return None;
        }
        Some(to_color(self.bits & 1))
    }

    #[inline]
    pub fn push(&mut self, color: Color) {
        self.bits = (self.bits << 1) | BitVec::from(color == Color::White);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Color> {
        if self.is_empty() {
            return None;
        }
        let color = to_color(self.bits & 1);
        self.bits >>= 1;
        Some(color)
    }

    #[inline]
    pub fn take(&mut self, amount: u32) -> Option<Self> {
        if amount > self.len() {
            return None;
        }
        let mask: BitVec = !(!0 << amount);
        let bits = (1 << amount) | (self.bits & mask);
        let taken = Self { bits };
        self.bits >>= amount;
        Some(taken)
    }

    #[inline]
    pub const fn reverse(self) -> Self {
        let len = self.len();
        let bits = (1 << len) | (self.bits.reverse_bits() >> (BitVec::BITS - len));
        Self { bits }
    }
}

impl IntoIterator for Colors {
    type IntoIter = ColorsIter;
    type Item = Color;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ColorsIter(self.reverse())
    }
}

pub struct ColorsIter(Colors);

impl Iterator for ColorsIter {
    type Item = Color;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len() as usize;
        (len, Some(len))
    }
}

const fn to_color(n: BitVec) -> Color {
    if n == 0 {
        Color::Black
    } else {
        Color::White
    }
}

const fn from_color(color: Color) -> BitVec {
    match color {
        Color::White => 1,
        Color::Black => 0,
    }
}

impl FromIterator<Color> for Colors {
    fn from_iter<T: IntoIterator<Item = Color>>(iter: T) -> Self {
        let mut colors = Self::default();
        for color in iter {
            colors.push(color);
        }
        colors
    }
}

#[cfg(test)]
mod tests {
    use takparse::Color;

    use super::{from_color, to_color, Colors};

    #[test]
    fn color_num() {
        assert_eq!(Color::White, to_color(from_color(Color::White)));
        assert_eq!(Color::Black, to_color(from_color(Color::Black)));
        assert_eq!(from_color(Color::White), 1);
        assert_eq!(from_color(Color::Black), 0);
    }

    #[test]
    fn push_pop() {
        let mut colors = Colors::default();
        colors.push(Color::White);
        colors.push(Color::White);
        colors.push(Color::Black);
        colors.push(Color::White);
        colors.push(Color::Black);

        assert_eq!(colors.len(), 5);
        assert_eq!(colors.pop(), Some(Color::Black));
        assert_eq!(colors.pop(), Some(Color::White));
        assert_eq!(colors.pop(), Some(Color::Black));
        assert_eq!(colors.pop(), Some(Color::White));
        assert_eq!(colors.pop(), Some(Color::White));
        assert_eq!(colors.pop(), None);
    }

    #[test]
    fn iter() {
        let mut colors = Colors::of_one(Color::White);
        colors.push(Color::Black);
        colors.push(Color::Black);
        colors.push(Color::White);
        colors.push(Color::White);
        colors.push(Color::Black);

        assert_eq!(colors.len(), 6);
        let v: Vec<_> = colors.into_iter().collect();
        assert_eq!(v, [
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::White,
            Color::Black
        ]);
    }

    #[test]
    fn take() {
        let mut colors = Colors::of_one(Color::White);
        colors.push(Color::Black);
        colors.push(Color::White);
        colors.push(Color::Black);
        colors.push(Color::Black);
        colors.push(Color::White);

        assert_eq!(colors.len(), 6);
        let mut a = colors.take(5).unwrap();
        assert_eq!(colors.len(), 1);
        assert_eq!(a.len(), 5);

        assert_eq!(a.pop(), Some(Color::White));
        assert_eq!(a.pop(), Some(Color::Black));
        assert_eq!(a.pop(), Some(Color::Black));
        assert_eq!(a.pop(), Some(Color::White));
        assert_eq!(a.pop(), Some(Color::Black));
        assert_eq!(a.pop(), None);

        assert_eq!(colors.pop(), Some(Color::White));
        assert_eq!(colors.pop(), None);
    }

    #[test]
    fn reverse() {
        let mut colors = Colors::of_one(Color::White);
        colors.push(Color::White);
        colors.push(Color::Black);
        colors.push(Color::Black);
        colors.push(Color::Black);
        colors.push(Color::White);

        assert_eq!(colors.len(), 6);
        assert_eq!(colors, colors.reverse().reverse());
        let v: Vec<_> = colors.reverse().into_iter().collect();
        assert_eq!(v, [
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::White
        ]);

        // Check that iterator goes from bottom to top.
        let mut iter = colors.into_iter();
        let mut reverse = colors.reverse();
        assert_eq!(reverse.len(), 6);
        while let (Some(x), Some(y)) = (iter.next(), reverse.pop()) {
            assert_eq!(x, y);
        }
        assert_eq!(iter.count(), 0);
        assert!(reverse.is_empty());
    }
}
