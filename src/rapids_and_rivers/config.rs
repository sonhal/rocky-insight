use std::env;

#[derive(Debug)]
pub struct Config {
    pub bootstrap_servers: String,
    pub topic: String,
    pub ssl_ca_location	: Option<String>,
    pub ssl_certificate_location: Option<String>,
    pub ssl_key_location: Option<String>,
    pub ssl_key_password: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            bootstrap_servers: env::var("BOOTSTRAP_SERVERS")
                .expect("missing config BOOTSTRAP_SERVERS"),
            topic: env::var("TOPIC").expect("missing config TOPIC"),
            ssl_ca_location: env::var("SSL_CA_LOCATION")
                .map_or_else(|_| None, |s| Some(s)),
            ssl_certificate_location: env::var("SSL_CERTIFICATE_LOCATION")
                .map_or_else(|_| None, |s| Some(s)),
            ssl_key_location: env::var("SSL_KEY_LOCATION")
                .map_or_else(|_| None, |s| Some(s)),
            ssl_key_password: env::var("SSL_KEY_PASSWORD")
                .map_or_else(|_| None, |s| Some(s)),
        }
    }

    pub fn is_ssl(&self) -> bool {
        self.ssl_key_location.is_some()
    }
}
