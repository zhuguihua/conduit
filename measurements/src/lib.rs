#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use std::fmt;
use std::num::ParseIntError;
use std::error::Error;

#[macro_export]
macro_rules! mk_measure {
    (struct $measure:ident($base_unit:ident))=> {
        mk_measure!{ struct $measure($base_unit: u64) }
    };
    (pub struct $measure:ident($base_unit:ident)) => {
        mk_measure!{ pub struct $measure($base_unit: u64) }
    };
    (pub struct $measure:ident($base_unit:ident:$repr:ty)) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $measure<U>
        where
            U: $crate::Unit<Measure=$measure<U>>,
        {
            $base_unit: $repr,
            unit: ::std::marker::PhantomData<U>,
        }

        mk_measure!{ @impl $measure($base_unit:$repr) }
    };
    (struct $measure:ident($base_unit:ident:$repr:ty)) => {
        #[derive(Copy, Clone, Debug)]
        struct $measure<U>
        where
            U: $crate::Unit<Measure=$measure<U>>,
        {
            $base_unit: $repr,
            unit: std::marker::PhantomData<U>,
        }

        mk_measure!{ @impl $measure($base:$repr) }
    };

    (@impl $measure:ident($base_unit:ident:$repr:ty))=> {
        impl_ops! { measure: $measure, base_unit: $base_unit: $repr =>
            Add, add,
            Sub, sub,
            Div, div,
            Mul, mul
        }

        impl<U> ::std::fmt::Display for $measure<U>
        where 
            U: $crate::Unit<Measure=$measure<U>, Repr=$repr>,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let float_value = 
                    (self.$base_unit as f64) / (U::BASE_UNITS_PER_UNIT as f64);
                let plural_idx = if float_value == 1f64 { 
                    U::NAME.len() - 1
                } else {
                    U::NAME.len()
                };
                write!(f,
                    "{number} {name}",
                    number=float_value,
                    name=&U::NAME[..plural_idx],
                )
            }
        }

        impl<U> From<$repr> for $measure<U>
        where
            U: $crate::Unit<Measure=$measure<U>, Repr=$repr>,
        {
            fn from(u: $repr) -> Self {
                Self {
                    $base_unit: u * U::BASE_UNITS_PER_UNIT,
                    unit: ::std::marker::PhantomData,
                }
            }
        }

        impl<A> $measure<A> 
        where 
            A: $crate::Unit<Measure=$measure<A>, Repr=$repr>,
        {
            pub fn into<B>(self) -> $measure<B>
            where
                B: $crate::Unit<Measure=$measure<B>, Repr=$repr>,
            {
                $measure {
                    $base_unit: self.$base_unit,
                    unit: ::std::marker::PhantomData,
                }
            }
        }

        impl<A, B> PartialEq<$measure<B>> for $measure<A>
        where   
            A: $crate::Unit<Measure=$measure<A>, Repr=$repr>,
            B: $crate::Unit<Measure=$measure<B>, Repr=$repr>,
        {
            fn eq(&self, rhs: &$measure<B>) -> bool{
                self.$base_unit == rhs.$base_unit
            }
        }

        impl<A, B> PartialOrd<$measure<B>> for $measure<A>
        where   
            A: $crate::Unit<Measure=$measure<A>, Repr=$repr>,
            B: $crate::Unit<Measure=$measure<B>, Repr=$repr>,
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
        measure: $measure:ident, base_unit: $base_unit:ident: $repr:ty => 
        $($trait:ident, $fun:ident ),+
    ) => {
        $(
            impl<A, B> ::std::ops::$trait<$measure<B>> for $measure<A>
            where   
                A: $crate::Unit<Measure=$measure<A>, Repr=$repr>,
                B: $crate::Unit<Measure=$measure<B>>,
            {
                type Output = Self;
                fn $fun(self, rhs: $measure<B>) -> Self {
                    Self {
                        $base_unit: self.$base_unit.$fun(rhs.$base_unit),
                        unit: ::std::marker::PhantomData
                    }
                }
            }
        )+
    }
}
#[macro_export]
macro_rules! mk_units {
    (
        $measure:ident, $to_measure:ident => 
        $(
            $name:ident { 
                base: $base_per:expr,
                long: $long_name:ident, 
                short: $short_name:expr, $($extra_name:expr),*
            }
        ),+
    ) => {
        mk_units! { $measure : u64, $to_measure => 
            $(
            $name { 
                base: $base_per,
                long: $long_name,
                short: $short_name, $($extra_name),*
            }
            ),+
        }
    };
    (
        $measure:ident : $repr:ty, $to_measure:ident => 
        $(
            $name:ident { 
                base: $base_per:expr,
                long: $long_name:ident, 
                short: $short_name:expr, $($extra_name:expr),*
            }
        ),+
    ) => {
        mk_units_inner! {
            $measure : $repr, stringify!($measure), $to_measure => 
            $(
                $name { 
                    base: $base_per,
                    long: $long_name, stringify!($long_name), 
                    short: $short_name, $($extra_name),*
                }
            ),+
        }
    };
}
// factored out so we can stringify more eagerly for making doc comments.
macro_rules! mk_units_inner {
    (
        $measure:ident : $repr:ty, $smeasure:expr, $to_measure:ident => 
        $(
            $name:ident { 
                base: $base_per:expr,
                long: $long_name:ident, $slong_name:expr,
                short: $short_name:expr, $($extra_name:expr),* 
            }
        ),+
    ) => {

        #[doc = "Trait for conversions to "] #[doc = $smeasure] #[doc = "."]
        pub trait $to_measure {
            $(
                fn $long_name(self) -> $measure<$name>;
            )+
        }

        impl $to_measure for $repr {
            $(
                fn $long_name(self) -> $measure<$name> {
                    $measure::<$name>::from(self)
                }
            )+
        }

        impl<U> ::std::str::FromStr for $measure<U> 
        where 
            U: $crate::Unit<Measure=$measure<U>, Repr=$repr>,
            $repr: ::std::str::FromStr,
            $crate::MeasureError: From<<$repr as ::std::str::FromStr>::Err>
        {
            type Err = $crate::MeasureError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // ignore leading + trailing whitespace.
                let s = s.trim();
                // XXX: this is kinda janky, what we really need is just like 
                //      an iterator-oriented LL(1) parser...
                let num_part = 
                    // XXX: also we dont support hex because `is_numeric` will 
                    //      gobble up the 'b's in *bytes...
                    s.trim_matches(|c: char| !c.is_digit(10))
                    .trim(); // trim again to skip any interstital whitespace.
                let unit_part: String = 
                    s.trim_matches(|c: char| !c.is_alphabetic())
                    // NOTE: could save a string allocation by matching 
                    // patterns like `"B" | "b"`, but that's much harder to 
                    // generate from a macro (and significantly uglier) and 
                    // this shouldn't be in the hot path...
                    .to_lowercase(); 
                let num: u64 = num_part
                    .parse()
                    .map_err($crate::MeasureError::from)?;
                match unit_part[..].trim() {
                    $(
                       $($extra_name |)* $short_name | $slong_name =>
                            Ok($measure::<$name>::from(num).into::<U>()),
                    )+
                    _    => Err($crate::MeasureError::InvalidUnit),
                }
            }
        }
        $(

            #[doc = "Unit representing a measurement of "] 
            #[doc = $smeasure] #[doc = " in "] #[doc = $slong_name] 
            #[doc = "s."]
            #[derive(Copy, Clone, Debug, Eq, PartialEq)]
            pub struct $name;

            impl $crate::Unit for $name {
                type Measure = $measure<$name>;
                type Repr = $repr;
                const NAME: &'static str = $slong_name;
                const SHORT_NAME: &'static str = $short_name;
                const BASE_UNITS_PER_UNIT: $repr = $base_per;
            }
        )+
    }
} 

/// Trait representing a measurement unit.
pub trait Unit {
    /// The corresponding type measurable with this unit.
    type Measure;
    /// Representation of a value measured by this unit.
    /// 
    /// This must be the same as the type used internally by 
    /// `Self::Measure`.
    // TODO: if `Measure` was a trait as well, the `Repr` type could go th
    type Repr;
    const NAME: &'static str;
    const SHORT_NAME: &'static str;
    const BASE_UNITS_PER_UNIT: Self::Repr;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MeasureError {
    /// An invalid unit was used.
    InvalidUnit,
    InvalidNumber(ParseIntError),
}

pub mod storage;
pub mod time;

// ===== impl MeasureError =====

impl Error for MeasureError {
    fn description(&self) -> &str {
        match *self {
            MeasureError::InvalidUnit => "invalid unit",
            MeasureError::InvalidNumber(_) => "invalid number",
        }
    }

    fn cause(&self) -> Option<&Error> {
        if let MeasureError::InvalidNumber(ref err) = *self {
            Some(err)
        } else {
            None
        }
    }
}

impl fmt::Display for MeasureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MeasureError::InvalidUnit => 
                write!(f, "invalid unit found in string"),
            MeasureError::InvalidNumber(ref err) => 
                write!(f, "invalid number: {}", err),
        }
    }
}

impl From<ParseIntError> for MeasureError {
    fn from(err: ParseIntError) -> Self {
        MeasureError::InvalidNumber(err)
    }
}