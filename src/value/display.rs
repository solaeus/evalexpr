use std::fmt::{Display, Error, Formatter};

use crate::Value;

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::String(string) => write!(f, "\"{}\"", string),
            Value::Float(float) => write!(f, "{}", float),
            Value::Int(int) => write!(f, "{}", int),
            Value::Boolean(boolean) => write!(f, "{}", boolean),
            Value::Tuple(tuple) => {
                write!(f, "(")?;
                let mut once = false;
                for value in tuple {
                    if once {
                        write!(f, ", ")?;
                    } else {
                        once = true;
                    }
                    value.fmt(f)?;
                }
                write!(f, ")")
            },
            Value::Map(map) => {
                write!(f, "( ")?;
                let mut once = false;
                for (key, value) in map {
                    if once {
                        write!(f, "{} = {}; ", key, value)?;
                    } else {
                        once = true;
                    }
                    value.fmt(f)?;
                }
                write!(f, ")")
            },
            Value::Empty => write!(f, "()"),
        }
    }
}
