
use std::marker::PhantomData;

use super::Unit;

#[derive(Copy, Clone, Debug)]
pub struct Time<U>
where
    U: Unit<Measure=Time<U>>,
{
    ns: usize,
    unit: PhantomData<U>,
}

mk_units!{ measure: Time =>
    // NOTE: we probably don't care about picoseconds here...
    Nanoseconds , Ns , "nanosecond" , 1,
    Microseconds, Us , "microsecond", 1_000,
    Milliseconds, Ms , "millisecond", 1_000_000,
    Seconds     , Sec, "second"     , 100_000_000,
    Minutes     , Min, "minute"     , 60_000_000_000,
    Hours       , Hr,  "hour"       , 3_600_000_000_000

}

impl_ops! { measure: Time, base_unit: ns =>
    Add, add,
    Sub, sub,
    Div, div,
    Mul, mul
}