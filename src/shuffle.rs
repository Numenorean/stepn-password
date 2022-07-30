use java_rand::Random;

pub trait Shuffle {
	fn shuffle<T>(&mut self, arr: &mut [T]);
}

impl Shuffle for Random {
    fn shuffle<T>(&mut self, arr: &mut [T]) {
        for i in (1..arr.len()).rev() {
            let a = self.next_u32_bound((i + 1) as u32);
            arr.swap(i, a as usize);
        }
    }
}
