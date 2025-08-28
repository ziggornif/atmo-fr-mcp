use crate::model::geo::CityCodes;

pub async fn get_city_codes(name: &str, code_postal: &str) -> Result<CityCodes, anyhow::Error> {
    let url = format!(
        "https://geo.api.gouv.fr/communes?codePostal={}&fields=code,nom,codeEpci&format=json&geometry=centre",
        code_postal
    );
    let resp = reqwest::get(url).await?.json::<Vec<CityCodes>>().await?;
    match resp.iter().find(|&city| city.nom == name) {
        Some(city) => Ok(city.clone()),
        None => Err(anyhow::anyhow!(
            "No codes for city {} {}",
            name,
            code_postal
        )),
    }
}
