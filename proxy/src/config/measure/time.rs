use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use std::marker::PhantomData;
use ::config::{parse_number, ParseError};

use super::Unit;

#[derive(Copy, Clone, Debug, Eq, Ord)]
pub struct Time<U>
where
    U: Unit<Measure=Time<U>>,
{
    ns: usize,
    unit: PhantomData<U>,
}
