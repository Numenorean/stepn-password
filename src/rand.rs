use std::cell::Cell;

pub struct Rng(Cell<u64>);

impl Rng {
	const MULTIPLIER: u64 = 0x5DEECE66D;
    const ADDEND: u64 = 0xB;
    const MASK: u64 = 2_u64.pow(48) - 1;

    pub fn with_seed(seed: u64) -> Rng {
        let seed = Self::initial_scramble(seed);
        Self(Cell::new(seed))
    }

    pub fn next(&self, bits: u8) -> u32 {
        let r = self
            .0
            .get()
            .wrapping_mul(Self::MULTIPLIER)
            .wrapping_add(Self::ADDEND)
            & Self::MASK;
        self.0.set(r);
        (r >> 48 - bits & i32::MAX as u64) as u32
    }

    pub fn next_bound(&self, bound: u32) -> u32 {
        let r = self.next(31);

        if bound & bound - 1 == 0 {
            return (r as u64 * bound as u64 >> 31 & u32::MAX as u64) as u32;
        }

        r % bound
    }

    pub fn shuffle<T>(&self, arr: &mut [T]) {
        for i in (1..arr.len()).rev() {
            let a = self.next_bound((i + 1) as u32);
            arr.swap(i, a as usize);
        }
    }

    pub(self) fn initial_scramble(seed: u64) -> u64 {
        seed ^ Self::MULTIPLIER & Self::MASK
    }
}

#[cfg(test)]
mod tests {
    use super::Rng;

    #[test]
    fn initial_scramble() {
        let seed = Rng::initial_scramble(1997399150);
        assert_eq!(seed, 24324932099);
    }

    #[test]
    fn next() {
        let rng = Rng::with_seed(1997399150);
        assert_eq!(rng.next(31), 1864530096);
    }

    #[test]
    fn shuffle_array() {
        let rng = Rng::with_seed(1997399150);
        let mut arr: Vec<_> = (0..78).collect();
        rng.shuffle(&mut arr);
        assert_eq!(
            arr,
            [
                37, 12, 16, 71, 59, 32, 6, 22, 63, 36, 3, 31, 61, 70, 48, 52, 65, 76, 17, 8, 34,
                45, 54, 28, 73, 46, 20, 58, 18, 2, 39, 10, 24, 74, 19, 72, 21, 40, 26, 42, 64, 60,
                55, 35, 11, 56, 13, 30, 67, 69, 1, 47, 43, 50, 44, 53, 49, 68, 23, 38, 5, 27, 51,
                33, 29, 75, 77, 15, 7, 4, 25, 66, 62, 9, 57, 14, 41, 0
            ]
        );
    }

    #[test]
    fn shuffle_array_2() {
        let rng = Rng::with_seed(1997399150);
        let mut arr =
            b"dfb488dff049a35ae6bd81f32888de972edb0a98b47fd68b321ab79bf32c5ee0_1657986131278"
                .to_vec();
        rng.shuffle(&mut arr);
        assert_eq!(
            arr,
            [
                97, 97, 101, 54, 99, 50, 100, 102, 48, 48, 52, 55, 101, 56, 51, 98, 49, 55, 54,
                102, 100, 54, 57, 100, 51, 56, 56, 50, 98, 98, 56, 52, 50, 49, 100, 49, 49, 98, 56,
                55, 95, 53, 98, 98, 57, 102, 51, 57, 53, 57, 102, 98, 102, 49, 100, 55, 50, 55, 51,
                57, 56, 56, 97, 101, 101, 50, 56, 97, 102, 56, 56, 54, 101, 48, 51, 53, 52, 100
            ]
        )
    }
}
