use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use std::marker::PhantomData;
use ::config::{parse_number, ParseError};

use super::Unit;

#[derive(Copy, Clone, Debug, Eq, Ord)]
pub struct Storage<U>
where
    U: Unit<Measure=Storage<U>>,
{
    bytes: usize,
    unit: PhantomData<U>,
}

// ===== impl Storage =====

impl<U> fmt::Display for Storage<U>
where 
    U: Unit<Measure=Storage<U>>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let float_value = 
            (self.bytes as f64) / (U::BASE_UNITS_PER_UNIT as f64);

        write!(f,
            "{number} {name}{plural}",
            number=float_value,
            name=U::NAME,
            plural=if float_value == 1f64 { "" } else { "s" }
        )
    }
}

impl<U> From<usize> for Storage<U>
where
    U: Unit<Measure=Storage<U>>,
{
    fn from(u: usize) -> Self {
        Self {
            bytes: u * U::BASE_UNITS_PER_UNIT,
            unit: PhantomData
        }
    }
}

impl<A> Storage<A> 
where 
    A: Unit<Measure=Storage<A>>
{
    pub fn into<B>(self) -> Storage<B>
    where
        B: Unit<Measure=Storage<B>>
    {
        Storage {
            bytes: self.bytes,
            unit: PhantomData,
        }
    }
}

impl<A, B> PartialEq<Storage<B>> for Storage<A>
where   
    A: Unit<Measure=Storage<A>>,
    B: Unit<Measure=Storage<B>>,
{
    fn eq(&self, rhs: &Storage<B>) -> bool{
        self.bytes == rhs.bytes
    }
}

impl<A, B> PartialOrd<Storage<B>> for Storage<A>
where   
    A: Unit<Measure=Storage<A>>,
    B: Unit<Measure=Storage<B>>,
{
    fn partial_cmp(&self, rhs: &Storage<B>) -> Option<Ordering>{
        self.bytes.partial_cmp(&rhs.bytes)
    }
}

impl_ops! { measure: Storage, base_unit: bytes =>
    Add, add,
    Sub, sub,
    Div, div,
    Mul, mul
}

mk_units!{ Storage =>
    Bytes    , "bytes"    , "B" , 1,
    Kilobytes, "kilobytes", "KB", 1_024,
    Megabytes, "megabytes", "MB", 1_048_576,
    Gigabytes, "gigabytes", "GB", 1_073_741_824
}

impl<U> FromStr for Storage<U> 
where 
    U: Unit<Measure=Storage<U>>
{
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ignore leading + trailing whitespace.
        let s = s.trim();
        // XXX: this is kinda janky, what we really need is just like an 
        //      iterator-oriented LL(1) parser...
        let num_part = 
            // XXX: also we dont support hex because `is_numeric` will gobble up 
            //      the 'b's in *bytes...
            s.trim_matches(|c: char| !c.is_digit(10))
             .trim(); // trim again to skip any interstital whitespace.
        let unit_part: String = 
            s.trim_matches(|c: char| !c.is_alphabetic())
            // NOTE: could save a string allocation by matching patterns 
            //       like `"B" | "b"`, but that ends up looking much uglier
            //       and this shouldn't be in the hot path...
             .to_lowercase(); 
        let num: usize = parse_number(num_part)?;
        match unit_part[..].trim() {
            "b"   => Ok(Storage::<Bytes>::from(num).into::<U>()),
            "kb"  => Ok(Storage::<Kilobytes>::from(num).into::<U>()),
            "mb"  => Ok(Storage::<Megabytes>::from(num).into::<U>()),
            "gb"  => Ok(Storage::<Gigabytes>::from(num).into::<U>()),
            unit => {
                error!("invalid storage unit '{}'", unit);
                Err(ParseError::InvalidUnit)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_zero_cost() {
        use std::mem::size_of;
        assert_eq!(
            size_of::<Storage<Bytes>>(), size_of::<usize>()
        );
        assert_eq!(
            size_of::<Storage<Kilobytes>>(), size_of::<usize>()
        );
        assert_eq!(
            size_of::<Storage<Megabytes>>(), size_of::<usize>()
        );
        assert_eq!(
            size_of::<Storage<Gigabytes>>(), size_of::<usize>()
        );
    }

    #[test]
    fn parsing_simple() {
        assert_eq!(
            "15 GB".parse::<Storage<Gigabytes>>()
                    .expect("parse"),
            Storage::<Gigabytes>::from(15)
        );
        assert_eq!(
            "15 MB".parse::<Storage<Megabytes>>()
                    .expect("parse"),
            Storage::<Megabytes>::from(15)
        );
    }


    #[test]
    fn parsing_does_unit_conversions() {
        assert_eq!(
            "1024 B".parse::<Storage<Kilobytes>>()
                    .expect("parse"),
            Storage::<Kilobytes>::from(1)
        );

        assert_eq!(
            "4096 KB".parse::<Storage<Kilobytes>>()
                    .expect("parse"),
            Storage::<Megabytes>::from(4)
        );
    }


    #[test]
    fn parsing_is_case_insensitive() {
        assert_eq!(
            "15 gb".parse::<Storage<Gigabytes>>()
                   .expect("parse"),
            Storage::<Gigabytes>::from(15)
        );
        assert_eq!(
            "15 gB".parse::<Storage<Megabytes>>()
                    .expect("parse"),
            "15 GB".parse::<Storage<Gigabytes>>()
                   .expect("parse"),
        );
        assert_eq!(
            "15 Gb".parse::<Storage<Megabytes>>()
                    .expect("parse"),
            "15 gb".parse::<Storage<Gigabytes>>()
                   .expect("parse"),
        );
    }

    #[test]
    fn parsing_handles_leading_and_trailing_whitespace() {
        assert_eq!(
            " 15 GB".parse::<Storage<Gigabytes>>()
                   .expect("parse ' 15 GB'"),
            Storage::<Gigabytes>::from(15)
        );
        assert_eq!(
            "15GB".parse::<Storage<Gigabytes>>()
                  .expect("parse '15GB'"),
            Storage::<Gigabytes>::from(15)
        );
        assert_eq!(
            "15 gb ".parse::<Storage<Gigabytes>>()
                  .expect("parse '15 gb '"),
            Storage::<Gigabytes>::from(15)
        );
        assert_eq!(
            " 15 gb ".parse::<Storage<Gigabytes>>()
                  .expect("parse ' 15 gb '"),
            Storage::<Gigabytes>::from(15)
        );

    }
}
