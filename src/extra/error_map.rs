use std::fmt::Display;
use ErrorType::*;

// ERROR TYPE
#[derive(Debug)]
pub enum ErrorType<T,U> {
    First(T),
    Last(U)
}

impl<T, U> ErrorType<T, U> {
    pub fn new_first<A> (value: T) -> Result<A, ErrorType<T,U>> {
        Err(First(value))
    }

    pub fn new_last<A> (value: U) -> Result<A, ErrorType<T,U>> {
        Err(Last(value))
    }

    pub fn map_first<T2, F: FnOnce(T) -> T2> (self, map: F) -> ErrorType<T2,U> {
        match self {
            First(x) => First(map(x)),
            Last(x) => Last(x)
        }
    }

    pub fn map_last<U2, F: FnOnce(U) -> U2> (self, map: F) -> ErrorType<T,U2> {
        match self {
            First(x) => First(x),
            Last(x) => Last(map(x))
        }
    }

    pub fn map_to_first<F: FnOnce(U) -> T> (self, map: F) -> T {
        match self {
            First(x) => x,
            Last(x) => map(x)
        }
    }

    pub fn map_to_last<F: FnOnce(T) -> U> (self, map: F) -> U {
        match self {
            First(x) => map(x),
            Last(x) => x
        }
    }

    pub fn map_single<A, F1: FnOnce(T) -> A, F2: FnOnce(U) -> A> (self, first: F1, last: F2) -> A {
        match self {
            First(x) => first(x),
            Last(x) => last(x)
        }
    }
}

impl<T,U> ErrorType<ErrorType<T,U>,U> {
    pub fn flatten (self) -> ErrorType<T,U> {
        match self {
            First(x) => x,
            Last(x) => Last(x)
        }
    }
}

impl<T,U> ErrorType<T,ErrorType<T,U>> {
    pub fn flatten (self) -> ErrorType<T,U> {
        match self {
            First(x) => First(x),
            Last(x) => x
        }
    }
}

impl<T,U> Display for ErrorType<T,U> where T: Display, U: Display {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            First(x) => x.fmt(f),
            Last(x) => x.fmt(f)
        }
    }
}

// FLAT MAP
pub trait ResultFlatMap<R1,E1> {
    fn flat_map <R2, E2, O: FnOnce(R1) -> Result<R2,E2>> (self, map: O) -> Result<R2,ErrorType<E1,E2>>;
    fn flat_map_single <R2, O: FnOnce(R1) -> Result<R2,E1>> (self, map: O) -> Result<R2,E1>;
}

pub trait OptionFlatMap<T> {
    fn flat_map <T2, F: FnOnce(T) -> Option<T2>> (self, map: F) -> Option<T2>;
}

impl<T> OptionFlatMap<T> for Option<T> {
    fn flat_map <T2, F: FnOnce(T) -> Option<T2>> (self, map: F) -> Option<T2> {
        match self {
            None => None,
            Some(x) => map(x)
        }
    }
}   

impl<R1,E1> ResultFlatMap<R1, E1> for Result<R1,E1> {
    fn flat_map <R2, E2, O: FnOnce(R1) -> Result<R2,E2>> (self, map: O) -> Result<R2,ErrorType<E1,E2>> {
        match self {
            Err(x) => Err(First(x)),
            Ok(x) => map(x).map_err(|z| Last(z))
        }
    }

    fn flat_map_single <R2, O: FnOnce(R1) -> Result<R2,E1>> (self, map: O) -> Result<R2,E1> {
        match self {
            Err(x) => Err(x),
            Ok(x) => map(x)
        }
    }
}

// FLATTERN
pub trait Flattern<R,E1> {
    fn flattern<E2, F: FnOnce() -> E2> (self, err: F) -> Result<R, ErrorType<E1,E2>>;
    fn flattern_single<F: FnOnce() -> E1> (self, err: F) -> Result<R, E1>;
}

impl<R,E> Flattern<R,E> for Option<Result<R,E>>  {
    fn flattern<E2, F: FnOnce() -> E2> (self, err: F) -> Result<R, ErrorType<E,E2>> {
        match self {
            Some(x) => x.map_err(|e| First(e)),
            None => ErrorType::new_last(err())
        }
    }

    fn flattern_single<F: FnOnce() -> E> (self, err: F) -> Result<R, E> {
        match self {
            Some(x) => x,
            None => Err(err())
        }
    }
}

impl<R,E> Flattern<R,E> for Result<Option<R>,E>  {
    fn flattern<E2, F: FnOnce() -> E2> (self, err: F) -> Result<R, ErrorType<E,E2>> {
        match self {
            Err(x) => ErrorType::new_first(x),
            Ok(x) => match x {
                Some(x) => Ok(x),
                None => ErrorType::new_last(err())
            }
        }
    }

    fn flattern_single<F: FnOnce() -> E> (self, err: F) -> Result<R, E> {
        match self {
            Err(x) => Err(x),
            Ok(x) => match x {
                Some(x) => Ok(x),
                None => Err(err())
            }
        }
    }
}