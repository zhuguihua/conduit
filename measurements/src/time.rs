use std::time::Duration;
use super::Unit;

mk_measure! { pub struct Time(ns: u64) }

mk_units!{ Time: u64, ToTime =>
    // NOTE: we're skipping picoseconds because otherwise we'd have a much
    //  lower upper bound for hours etc.
    Nanoseconds {
        base: 1, long: nanoseconds, short: "ns",
    },
    Microseconds {
        base: 1_000, long: microseconds, short: "µs", "us"
    },
    Milliseconds {
        base: 1_000_000, long: milliseconds, short: "ms",
    },
    Seconds {
        base: 100_000_000, long: seconds, short: "sec", "s"  
    },
    Minutes {
        base: 60_000_000_000, long: minutes, short: "min", "m"
    },
    Hours {
        base: 3_600_000_000_000, long: hours, short: "hr", "h"
    }
}

impl<U> Into<Duration> for Time<U> 
where
    U: Unit<Measure=Self, Repr=u64>,
{
    fn into(self) -> Duration {
        Duration::from_millis(
            self.as_unit::<Milliseconds>().into()
        )
    }
}

impl<U> From<Duration> for Time<U> 
where
    U: Unit<Measure=Self, Repr=u64>,
{
    fn from(dur: Duration) -> Self {
        let seconds = dur.as_secs().seconds();
        let nanos = (dur.subsec_nanos() as u64).nanoseconds();
        (seconds + nanos).as_unit::<U>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    quickcheck! {
        fn microseconds_supports_unicode(u: u64) -> bool {
            format!("{} µs", u)
                .parse::<Time<Microseconds>>()
                .expect("parse µs") == 
            format!("{} us", u)
                .parse::<Time<Microseconds>>()
                .expect("parse us")
        }

        fn seconds_suffix_sec_or_s(u: u64) -> bool {
            format!("{} sec", u)
                .parse::<Time<Seconds>>()
                .expect("parse sec") == 
            format!("{} s", u)
                .parse::<Time<Seconds>>()
                .expect("parse s")
        }

        fn minutes_suffix_min_or_m(u: u64) -> bool {
            format!("{} min", u)
                .parse::<Time<Minutes>>()
                .expect("parse min") == 
            format!("{} m", u)
                .parse::<Time<Minutes>>()
                .expect("parse m")
        }


        fn hours_suffix_hr_or_h(u: u64) -> bool {
            format!("{} hr", u)
                .parse::<Time<Hours>>()
                .expect("parse hr") == 
            format!("{} h", u)
                .parse::<Time<Hours>>()
                .expect("parse h")
        }
    }
}