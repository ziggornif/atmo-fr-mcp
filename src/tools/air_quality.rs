use chrono::Local;
use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};

use crate::{
    adapters::{
        atmo_api::{get_atmo_bearer, get_qualite_air},
        geo_api::get_city_codes,
    },
    config::config::Config,
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AirQualityRequest {
    #[schemars(description = "le nom de la ville")]
    pub ville: String,
    #[schemars(description = "le code postal de la ville")]
    pub code_postal: String,
}

#[derive(Debug, Clone)]
pub struct AirQuality {
    config: Config,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl AirQuality {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Récupérer la qualité de l'air d'une ville")]
    async fn get_air_quality(
        &self,
        Parameters(AirQualityRequest { ville, code_postal }): Parameters<AirQualityRequest>,
    ) -> String {
        let city_codes = get_city_codes(&ville, &code_postal).await.unwrap();
        let token = get_atmo_bearer(&self.config.atmo_username, &self.config.atmo_password)
            .await
            .unwrap_or_default();
        let date = Local::now().format("%Y-%m-%d").to_string();
        let resp = match get_qualite_air(&date, &city_codes.code_insee, &token).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Erreur avec code_insee: {e}. Tentative avec code_epci...");
                match get_qualite_air(&date, &city_codes.code_epci, &token).await {
                    Ok(r) => r,
                    Err(e2) => {
                        return format!(
                            "Impossible de récupérer les données de qualité de l'air pour {} ({}).\n\
                            Erreur avec code INSEE: {}\n\
                            Erreur avec code EPCI: {}",
                            ville, code_postal, e, e2
                        );
                    }
                }
            }
        };

        if resp.features.is_empty() {
            return format!(
                "Aucune donnée de qualité de l'air trouvée pour {} ({})",
                ville, code_postal
            );
        }

        let feature = &resp.features[0];
        let props = &feature.properties;

        format!(
            "🌍 Qualité de l'air pour {} ({})\n\
            📅 Date: {} (mise à jour: {})\n\
            🔍 Zone: {} ({})\n\
            \n\
            📊 Indice de qualité global: {} - {}\n\
            (0: Absent, 1: Bon, 2: Moyen, 3: Dégradé, 4: Mauvais, 5: Très mauvais, 6: Extrêmement mauvais, 7: Evénement)\n\
            \n\
            💨 Détail des polluants:\n\
            • NO₂ (dioxyde d'azote): {}\n\
            • O₃ (ozone): {}\n\
            • PM10 (particules fines): {}\n\
            • PM2.5 (particules très fines): {}\n\
            • SO₂ (dioxyde de soufre): {}\n\
            \n\
            📍 Coordonnées: {:.4}, {:.4}\n\
            🏢 Organisme: {} ({})",
            ville,
            code_postal,
            props.date_ech,
            props.date_maj,
            props.lib_zone,
            props.code_zone,
            props.code_qual,
            props.lib_qual,
            props.code_no2,
            props.code_o3,
            props.code_pm10,
            props.code_pm25,
            props.code_so2,
            props.x_wgs84,
            props.y_wgs84,
            props.aasqa,
            props.source
        )
    }
}

#[tool_handler]
impl ServerHandler for AirQuality {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Une intégration de Atmo France pour récupérer la qualité de l'air".into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
