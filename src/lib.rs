#[macro_use]
extern crate lazy_static;

use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;

const A: i64 = 1103515245;
const C: i16 = 12345;
const M: i32 = 1 << 31;

lazy_static! {
    static ref BASE_RAND: Mutex<Rng> = Mutex::new(Rng::new());
}

pub fn rand() -> i64 {
    BASE_RAND.lock().unwrap().rand()
}

pub fn randn(n: i64) -> i64 {
    BASE_RAND.lock().unwrap().randn(n)
}

pub fn rand_range(min: i64, max: i64) -> i64 {
    BASE_RAND.lock().unwrap().rand_range(min, max)
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

#[cfg(test)]
mod tests {
    use crate::*;
    use std::time::{Instant};

    #[test]
    fn base_rand() {
        let n1 = rand();
        let n2 = rand();

        if n1 == n2 {
            panic!("{} shouldn't be equal with {}", n1, n2)
        }
    }

    #[test]
    fn base_randn() {
        let n1 = randn(10000);
        let n2 = randn(10000);

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
        let n1 = rand_range(4000, 10000);
        let n2 = rand_range(4000, 10000);

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
