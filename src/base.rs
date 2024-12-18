use std::time::{SystemTime, UNIX_EPOCH};

const A: i64 = 1103515245;
const C: i16 = 12345;
const M: u64 = 1 << 63;

#[derive(Debug, Copy, Clone)]
pub struct Rng {
    seed: u128,
}

impl Rng {
    pub fn new() -> Rng {
        Rng {
            seed: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        }
    }

    pub fn set_seed(&mut self, s: u128) {
        self.seed = s;
    }

    pub fn get_seed(self) -> u128 {
        self.seed
    }

    pub fn rand(&mut self) -> u128 {
        // https://stackoverflow.com/questions/3062746/special-simple-random-number-generator
        self.seed = ((A as u128 * self.seed + C as u128) % M as u128) as u128;
        self.seed
    }

    pub fn randn(&mut self, n: u128) -> u128 {
        if n <= 0 {
            panic!("invalid argument, must be bigger than 0");
        }

        if n & (n - 1) == 0 {
            // n is power of two, can mask
            return self.rand() & (n - 1);
        }

        // Ensure we don't subtract more than we have
        let max: u128 = if n > (1 << 63) {
            // If n is larger than (1 << 63), avoid overflow
            (1 << 63) - 1
        } else {
            (1 << 63) - 1 - ((1 << 63) % n)
        };

        let mut v = self.rand();
        while v > max {
            v = self.rand();
        }

        v % n
    }

    pub fn rand_range(&mut self, min: u128, max: u128) -> u128 {
        self.randn(max - min) + min
    }
}

pub trait Randomable {
    fn rand(rng: &mut Rng) -> Self;
    fn randn(rng: &mut Rng, n: Self) -> Self;
    fn rand_range(rng: &mut Rng, min: Self, max: Self) -> Self;

    fn into_randomable(self) -> Self;
}

impl Randomable for u8 {
    fn rand(rng: &mut Rng) -> u8 {
        let range: u128 = (1 << 8) - 1;
        rng.rand_range(0, range) as u8
    }

    fn randn(rng: &mut Rng, n: u8) -> u8 {
        rng.randn(n as u128) as u8
    }

    fn rand_range(rng: &mut Rng, min: u8, max: u8) -> u8 {
        rng.rand_range(min as u128, max as u128) as u8
    }

    fn into_randomable(self) -> u8 {
        self // No transformation needed
    }
}

impl Randomable for u16 {
    fn rand(rng: &mut Rng) -> u16 {
        let range: u128 = (1 << 16) - 1;
        rng.rand_range(0, range) as u16
    }

    fn randn(rng: &mut Rng, n: u16) -> u16 {
        rng.randn(n as u128) as u16
    }

    fn rand_range(rng: &mut Rng, min: u16, max: u16) -> u16 {
        rng.rand_range(min as u128, max as u128) as u16
    }

    fn into_randomable(self) -> u16 {
        self
    }
}

impl Randomable for u32 {
    fn rand(rng: &mut Rng) -> u32 {
        let range: u128 = (1 << 32) - 1;
        rng.rand_range(0, range) as u32
    }

    fn randn(rng: &mut Rng, n: u32) -> u32 {
        rng.randn(n as u128) as u32
    }

    fn rand_range(rng: &mut Rng, min: u32, max: u32) -> u32 {
        rng.rand_range(min as u128, max as u128) as u32
    }

    fn into_randomable(self) -> u32 {
        self
    }
}

impl Randomable for u64 {
    fn rand(rng: &mut Rng) -> u64 {
        let range: u64 = u64::MAX;
        rng.rand_range(0, range as u128) as u64
    }

    fn randn(rng: &mut Rng, n: u64) -> u64 {
        rng.randn(n as u128) as u64
    }

    fn rand_range(rng: &mut Rng, min: u64, max: u64) -> u64 {
        rng.rand_range(min as u128, max as u128) as u64
    }

    fn into_randomable(self) -> u64 {
        self
    }
}

impl Randomable for u128 {
    fn rand(rng: &mut Rng) -> u128 {
        rng.rand_range(0, u128::MAX)
    }

    fn randn(rng: &mut Rng, n: u128) -> u128 {
        rng.randn(n)
    }

    fn rand_range(rng: &mut Rng, min: u128, max: u128) -> u128 {
        rng.rand_range(min, max)
    }

    fn into_randomable(self) -> u128 {
        self
    }
}

impl Randomable for usize {
    fn rand(rng: &mut Rng) -> usize {
        rng.rand_range(0, usize::MAX as u128) as usize
    }

    fn randn(rng: &mut Rng, n: usize) -> usize {
        rng.randn(n as u128) as usize
    }

    fn rand_range(rng: &mut Rng, min: usize, max: usize) -> usize {
        rng.rand_range(min as u128, max as u128) as usize
    }

    fn into_randomable(self) -> usize {
        self
    }
}

impl Randomable for i8 {
    fn rand(rng: &mut Rng) -> i8 {
        rng.rand_range(i8::MIN as u128, i8::MAX as u128) as i8
    }

    fn randn(rng: &mut Rng, n: i8) -> i8 {
        rng.randn(n as u128) as i8
    }

    fn rand_range(rng: &mut Rng, min: i8, max: i8) -> i8 {
        rng.rand_range(min as u128, max as u128) as i8
    }

    fn into_randomable(self) -> i8 {
        self
    }
}

impl Randomable for i16 {
    fn rand(rng: &mut Rng) -> i16 {
        rng.rand_range(i16::MIN as u128, i16::MAX as u128) as i16
    }

    fn randn(rng: &mut Rng, n: i16) -> i16 {
        rng.randn(n as u128) as i16
    }

    fn rand_range(rng: &mut Rng, min: i16, max: i16) -> i16 {
        rng.rand_range(min as u128, max as u128) as i16
    }

    fn into_randomable(self) -> i16 {
        self
    }
}

impl Randomable for i32 {
    fn rand(rng: &mut Rng) -> i32 {
        rng.rand_range(i32::MIN as u128, i32::MAX as u128) as i32
    }

    fn randn(rng: &mut Rng, n: i32) -> i32 {
        rng.randn(n as u128) as i32
    }

    fn rand_range(rng: &mut Rng, min: i32, max: i32) -> i32 {
        rng.rand_range(min as u128, max as u128) as i32
    }

    fn into_randomable(self) -> i32 {
        self
    }
}

impl Randomable for i64 {
    fn rand(rng: &mut Rng) -> i64 {
        rng.rand_range(i64::MIN as u128, i64::MAX as u128) as i64
    }

    fn randn(rng: &mut Rng, n: i64) -> i64 {
        rng.randn(n as u128) as i64
    }

    fn rand_range(rng: &mut Rng, min: i64, max: i64) -> i64 {
        rng.rand_range(min as u128, max as u128) as i64
    }

    fn into_randomable(self) -> i64 {
        self
    }
}

impl Randomable for i128 {
    fn rand(rng: &mut Rng) -> i128 {
        rng.rand_range(i128::MIN as u128, i128::MAX as u128) as i128
    }

    fn randn(rng: &mut Rng, n: i128) -> i128 {
        rng.randn(n as u128) as i128
    }

    fn rand_range(rng: &mut Rng, min: i128, max: i128) -> i128 {
        rng.rand_range(min as u128, max as u128) as i128
    }

    fn into_randomable(self) -> i128 {
        self
    }
}

impl Randomable for isize {
    fn rand(rng: &mut Rng) -> isize {
        rng.rand_range(isize::MIN as u128, isize::MAX as u128) as isize
    }

    fn randn(rng: &mut Rng, n: isize) -> isize {
        rng.randn(n as u128) as isize
    }

    fn rand_range(rng: &mut Rng, min: isize, max: isize) -> isize {
        rng.rand_range(min as u128, max as u128) as isize
    }

    fn into_randomable(self) -> isize {
        self
    }
}

impl Randomable for f32 {
    fn rand(rng: &mut Rng) -> f32 {
        rng.rand_range(0, u32::MAX as u128) as f32 / u32::MAX as f32
    }

    fn randn(rng: &mut Rng, n: f32) -> f32 {
        (rng.randn((n * u32::MAX as f32) as u128) as f32) / u32::MAX as f32
    }

    fn rand_range(rng: &mut Rng, min: f32, max: f32) -> f32 {
        let range = max - min;
        min + (rng.rand_range(0, u32::MAX as u128) as f32 / u32::MAX as f32) * range
    }

    fn into_randomable(self) -> f32 {
        self
    }
}

impl Randomable for f64 {
    fn rand(rng: &mut Rng) -> f64 {
        rng.rand_range(0, u64::MAX as u128) as f64 / u64::MAX as f64
    }

    fn randn(rng: &mut Rng, n: f64) -> f64 {
        (rng.randn((n * u64::MAX as f64) as u128) as f64) / u64::MAX as f64
    }

    fn rand_range(rng: &mut Rng, min: f64, max: f64) -> f64 {
        let range = max - min;
        min + (rng.rand_range(0, u64::MAX as u128) as f64 / u64::MAX as f64) * range
    }

    fn into_randomable(self) -> f64 {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_random_range<T: Randomable + PartialOrd + std::fmt::Debug + Copy>(
        rng: &mut Rng,
        min: T,
        max: T,
    ) {
        for _ in 0..100 {
            let value = T::rand_range(rng, min, max);
            assert!(
                value >= min && value < max,
                "Value {:?} not in range [{:?}, {:?})",
                value,
                min,
                max
            );
        }
    }

    fn test_randomn<T: Randomable + PartialOrd + std::fmt::Debug + Copy>(rng: &mut Rng, n: T) {
        for _ in 0..100 {
            let value = T::randn(rng, n);
            assert!(value < n, "Value {:?} is not less than {:?}", value, n);
        }
    }

    #[test]
    fn test_u64_randomable() {
        let mut rng = Rng::new();
        rng.set_seed(1);
        let value = u64::rand(&mut rng);
        assert_eq!(value, 1103527590, "u64::rand() generated a negative value");

        test_random_range(&mut rng, 10u64, 100u64);
        test_randomn(&mut rng, 50u64);
    }

    #[test]
    fn test_u128_randomable() {
        let mut rng = Rng::new();
        rng.set_seed(1);
        let value = u128::rand(&mut rng);
        assert_eq!(value, 1103527590, "u128::rand() generated a negative value");

        test_random_range(&mut rng, 100u128, 1000u128);
        test_randomn(&mut rng, 500u128);
    }

    #[test]
    fn test_usize_randomable() {
        let mut rng = Rng::new();
        rng.set_seed(1);
        let value = usize::rand(&mut rng);
        assert_eq!(value, 1103527590, "usize::rand() generated a negative value");

        test_random_range(&mut rng, 5usize, 25usize);
        test_randomn(&mut rng, 15usize);
    }

    #[test]
    fn test_f32_randomable() {
        let mut rng = Rng::new();
        for _ in 0..100 {
            let value = f32::rand(&mut rng);
            assert!(
                value >= 0.0 && value <= 1.0,
                "f32::rand() generated out of range: {}",
                value
            );

            let range_value = f32::rand_range(&mut rng, 0.0, 10.0);
            assert!(
                range_value >= 0.0 && range_value < 10.0,
                "f32::rand_range() generated out of range: {}",
                range_value
            );

            let n_value = f32::randn(&mut rng, 5.0);
            assert!(
                n_value < 5.0,
                "f32::randn() generated out of range: {}",
                n_value
            );
        }
    }

    #[test]
    fn test_f64_randomable() {
        let mut rng = Rng::new();
        for _ in 0..100 {
            let value = f64::rand(&mut rng);
            assert!(
                value >= 0.0 && value <= 1.0,
                "f64::rand() generated out of range: {}",
                value
            );

            let range_value = f64::rand_range(&mut rng, 0.0, 10.0);
            assert!(
                range_value >= 0.0 && range_value < 10.0,
                "f64::rand_range() generated out of range: {}",
                range_value
            );

            let n_value = f64::randn(&mut rng, 5.0);
            assert!(
                n_value < 5.0,
                "f64::randn() generated out of range: {}",
                n_value
            );
        }
    }
}
