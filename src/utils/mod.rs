use anyhow::{anyhow, Error};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option::NoneError;

#[derive(Debug)]
pub struct AOCError {
    pub e: Error,
}

pub type AOCResult<T> = Result<T, AOCError>;

impl std::fmt::Display for AOCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.e)
    }
}

// This type of implementation allows me to use '?' to
// return early from an Optional that resolves to None in
// a function that returns Result<T, E>.
//
// This special logic -- creating a new struct -- is required because
//   1. NoneError does not implement std::error::Error and therefore can't
//      be given straight to an anyhow::Error
//   2. I cannot implement std's Error for NoneError -- I've seen this referred
//      to as orphan implementation. I own neither type nor trait; it's illegal.
//
//
impl From<NoneError> for AOCError {
    fn from(_: NoneError) -> Self {
        AOCError {
            e: anyhow!("Got None, expected Some(.)"),
        }
    }
}

// Oh gosh what a nightmare. I can't implement both a generic impl for
// From<T: std::error::Error> AND From<NoneError> because the two seem
// to overlap! This is bizarre because NoneError doesn't implement
// std::error::Error -- if it did, specialization should fix it -- but
// I think it's because NoneError quacks like a std::error::Error, which
// confuses the type system.
//
// To get around this, I'm just manually implementing per error as I
// encounter them.
//
// I hate this so much.
macro_rules! aocerror {
    ($type:path) => {
        impl From<$type> for AOCError {
            fn from(e: $type) -> Self {
                AOCError { e: Error::from(e) }
            }
        }
    };
}
aocerror!(std::num::ParseIntError);
aocerror!(regex::Error);
aocerror!(strum::ParseError);

//
// WHAT DO YOU MEAN MACROS HAVE DIFFERENT EXPORT RULES?
// Because modules are secretly slurped together into one giant file at the
// end of the day, and macros are metaprogramming widgets that are applied
// during a preprocessing, it doesn't make sense to have them namespaced.
// So we have the super ugly custom bail! macro to replace the fact that
// I can't just use anyhow::bail!()
#[macro_export]
macro_rules! aocbail {
    ($($args:tt)*) => {
        return Err(AOCError {
            e: anyhow::anyhow!($($args)*),
        });
    };
}

#[macro_export]
macro_rules! regex {
    ($regex:literal) => {
        regex::Regex::new($regex).unwrap()
    };
}

pub fn get_input(filename: &str) -> Box<dyn Iterator<Item = String>> {
    Box::new(
        BufReader::new(File::open("input/".to_owned() + filename).unwrap())
            .lines()
            .map(|l| l.unwrap()),
    )
}
