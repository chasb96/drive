use crate::env::Env;
use rusoto_core::credential::AwsCredentials;
use rusoto_core::credential::StaticProvider;

pub struct Credentials {
    id: String,
    secret: String,
}

impl Credentials {
    pub fn env() -> Self {
        Self {
            id: Env::aws_access_key_id(),
            secret: Env::aws_access_key_secret(),
        }
    }
}

impl Default for Credentials {
    fn default() -> Self {
        Self::env()
    }
}

impl Into<AwsCredentials> for Credentials {
    fn into(self) -> AwsCredentials {
        AwsCredentials::new(self.id, self.secret, None, None)
    }
}

impl Into<StaticProvider> for Credentials {
    fn into(self) -> StaticProvider {
        StaticProvider::new_minimal(self.id, self.secret)
    }
}
