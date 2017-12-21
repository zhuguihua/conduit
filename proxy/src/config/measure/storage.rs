use std::str::FromStr;
use std::marker::PhantomData;
use ::config::{parse_number, ParseError};

use super::Unit;


mk_measure! { pub struct Storage(bytes) }

mk_units!{ Storage =>
    Bytes    , B , bytes      , 1,
    Kilobytes, KB, kilobytes  , 1_024,
    Megabytes, MB, megabytes  , 1_048_576,
    Gigabytes, GB, gigabytes  , 1_073_741_824
}


impl<U> FromStr for Storage<U> 
where 
    U: Unit<Measure=Storage<U>, Repr=u64>,
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
        let num: u64 = parse_number(num_part)?;
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
            size_of::<Storage<Bytes>>(), size_of::<u64>()
        );
        assert_eq!(
            size_of::<Storage<Kilobytes>>(), size_of::<u64>()
        );
        assert_eq!(
            size_of::<Storage<Megabytes>>(), size_of::<u64>()
        );
        assert_eq!(
            size_of::<Storage<Gigabytes>>(), size_of::<u64>()
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


    #[test]
    fn integer_conversions_dsl() {
        assert_eq!(10.bytes(), Storage::<Bytes>::from(10));
        assert_eq!(22.kilobytes(), Storage::<Kilobytes>::from(22));
        assert_eq!(90.megabytes(), Storage::<Megabytes>::from(90));
        assert_eq!(128.gigabytes(), Storage::<Gigabytes>::from(128));
    }
}
