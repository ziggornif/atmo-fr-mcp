use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub atmo_username: String,
    pub atmo_password: String,
}
