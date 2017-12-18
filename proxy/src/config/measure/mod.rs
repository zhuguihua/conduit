macro_rules! impl_measure {
    (measure: $measure:ident, base_unit: $base_unit:ident) => {

        impl_ops! { measure: $measure, base_unit: $base_unit =>
            Add, add,
            Sub, sub,
            Div, div,
            Mul, mul
        }

        impl<U> ::std::fmt::Display for $measure<U>
        where 
            U: Unit<Measure=$measure<U>>
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let float_value = 
                    (self.$base_unit as f64) / (U::BASE_UNITS_PER_UNIT as f64);

                write!(f,
                    "{number} {name}{plural}",
                    number=float_value,
                    name=U::NAME,
                    plural=if float_value == 1f64 { "" } else { "s" }
                )
            }
        }

        impl<U> From<usize> for $measure<U>
        where
            U: Unit<Measure=$measure<U>>,
        {
            fn from(u: usize) -> Self {
                Self {
                    $base_unit: u * U::BASE_UNITS_PER_UNIT,
                    unit: PhantomData
                }
            }
        }

        impl<A> $measure<A> 
        where 
            A: Unit<Measure=$measure<A>>
        {
            pub fn into<B>(self) -> $measure<B>
            where
                B: Unit<Measure=$measure<B>>
            {
                $measure {
                    $base_unit: self.$base_unit,
                    unit: PhantomData,
                }
            }
        }

        impl<A, B> PartialEq<$measure<B>> for $measure<A>
        where   
            A: Unit<Measure=$measure<A>>,
            B: Unit<Measure=$measure<B>>,
        {
            fn eq(&self, rhs: &$measure<B>) -> bool{
                self.$base_unit == rhs.$base_unit
            }
        }

        impl<A, B> PartialOrd<$measure<B>> for $measure<A>
        where   
            A: Unit<Measure=$measure<A>>,
            B: Unit<Measure=$measure<B>>,
        {
            fn partial_cmp(&self, rhs: &$measure<B>) 
                          -> Option<::std::cmp::Ordering>
            {
                self.$base_unit.partial_cmp(&rhs.$base_unit)
            }
        }
    }
}

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
                        $base_unit: self.$base_unit.$fun(rhs.$base_unit),
                        unit: PhantomData
                    }
                }
            }
        )+
    }
}

macro_rules! mk_units {
    (
        measure: $measure:ident => 
        $($name:ident, $short_name:ident, $long_name:expr, $base_per:expr),+
    ) => {
        $(
            pub type $short_name = $measure<$name>;

            #[derive(Copy, Clone, Debug, Eq, PartialEq)]
            pub struct $name;

            impl Unit for $name {
                type Measure = $measure<$name>;
                const NAME: &'static str = $long_name;
                const SHORT_NAME: &'static str = stringify!($short_name);
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