use serde::{Deserialize, Serialize};

pub struct CityInfos {
    pub name: String,
    pub code_postal: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CityCodes {
    pub nom: String,
    #[serde(alias = "code")]
    pub code_insee: String,
    #[serde(alias = "codeEpci")]
    pub code_epci: String,
}
