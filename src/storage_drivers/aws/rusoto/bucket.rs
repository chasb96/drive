use crate::env::Env;

pub struct Bucket(String);

impl Bucket {
    pub fn env() -> Bucket {
        Bucket(Env::aws_bucket_name())
    }
}

impl Into<String> for Bucket {
    fn into(self) -> String {
        self.0
    }
}
