#[macro_use]
extern crate lazy_static;

use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;

const A: i64 = 1103515245;
const C: i16 = 12345;
const M: i32 = 1 << 31;

pub fn rand<T: Randomable>() -> T {
    T::rand()
}

pub fn randn<T: Randomable>(n: T) -> T {
    T::randn(n)
}

pub fn rand_range<T: Randomable>(min: T, max: T) -> T {
    T::rand_range(min, max)
}

#[derive(Debug, Copy, Clone)]
pub struct Rng {
    seed: i64,
}

impl Rng {
    pub fn new() -> Rng {
        Rng {
            seed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        }
    }

    pub fn set_seed(&mut self, s: i64) {
        self.seed = s;
    }

    pub fn get_seed(self) -> i64 {
        self.seed
    }

    pub fn rand(&mut self) -> i64 {
        // https://stackoverflow.com/questions/3062746/special-simple-random-number-generator
        self.seed = (A * self.seed + C as i64) % M as i64;
        self.seed
    }

    pub fn randn(&mut self, n: i64) -> i64 {
        if n <= 0 {
            panic!("invalid argument, must be bigger than 0")
        }
        if n&(n-1) == 0 { // n is power of two, can mask
            return self.rand() & (n - 1);
        }
        let max: i64 = ((1 << 63) - 1 - (1<<63)%n as u64) as i64;
        let mut v = self.rand();
        while v > max {
            v = self.rand();
        }

        v % n
    }

    pub fn rand_range(&mut self, min: i64, max: i64) -> i64 {
        self.randn(max-min) +min
    }
}

lazy_static! {
    static ref BASE_RAND: Mutex<Rng> = Mutex::new(Rng::new());
}

pub trait Randomable {
    fn rand() -> Self;
    fn randn(n: Self) -> Self;
    fn rand_range(min: Self, max: Self) -> Self;
}

impl Randomable for u8 {
    fn rand() -> u8 {
        let range: u8 = 1 << 8 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as i64) as u8
    }
    fn randn(n: u8) -> u8 {
        BASE_RAND.lock().unwrap().randn(n as i64) as u8
    }
    fn rand_range(min: u8, max: u8) -> u8 {
        BASE_RAND.lock().unwrap().rand_range(min as i64, max as i64) as u8
    }
}

impl Randomable for u16 {
    fn rand() -> u16 {
        let range: u16 = 1 << 16 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as i64) as u16
    }
    fn randn(n: u16) -> u16 {
        BASE_RAND.lock().unwrap().randn(n as i64) as u16
    }
    fn rand_range(min: u16, max: u16) -> u16 {
        BASE_RAND.lock().unwrap().rand_range(min as i64, max as i64) as u16
    }
}

impl Randomable for u32 {
    fn rand() -> u32 {
        let range: u32 = 1 << 32 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as i64) as u32
    }
    fn randn(n: u32) -> u32 {
        BASE_RAND.lock().unwrap().randn(n as i64) as u32
    }
    fn rand_range(min: u32, max: u32) -> u32 {
        BASE_RAND.lock().unwrap().rand_range(min as i64, max as i64) as u32
    }
}

impl Randomable for u64 {
    fn rand() -> u64 {
        let range: i64 = 1 << 63 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range) as u64
    }
    fn randn(n: u64) -> u64 {
        BASE_RAND.lock().unwrap().randn(n as i64) as u64
    }
    fn rand_range(min: u64, max: u64) -> u64 {
        BASE_RAND.lock().unwrap().rand_range(min as i64, max as i64) as u64
    }
}

impl Randomable for usize {
    fn rand() -> usize {
        let range: u32 = 1 << 32 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as i64) as usize
    }
    fn randn(n: usize) -> usize {
        BASE_RAND.lock().unwrap().randn(n as i64) as usize
    }
    fn rand_range(min: usize, max: usize) -> usize {
        BASE_RAND.lock().unwrap().rand_range(min as i64, max as i64) as usize
    }
}

impl Randomable for i8 {
    fn rand() -> i8 {
        let range: i8 = 1 << 7 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as i64) as i8
    }
    fn randn(n: i8) -> i8 {
        BASE_RAND.lock().unwrap().randn(n as i64) as i8
    }
    fn rand_range(min: i8, max: i8) -> i8 {
        BASE_RAND.lock().unwrap().rand_range(min as i64, max as i64) as i8
    }
}

impl Randomable for i16 {
    fn rand() -> i16 {
        let range: i16 = 1 << 15 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as i64) as i16
    }
    fn randn(n: i16) -> i16 {
        BASE_RAND.lock().unwrap().randn(n as i64) as i16
    }
    fn rand_range(min: i16, max: i16) -> i16 {
        BASE_RAND.lock().unwrap().rand_range(min as i64, max as i64) as i16
    }
}

impl Randomable for i32 {
    fn rand() -> i32 {
        let range: i32 = 1 << 31 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as i64) as i32
    }
    fn randn(n: i32) -> i32 {
        BASE_RAND.lock().unwrap().randn(n as i64) as i32
    }
    fn rand_range(min: i32, max: i32) -> i32 {
        BASE_RAND.lock().unwrap().rand_range(min as i64, max as i64) as i32
    }
}

impl Randomable for i64 {
    fn rand() -> i64 {
        let range: i64 = 1 << 63 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as i64) as i64
    }
    fn randn(n: i64) -> i64 {
        BASE_RAND.lock().unwrap().randn(n as i64) as i64
    }
    fn rand_range(min: i64, max: i64) -> i64 {
        BASE_RAND.lock().unwrap().rand_range(min as i64, max as i64) as i64
    }
}

impl Randomable for isize {
    fn rand() -> isize {
        let range: i32 = 1 << 31 - 1;
        BASE_RAND.lock().unwrap().rand_range(0, range as i64) as isize
    }
    fn randn(n: isize) -> isize {
        BASE_RAND.lock().unwrap().randn(n as i64) as isize
    }
    fn rand_range(min: isize, max: isize) -> isize {
        BASE_RAND.lock().unwrap().rand_range(min as i64, max as i64) as isize
    }
}

impl Randomable for f32 {
    fn rand() -> f32 {
        let range: i32 = 1 << 31 - 1;
        let num = BASE_RAND.lock().unwrap().rand_range(10000, range as i64);
        let divider = BASE_RAND.lock().unwrap().rand_range(10, 1000);
        num as f32 / divider as f32
    }
    fn randn(n: f32) -> f32 {
        let divider = 999 as u64;
        let range = n as i64 * 999;
        
        let num = BASE_RAND.lock().unwrap().rand_range(0, range);
        num as f32 / divider as f32
    }
    fn rand_range(min: f32, max: f32) -> f32 {
        let num = BASE_RAND.lock().unwrap().rand_range(min as i64, max as i64);
        num as f32 / 9.9
    }
}

impl Randomable for f64 {
    fn rand() -> f64 {
        let range: i64 = 1 << 63 - 1;
        let num = BASE_RAND.lock().unwrap().rand_range(10000, range as i64);
        let divider = BASE_RAND.lock().unwrap().rand_range(10, 1000);
        num as f64 / divider as f64
    }
    fn randn(n: f64) -> f64 {
        let divider = 999 as u64;
        let range = n as i64 * 999;
        
        let num = BASE_RAND.lock().unwrap().rand_range(0, range);
        num as f64 / divider as f64
    }
    fn rand_range(min: f64, max: f64) -> f64 {
        let num = BASE_RAND.lock().unwrap().rand_range(min as i64, max as i64);
        num as f64 / 9.9
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::time::{Instant};

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
        println!("Time taken to generate a random number ({}): {:.2?}", data, now.elapsed());
    }
}
