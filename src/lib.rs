mod base;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

pub fn rand<T: Randomable>() -> T {
    T::rand()
}

pub fn randn<T: Randomable>(n: T) -> T {
    T::randn(n)
}

pub fn rand_range<T: Randomable>(min: T, max: T) -> T {
    T::rand_range(min, max)
}

pub fn set_seed<T: Randomable>(s: u128) {
    T::set_seed(s)
}

#[derive(Debug, Copy, Clone)]
pub struct Rng {
    base: base::Rng,
}

impl Rng {
    pub fn new() -> Rng {
        Rng {
            base: base::Rng::new(),
        }
    }

    pub fn set_seed(&mut self, s: u128) {
        self.base.set_seed(s)
    }

    pub fn get_seed(self) -> u128 {
        self.base.get_seed()
    }

    pub fn rand(&mut self) -> u128 {
        self.base.rand()
    }

    pub fn randn(&mut self, n: u128) -> u128 {
        self.base.randn(n)
    }

    pub fn rand_range(&mut self, min: u128, max: u128) -> u128 {
        self.base.rand_range(min, max)
    }
}

lazy_static! {
    static ref BASE_RAND: Mutex<Rng> = Mutex::new(Rng::new());
}

pub struct Random {
    rng: Mutex<base::Rng>,
}

impl Random {
    pub fn new(s: u128) -> Random {
        let mut rng = base::Rng::new();
        rng.set_seed(s);
        Random {
            rng: Mutex::new(rng),
        }
    }

    pub fn rand<T: base::Randomable>(&self) -> T {
        let mut rng = self.rng.lock().unwrap();
        T::rand(&mut rng)
    }

    pub fn randn<T: base::Randomable>(&self, n: T) -> T {
        let mut rng = self.rng.lock().unwrap();
        T::randn(&mut rng, n)
    }

    pub fn rand_range<T: base::Randomable>(&self, min: T, max: T) -> T {
        let mut rng = self.rng.lock().unwrap();
        T::rand_range(&mut rng, min, max)
    }

    pub fn set_seed(&self, s: u128) {
        self.rng.lock().unwrap().set_seed(s);
    }
}

pub trait Randomable: Sized {
    fn rand() -> Self;
    fn randn(n: Self) -> Self;
    fn rand_range(min: Self, max: Self) -> Self;

    fn set_seed(s: u128) {
        BASE_RAND.lock().unwrap().set_seed(s);
    }
}

impl Randomable for u8 {
    fn rand() -> u8 {
        let range: u8 = 1 << 8 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as u128) as u8
    }
    fn randn(n: u8) -> u8 {
        BASE_RAND.lock().unwrap().randn(n as u128) as u8
    }
    fn rand_range(min: u8, max: u8) -> u8 {
        BASE_RAND
            .lock()
            .unwrap()
            .rand_range(min as u128, max as u128) as u8
    }
}

impl Randomable for u16 {
    fn rand() -> u16 {
        let range: u16 = 1 << 16 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as u128) as u16
    }
    fn randn(n: u16) -> u16 {
        BASE_RAND.lock().unwrap().randn(n as u128) as u16
    }
    fn rand_range(min: u16, max: u16) -> u16 {
        BASE_RAND
            .lock()
            .unwrap()
            .rand_range(min as u128, max as u128) as u16
    }
}

impl Randomable for u32 {
    fn rand() -> u32 {
        let range: u32 = 1 << 32 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as u128) as u32
    }
    fn randn(n: u32) -> u32 {
        BASE_RAND.lock().unwrap().randn(n as u128) as u32
    }
    fn rand_range(min: u32, max: u32) -> u32 {
        BASE_RAND
            .lock()
            .unwrap()
            .rand_range(min as u128, max as u128) as u32
    }
}

impl Randomable for u64 {
    fn rand() -> u64 {
        let range: u128 = 1 << 63 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range) as u64
    }
    fn randn(n: u64) -> u64 {
        BASE_RAND.lock().unwrap().randn(n as u128) as u64
    }
    fn rand_range(min: u64, max: u64) -> u64 {
        BASE_RAND
            .lock()
            .unwrap()
            .rand_range(min as u128, max as u128) as u64
    }
}

impl Randomable for u128 {
    fn rand() -> u128 {
        let range: u32 = 1 << 32 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as u128)
    }
    fn randn(n: u128) -> u128 {
        BASE_RAND.lock().unwrap().randn(n)
    }
    fn rand_range(min: u128, max: u128) -> u128 {
        BASE_RAND.lock().unwrap().rand_range(min, max)
    }
}

impl Randomable for usize {
    fn rand() -> usize {
        let range: u32 = 1 << 32 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as u128) as usize
    }
    fn randn(n: usize) -> usize {
        BASE_RAND.lock().unwrap().randn(n as u128) as usize
    }
    fn rand_range(min: usize, max: usize) -> usize {
        BASE_RAND
            .lock()
            .unwrap()
            .rand_range(min as u128, max as u128) as usize
    }
}

impl Randomable for i8 {
    fn rand() -> i8 {
        let range: i8 = 1 << 7 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as u128) as i8
    }
    fn randn(n: i8) -> i8 {
        BASE_RAND.lock().unwrap().randn(n as u128) as i8
    }
    fn rand_range(min: i8, max: i8) -> i8 {
        BASE_RAND
            .lock()
            .unwrap()
            .rand_range(0u128, (max - min) as u128) as i8
            + min
    }
}

impl Randomable for i16 {
    fn rand() -> i16 {
        let range: i16 = 1 << 15 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as u128) as i16
    }
    fn randn(n: i16) -> i16 {
        BASE_RAND.lock().unwrap().randn(n as u128) as i16
    }
    fn rand_range(min: i16, max: i16) -> i16 {
        BASE_RAND
            .lock()
            .unwrap()
            .rand_range(0u128, (max - min) as u128) as i16
            + min
    }
}

impl Randomable for i32 {
    fn rand() -> i32 {
        let range: i32 = 1 << 31 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as u128) as i32
    }
    fn randn(n: i32) -> i32 {
        BASE_RAND.lock().unwrap().randn(n as u128) as i32
    }
    fn rand_range(min: i32, max: i32) -> i32 {
        BASE_RAND
            .lock()
            .unwrap()
            .rand_range(0u128, (max - min) as u128) as i32
            + min
    }
}

impl Randomable for i64 {
    fn rand() -> i64 {
        let range: i64 = 1 << 63 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as u128) as i64
    }
    fn randn(n: i64) -> i64 {
        BASE_RAND.lock().unwrap().randn(n as u128) as i64
    }
    fn rand_range(min: i64, max: i64) -> i64 {
        BASE_RAND
            .lock()
            .unwrap()
            .rand_range(0u128, (max - min) as u128) as i64
            + min
    }
}

impl Randomable for i128 {
    fn rand() -> i128 {
        let range: u32 = 1 << 32 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as u128) as i128
    }
    fn randn(n: i128) -> i128 {
        BASE_RAND.lock().unwrap().randn(n as u128) as i128
    }
    fn rand_range(min: i128, max: i128) -> i128 {
        BASE_RAND
            .lock()
            .unwrap()
            .rand_range(min as u128, max as u128) as i128
    }
}

impl Randomable for isize {
    fn rand() -> isize {
        let range: i32 = 1 << 31 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as u128) as isize
    }
    fn randn(n: isize) -> isize {
        BASE_RAND.lock().unwrap().randn(n as u128) as isize
    }
    fn rand_range(min: isize, max: isize) -> isize {
        BASE_RAND
            .lock()
            .unwrap()
            .rand_range(0u128, (max - min) as u128) as isize
            + min
    }
}

impl Randomable for f32 {
    fn rand() -> f32 {
        let range: i32 = 1 << 31 - 1;
        let num = BASE_RAND.lock().unwrap().rand_range(10000, range as u128);
        let divider = BASE_RAND.lock().unwrap().rand_range(2, 9);
        num as f32 / divider as f32
    }
    fn randn(n: f32) -> f32 {
        let divider = 29 as u128;
        let range = n as u128 * divider;

        let num = BASE_RAND.lock().unwrap().randn(range);
        num as f32 / divider as f32
    }
    fn rand_range(min: f32, max: f32) -> f32 {
        let num = BASE_RAND
            .lock()
            .unwrap()
            .rand_range(0u128, (max - min) as u128);
        (num as f32 + min) / 1.16453434
    }
}

impl Randomable for f64 {
    fn rand() -> f64 {
        let range: i64 = 1 << 63 - 1;
        let num = BASE_RAND.lock().unwrap().rand_range(10000, range as u128);
        let divider = BASE_RAND.lock().unwrap().rand_range(2, 9);
        num as f64 / divider as f64
    }
    fn randn(n: f64) -> f64 {
        let divider = 29 as u128;
        let range = n as u128 * divider;

        let num = BASE_RAND.lock().unwrap().randn(range);
        num as f64 / divider as f64
    }
    fn rand_range(min: f64, max: f64) -> f64 {
        let num = BASE_RAND
            .lock()
            .unwrap()
            .rand_range(0u128, (max - min) as u128);
        (num as f64 + min) / 1.16453434
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::time::Instant;

    #[test]
    fn base_rand() {
        let n1 = rand::<u32>();
        let n2 = rand::<u32>();

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
    }

    #[test]
    fn base_randn() {
        let n1 = randn::<u32>(10000);
        let n2 = randn::<u32>(10000);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > 10000 {
            panic!("{} should be smaller than 10000", n1)
        }
        if n2 > 10000 {
            panic!("{} should be smaller than 10000", n2)
        }
    }

    #[test]
    fn base_rand_range() {
        let n1 = rand_range::<u32>(4000, 10000);
        let n2 = rand_range::<u32>(4000, 10000);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > 10000 && n1 < 4000 {
            panic!("{} should be between 4000 and 10000", n1)
        }
        if n2 > 10000 && n2 < 4000 {
            panic!("{} should be between 4000 and 10000", n2)
        }
    }

    #[test]
    fn random() {
        let mut prev_data: i64 = 0;
        for _ in 1..10 {
            let data = randn(10000);
            if prev_data == data {
                panic!("{} shouldn't be equal with {}", prev_data, data)
            }
            prev_data = data
        }

        let now = Instant::now();
        let data = randn(10000);
        println!(
            "Time taken to generate a random number ({}): {:.2?}",
            data,
            now.elapsed()
        );
    }

    #[test]
    fn random_u8() {
        let min = 10;
        let max = 30;
        let n1 = rand_range::<u8>(min, max);
        let n2 = rand_range::<u8>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_u16() {
        let min = 10;
        let max = 30000;
        let n1 = rand_range::<u16>(min, max);
        let n2 = rand_range::<u16>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_u32() {
        let min = 10;
        let max = 3000000000;
        let n1 = rand_range::<u32>(min, max);
        let n2 = rand_range::<u32>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_u64() {
        let min = 10;
        let max = 3000000000000000000;
        let n1 = rand_range::<u64>(min, max);
        let n2 = rand_range::<u64>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_usize() {
        let min = 10;
        let max = 304444;
        let n1 = rand_range::<usize>(min, max);
        let n2 = rand_range::<usize>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_i8() {
        let min = 10;
        let max = 30;
        let n1 = rand_range::<i8>(min, max);
        let n2 = rand_range::<i8>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }
    #[test]
    fn random_i16() {
        let min = 10;
        let max = 30000;
        let n1 = rand_range::<i16>(min, max);
        let n2 = rand_range::<i16>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_i32() {
        let min = 10;
        let max = 300000000;
        let n1 = rand_range::<i32>(min, max);
        let n2 = rand_range::<i32>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_i64() {
        let min = 10;
        let max = 3000000000000000000;
        let n1 = rand_range::<i64>(min, max);
        let n2 = rand_range::<i64>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_isize() {
        let min = 10;
        let max = 304444;
        let n1 = rand_range::<isize>(min, max);
        let n2 = rand_range::<isize>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_f32() {
        let min = 10.1;
        let max = 30000.1;
        let n1 = rand_range::<f32>(min, max);
        let n2 = rand_range::<f32>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_f64() {
        let min = 10.1;
        let max = 30000000000.1;
        let n1 = rand_range::<f64>(min, max);
        let n2 = rand_range::<f64>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_i8_negative_min() {
        let min = -6;
        let max = 30;
        let n1 = rand_range::<i8>(min, max);
        let n2 = rand_range::<i8>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_i16_negative_min() {
        let min = -6;
        let max = 30;
        let n1 = rand_range::<i16>(min, max);
        let n2 = rand_range::<i16>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_i32_negative_min() {
        let min = -6;
        let max = 30;
        let n1 = rand_range::<i32>(min, max);
        let n2 = rand_range::<i32>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_i64_negative_min() {
        let min = -6;
        let max = 30;
        let n1 = rand_range::<i64>(min, max);
        let n2 = rand_range::<i64>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_isize_negative_min() {
        let min = -6;
        let max = 30;
        let n1 = rand_range::<isize>(min, max);
        let n2 = rand_range::<isize>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_i8_negative_range() {
        let min = -30;
        let max = -6;
        let n1 = rand_range::<i8>(min, max);
        let n2 = rand_range::<i8>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_i16_negative_range() {
        let min = -30;
        let max = -6;
        let n1 = rand_range::<i16>(min, max);
        let n2 = rand_range::<i16>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_i32_negative_range() {
        let min = -30;
        let max = -6;
        let n1 = rand_range::<i32>(min, max);
        let n2 = rand_range::<i32>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_i64_negative_range() {
        let min = -30;
        let max = -6;
        let n1 = rand_range::<i64>(min, max);
        let n2 = rand_range::<i64>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_isize_negative_range() {
        let min = -30;
        let max = -6;
        let n1 = rand_range::<isize>(min, max);
        let n2 = rand_range::<isize>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_f32_negative_min() {
        let min = -10.1;
        let max = 30000.1;
        let n1 = rand_range::<f32>(min, max);
        let n2 = rand_range::<f32>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_f64_negative_min() {
        let min = -10.1;
        let max = 30000.1;
        let n1 = rand_range::<f64>(min, max);
        let n2 = rand_range::<f64>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_f32_negative_range() {
        let min = -30000.1;
        let max = -10.1;
        let n1 = rand_range::<f32>(min, max);
        let n2 = rand_range::<f32>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_f64_negative_range() {
        let min = -30000.1;
        let max = -10.1;
        let n1 = rand_range::<f64>(min, max);
        let n2 = rand_range::<f64>(min, max);

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
        if n1 > max && n1 < min {
            panic!("{} should be between {} and {}", n1, min, max)
        }
        if n2 > max && n2 < min {
            panic!("{} should be between {} and {}", n2, min, max)
        }
    }

    #[test]
    fn random_instance() {
        let random = Random::new(1);
        let result: u16 = random.rand_range(6, 123);
        assert_eq!(result, 93);
    }
}
