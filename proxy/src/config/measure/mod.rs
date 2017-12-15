macro_rules! impl_ops {
    (
        measure: $measure:ident, base_unit: $base_unit:ident => 
        $($trait:ident, $fun:ident ),+
    ) => {
        $(
            impl<A, B> ::std::ops::$trait<$measure<B>> for $measure<A>
            where   
                A: $crate::config::measure::Unit<Measure=$measure<A>>,
                B: $crate::config::measure::Unit<Measure=$measure<B>>,
            {
                type Output = Self;
                fn $fun(self, rhs: $measure<B>) -> Self {
                    Self {
                        bytes: self.$base_unit.$fun(rhs.$base_unit),
                        unit: PhantomData
                    }
                }
            }
        )+
    }
}

macro_rules! mk_units {
    (
        $measure:ident => 
        $($name:ident, $long_name:expr, $short_name:expr, $base_per:expr),+
    ) => {
        $(
            #[derive(Copy, Clone, Debug, Eq, PartialEq)]
            pub struct $name;

            impl Unit for $name {
                type Measure = $measure<$name>;
                const NAME: &'static str = $long_name;
                const SHORT_NAME: &'static str = $short_name;
                const BASE_UNITS_PER_UNIT: usize = $base_per;
            }
        )+
    }
}

pub trait Unit {
    type Measure;
    const NAME: &'static str;
    const SHORT_NAME: &'static str;
    const BASE_UNITS_PER_UNIT: usize;
}

pub mod storage;
pub mod time;