
use std::marker::PhantomData;

use super::Unit;

mk_measure! { pub struct Time(ns: u64) }

mk_units!{ Time: u64 =>
    // NOTE: we probably don't care about picoseconds here...
    Nanoseconds , Ns , "nanosecond" , 1,
    Microseconds, Us , "microsecond", 1_000,
    Milliseconds, Ms , "millisecond", 1_000_000,
    Seconds     , Sec, "second"     , 100_000_000,
    Minutes     , Min, "minute"     , 60_000_000_000,
    Hours       , Hr,  "hour"       , 3_600_000_000_000

}

