// Copyright 2024 Nord Security
use std::time::Duration;
use tokio::time::MissedTickBehavior;

const D: Duration = Duration::from_secs(1);

mod interval {
    use super::*;

    fn incorrect1() {
        let _interval = tokio::time::interval(D);
    }

    fn incorrect2() {
        use tokio::time::interval;
        let _interval = interval(D);
    }

    fn different_function_with_same_name() {
        fn interval(_: Duration) -> u32 {
            42
        }
        let _interval = interval(D);
    }

    #[allow(tokio_time_interval)]
    fn correct() {
        let mut interval = tokio::time::interval(D);
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    }
}

mod interval_at {
    use tokio::time::Instant;

    use super::*;

    fn incorrect1() {
        let _interval = tokio::time::interval_at(Instant::now(), D);
    }

    fn incorrect2() {
        use tokio::time::interval_at;
        let _interval = interval_at(Instant::now(), D);
    }

    fn different_function_with_same_name() {
        fn interval_at(_: Instant, _: Duration) -> u32 {
            42
        }
        let _interval = interval_at(Instant::now(), D);
    }

    #[allow(tokio_time_interval)]
    fn correct() {
        let mut interval = tokio::time::interval_at(Instant::now(), D);
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    }
}

fn main() {}
