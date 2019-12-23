use crate::env::Env;
use rusoto_core::region::Region as RusotoRegion;
use std::str::FromStr;

pub struct Region(String);

impl Region {
    pub fn env() -> Self {
        Region(Env::aws_bucket_region())
    }
}

impl Into<RusotoRegion> for Region {
    fn into(self) -> RusotoRegion {
        RusotoRegion::from_str(&self.0).unwrap()
    }
}
