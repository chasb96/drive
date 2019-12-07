mod file;
mod folder;
mod user;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Error};

type ToJsonError = Error;

pub trait ToJson: Serialize {
    fn to_json(&self) -> Result<String, ToJsonError> {
        to_string(self)
    }
}

#[allow(dead_code)]
type FromJsonError = Error;

pub trait FromJson<'a>: Deserialize<'a> {
    fn from_json(payload: &'a str) -> Result<Self, FromJsonError> {
        from_str::<'a>(payload)
    }
}

impl<T: ToJson> ToJson for Vec<T> {}
impl<'a, T: FromJson<'a>> FromJson<'a> for Vec<T> {}
