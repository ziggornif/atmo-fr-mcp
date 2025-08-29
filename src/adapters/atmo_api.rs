use crate::model::atmo::{LoginForm, LoginResponse, QualiteAirReponse};

pub async fn get_atmo_bearer(username: &str, password: &str) -> Result<String, anyhow::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://admindata.atmo-france.org/api/login")
        .json(&LoginForm {
            username: username.to_string(),
            password: password.to_string(),
        })
        .send()
        .await?
        .json::<LoginResponse>()
        .await?;

    Ok(res.token)
}

async fn call_atmo(
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
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "❌ Impossible de récupérer les données de qualité de l'air. Erreur : {}",
                e
            )
        })?;

    Ok(resp)
}

pub async fn get_qualite_air(
    date: &str,
    code_insee: &str,
    code_epci: &str,
    token: &str,
) -> Result<QualiteAirReponse, anyhow::Error> {
    let insee_resp = call_atmo(date, code_insee, token).await?;
    if !insee_resp.features.is_empty() {
        return Ok(insee_resp);
    }

    // fallback on bascule sur la recherche par code EPCI
    eprintln!("Erreur avec code_insee: {code_insee}. Tentative avec code_epci...");

    let epci_resp = call_atmo(date, code_epci, token).await?;
    Ok(epci_resp)
}
