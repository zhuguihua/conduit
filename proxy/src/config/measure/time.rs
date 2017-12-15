
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
