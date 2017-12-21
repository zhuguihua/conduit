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