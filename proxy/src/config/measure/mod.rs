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
            U: Unit<Measure=$measure<U>>,
        {
            $base_unit: $repr,
            unit: PhantomData<U>,
        }

        mk_measure!{ @impl $measure($base_unit:$repr) }
    };
    (struct $measure:ident($base_unit:ident:$repr:ty)) => {
        #[derive(Copy, Clone, Debug)]
        struct $measure<U>
        where
            U: Unit<Measure=$measure<U>>,
        {
            $base_unit: $repr,
            unit: PhantomData<U>,
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
            U: Unit<Measure=$measure<U>, Repr=$repr>,
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

        impl<U> From<$repr> for $measure<U>
        where
            U: Unit<Measure=$measure<U>, Repr=$repr>,
        {
            fn from(u: $repr) -> Self {
                Self {
                    $base_unit: u * U::BASE_UNITS_PER_UNIT,
                    unit: PhantomData
                }
            }
        }

        impl<A> $measure<A> 
        where 
            A: Unit<Measure=$measure<A>, Repr=$repr>,
        {
            pub fn into<B>(self) -> $measure<B>
            where
                B: Unit<Measure=$measure<B>, Repr=$repr>,
            {
                $measure {
                    $base_unit: self.$base_unit,
                    unit: PhantomData,
                }
            }
        }

        impl<A, B> PartialEq<$measure<B>> for $measure<A>
        where   
            A: Unit<Measure=$measure<A>, Repr=$repr>,
            B: Unit<Measure=$measure<B>, Repr=$repr>,
        {
            fn eq(&self, rhs: &$measure<B>) -> bool{
                self.$base_unit == rhs.$base_unit
            }
        }

        impl<A, B> PartialOrd<$measure<B>> for $measure<A>
        where   
            A: Unit<Measure=$measure<A>, Repr=$repr>,
            B: Unit<Measure=$measure<B>, Repr=$repr>,
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
                A: Unit<Measure=$measure<A>, Repr=$repr>,
                B: Unit<Measure=$measure<B>>,
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
        $measure:ident => 
        $($name:ident, $short_name:ident, $long_name:ident, $base_per:expr),+
    ) => {
        mk_units! { $measure: u64 => 
            $($name, $short_name, $long_name, $base_per),+
        }
    };
    (
        $measure:ident : $repr:ty => 
        $($name:ident, $short_name:ident, $long_name:expr, $base_per:expr),+
    ) => {
        $(
            pub type $short_name = $measure<$name>;

            #[derive(Copy, Clone, Debug, Eq, PartialEq)]
            pub struct $name;

            impl Unit for $name {
                type Measure = $measure<$name>;
                type Repr = $repr;
                const NAME: &'static str = stringify!($long_name);
                const SHORT_NAME: &'static str = stringify!($short_name);
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

pub mod storage;
pub mod time;