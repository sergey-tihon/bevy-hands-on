use rand::{Rng, SeedableRng, distributions::uniform::SampleRange};
use std::sync::Mutex;

pub use rand;

#[cfg(all(not(feature = "pcg"), not(feature = "xorshift")))]
type RngCore = rand::prelude::StdRng;

#[cfg(feature = "pcg")]
type RngCore = rand_pcg::Pcg64Mcg;

#[cfg(feature = "xorshift")]
type RngCore = rand_xorshift::XorShiftRng;

#[derive(bevy::prelude::Resource)]
pub struct RandomNumberGenerator {
    rng: Mutex<RngCore>,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            rng: Mutex::new(RngCore::from_entropy()),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: Mutex::new(RngCore::seed_from_u64(seed)),
        }
    }

    pub fn range<T>(&self, range: impl SampleRange<T>) -> T
    where
        T: rand::distributions::uniform::SampleUniform + PartialOrd,
    {
        let mut lock = self.rng.lock().unwrap();
        lock.gen_range(range)
    }

    pub fn next<T>(&self) -> T
    where
        rand::distributions::Standard: rand::prelude::Distribution<T>,
    {
        let mut lock = self.rng.lock().unwrap();
        lock.r#gen()
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RandomPlugin;

impl bevy::prelude::Plugin for RandomPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(RandomNumberGenerator::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_bounds() {
        let rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let num = rng.range(10..20);
            assert!((10..20).contains(&num));
        }
    }

    #[test]
    fn test_seeded_reproducibility() {
        let rng1 = RandomNumberGenerator::seeded(42);
        let rng2 = RandomNumberGenerator::seeded(42);

        for _ in 0..1000 {
            assert_eq!(
                rng1.range(u32::MIN..u32::MAX),
                rng2.range(u32::MIN..u32::MAX)
            );
        }
    }

    #[test]
    fn test_next_types() {
        let rng = RandomNumberGenerator::new();
        let _: i32 = rng.next();
        let _: f64 = rng.next();
        let _ = rng.next::<usize>();
    }

    #[test]
    fn test_floating_point() {
        let rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.range(-5000.0f32..5000.0f32);
            assert!(n.is_finite());
            assert!(n >= -5000.0 && n < 5000.0);
        }
    }
}
