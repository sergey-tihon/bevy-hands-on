use rand::{Rng, SeedableRng, distributions::uniform::SampleRange, rngs::StdRng};

pub use rand;

pub struct RandomNumberGenerator {
    rng: StdRng,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            rng: StdRng::from_entropy(),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn range<T>(&mut self, range: impl SampleRange<T>) -> T
    where
        T: rand::distributions::uniform::SampleUniform + PartialOrd,
    {
        self.rng.gen_range(range)
    }

    pub fn next<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::prelude::Distribution<T>,
    {
        self.rng.r#gen()
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_bounds() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let num = rng.range(10..20);
            assert!((10..20).contains(&num));
        }
    }

    #[test]
    fn test_seeded_reproducibility() {
        let mut rng1 = RandomNumberGenerator::seeded(42);
        let mut rng2 = RandomNumberGenerator::seeded(42);

        for _ in 0..1000 {
            assert_eq!(
                rng1.range(u32::MIN..u32::MAX),
                rng2.range(u32::MIN..u32::MAX)
            );
        }
    }

    #[test]
    fn test_next_types() {
        let mut rng = RandomNumberGenerator::new();
        let _: i32 = rng.next();
        let _: f64 = rng.next();
        let _ = rng.next::<usize>();
    }

    #[test]
    fn test_floating_point() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.range(-5000.0f32..5000.0f32);
            assert!(n.is_finite());
            assert!(n >= -5000.0 && n < 5000.0);
        }
    }
}
