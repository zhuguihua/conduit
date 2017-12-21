mk_measure! { pub struct Storage(bytes) }

mk_units!{ Storage, ToStorage =>
    Bytes {
        base: 1, long: bytes, short: "b",
    },
    Kilobytes { 
        base: 1_024, long: kilobytes, short: "kb",
    },
    Megabytes { 
        base: 1_048_576, long: megabytes, short: "mb",
    },
    Gigabytes {
        base: 1_073_741_824, long: gigabytes, short: "gb",
    }
}


// impl<U> FromStr for Storage<U> 
// where 
//     U: Unit<Measure=Storage<U>, Repr=u64>,
// {
//     type Err = MeasureError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         // ignore leading + trailing whitespace.
//         let s = s.trim();
//         // XXX: this is kinda janky, what we really need is just like an 
//         //      iterator-oriented LL(1) parser...
//         let num_part = 
//             // XXX: also we dont support hex because `is_numeric` will gobble up 
//             //      the 'b's in *bytes...
//             s.trim_matches(|c: char| !c.is_digit(10))
//              .trim(); // trim again to skip any interstital whitespace.
//         let unit_part: String = 
//             s.trim_matches(|c: char| !c.is_alphabetic())
//             // NOTE: could save a string allocation by matching patterns 
//             //       like `"B" | "b"`, but that ends up looking much uglier
//             //       and this shouldn't be in the hot path...
//              .to_lowercase(); 
//         let num: u64 = num_part.parse().map_err(MeasureError::from)?;
//         match unit_part[..].trim() {
//             "b"  => Ok(Storage::<Bytes>::from(num).into::<U>()),
//             "kb" => Ok(Storage::<Kilobytes>::from(num).into::<U>()),
//             "mb" => Ok(Storage::<Megabytes>::from(num).into::<U>()),
//             "gb" => Ok(Storage::<Gigabytes>::from(num).into::<U>()),
//             _    => Err(MeasureError::InvalidUnit),
//         }
//     }
// }

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

    quickcheck!{
        fn parsing_simple(u: u64) -> bool{
            format!("{} B", u)
                .parse::<Storage<Bytes>>()
                .expect("parse") == Storage::<Bytes>::from(u) && 
            format!("{} KB", u)
                .parse::<Storage<Kilobytes>>()
                .expect("parse") == Storage::<Kilobytes>::from(u) &&
            format!("{} MB", u)
                .parse::<Storage<Megabytes>>()
                .expect("parse") == Storage::<Megabytes>::from(u) &&
            format!("{} GB", u)
                .parse::<Storage<Gigabytes>>()
                .expect("parse") == Storage::<Gigabytes>::from(u)
        }

        fn parsing_long_names(u: u64) -> bool{
            format!("{} bytes", u)
                .parse::<Storage<Bytes>>()
                .expect("parse bytes") == Storage::<Bytes>::from(u) && 
            format!("{} kilobytes", u)
                .parse::<Storage<Kilobytes>>()
                .expect("parse kilobytes") == Storage::<Kilobytes>::from(u) &&
            format!("{} megabytes", u)
                .parse::<Storage<Megabytes>>()
                .expect("parse megabytes") == Storage::<Megabytes>::from(u) &&
            format!("{} gigabytes", u)
                .parse::<Storage<Gigabytes>>()
                .expect("parse gigabytes") == Storage::<Gigabytes>::from(u)
        }

        fn parsing_is_case_insensitive(u: u64) -> bool {
            format!("{} gb", u)
                .parse::<Storage<Gigabytes>>()
                .expect("parse 'gb'") == Storage::<Gigabytes>::from(u) &&
            format!("{} gB", u)
                .parse::<Storage<Gigabytes>>()
                .expect("parse 'gB'") == Storage::<Gigabytes>::from(u) &&
            format!("{} GB", u)
                .parse::<Storage<Gigabytes>>()
                .expect("parse 'GB'") == Storage::<Gigabytes>::from(u) &&
            format!("{} gB", u)
                .parse::<Storage<Gigabytes>>()
                .expect("parse 'gB'") == Storage::<Gigabytes>::from(u)
        }

        fn parsing_handles_leading_and_trailing_whitespace(u: u64) -> bool {
            format!("{} gb", u)
                .parse::<Storage<Gigabytes>>()
                .expect("parse 'n gb'") == Storage::<Gigabytes>::from(u) &&
            format!(" {} gb", u)
                .parse::<Storage<Gigabytes>>()
                .expect("parse ' n gB'") == Storage::<Gigabytes>::from(u) &&
            format!("{}gb", u)
                .parse::<Storage<Gigabytes>>()
                .expect("parse 'ngb'") == Storage::<Gigabytes>::from(u) &&
            format!("{} gb ", u)
                .parse::<Storage<Gigabytes>>()
                .expect("parse 'n gb '") == Storage::<Gigabytes>::from(u) &&
            format!(" {} gb ", u)
                .parse::<Storage<Gigabytes>>()
                .expect("parse ' n gb '") ==  Storage::<Gigabytes>::from(u)

        }

        fn integer_conversion_dsl(u: u64) -> bool {
            u.bytes() == Storage::<Bytes>::from(u)         &&
            u.kilobytes() == Storage::<Kilobytes>::from(u) && 
            u.megabytes() == Storage::<Megabytes>::from(u) && 
            u.gigabytes() == Storage::<Gigabytes>::from(u)
 
        }
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
}
