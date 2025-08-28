use crate::model::atmo::{LoginForm, LoginResponse, QualiteAirReponse};

pub async fn get_atmo_bearer(username: String, password: String) -> Result<String, anyhow::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://admindata.atmo-france.org/api/login")
        .json(&LoginForm { username, password })
        .send()
        .await?
        .json::<LoginResponse>()
        .await?;

    Ok(res.token)
}

pub async fn get_qualite_air(
    date: &str,
    code: &str,
    token: &str,
) -> Result<QualiteAirReponse, anyhow::Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://admindata.atmo-france.org/api/v2/data/indices/atmo?format=geojson&date={}&code_zone={}",
        date, code
    );
    let resp = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<QualiteAirReponse>()
        .await?;
    if resp.features.is_empty() {
        Err(anyhow::anyhow!("Empty response with code {}", code))
    } else {
        Ok(resp)
    }
}
