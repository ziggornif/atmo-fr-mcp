use atmo_fr_mcp::{
    adapters::{
        atmo_api::{get_atmo_bearer, get_qualite_air},
        geo_api::get_city_codes,
    },
    model::geo::CityInfos,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    atmo_username: String,
    atmo_password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = envy::from_env::<Config>().unwrap();
    let date = "2025-08-28";
    let city = CityInfos {
        name: "Vannes".to_owned(),
        code_postal: "56000".to_owned(),
    };
    let city_codes = get_city_codes(&city.name, &city.code_postal).await?;
    let token = get_atmo_bearer(config.atmo_username, config.atmo_password).await?;

    let resp = match get_qualite_air(date, &city_codes.code_insee, &token).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Erreur avec code_insee: {e}. Tentative avec code_epci...");
            get_qualite_air(date, &city_codes.code_epci, &token).await?
        }
    };

    println!(
        "Qualité de l'air à {} : {} {}",
        resp.features[0].properties.lib_zone,
        resp.features[0].properties.code_qual,
        resp.features[0].properties.lib_qual
    );
    Ok(())
}
