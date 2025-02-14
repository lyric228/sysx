use rand::distr::uniform::SampleUniform;
use std::time::Duration;
use rand::Rng;
use num::Num;

pub use std::thread::sleep;


pub trait DurationUnits {
    fn ns(self) -> Duration;
    fn ms(self) -> Duration;
    fn s(self) -> Duration;
    fn m(self) -> Duration;
    fn h(self) -> Duration;
    fn d(self) -> Duration;
    fn y(self) -> Duration;
}

impl DurationUnits for u64 {
    fn ns(self) -> Duration {
        Duration::from_nanos(self)
    }

    fn ms(self) -> Duration {
        Duration::from_millis(self)
    }
    
    fn s(self) -> Duration {
        Duration::from_secs(self)
    }
    
    fn m(self) -> Duration {
        Duration::from_secs(self * 60)
    }
    
    fn h(self) -> Duration {
        Duration::from_secs(self * 60 * 60)
    }
    
    fn d(self) -> Duration {
        Duration::from_secs(self * 60 * 60 * 24)
    }
    
    fn y(self) -> Duration {
        Duration::from_secs(self * 60 * 60 * 24 * 365)
    }
}

pub fn random<T>(min: T, max: T) -> T 
where
    T: Num + Copy + SampleUniform + PartialOrd
{
    rand::rng().random_range(min..=max)
}
