use std::time::{SystemTime, UNIX_EPOCH};

const A: i64 = 1103515245;
const C: i16 = 12345;
const M: i32 = 1 << 31;

struct Rng {
    seed: i64,
}

impl Rng {
    fn new() -> Rng {
        Rng {
            seed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        }
    }

    fn set_seed(mut self, s: i64) {
        self.seed = s;
    }

    pub fn rand(&mut self) -> i64 {
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
    use crate::{Rng};
    use std::time::{Instant};

    #[test]
    fn random() {
        let mut rng = Rng::new();
        
        for _ in 1..10 {
            let data = rng.randn(10000);
            println!("data: {}", data);
        }   
        let now = Instant::now();    
        let data = rng.randn(10000);
        println!("\n\n{:.2?}", now.elapsed());
        println!("data: {}", data); 
    }
}
