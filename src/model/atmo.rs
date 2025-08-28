use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualiteAirReponse {
    pub r#type: String,
    pub name: String,
    pub crs: Crs,
    pub features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crs {
    pub r#type: String,
    pub properties: CrsProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrsProperties {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    pub r#type: String,
    pub properties: FeatureProperties,
    pub geometry: Geometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureProperties {
    pub aasqa: String,
    pub date_maj: String,
    pub code_no2: i32,
    pub code_o3: i32,
    pub code_pm10: i32,
    pub code_pm25: i32,
    pub code_qual: i32,
    pub code_so2: i32,
    pub code_zone: String,
    pub coul_qual: String,
    pub date_dif: String,
    pub date_ech: String,
    pub epsg_reg: String,
    pub lib_qual: String,
    pub lib_zone: String,
    pub source: String,
    pub type_zone: String,
    pub x_reg: f64,
    pub x_wgs84: f64,
    pub y_reg: f64,
    pub y_wgs84: f64,
    pub gml_id2: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Vec<f64>,
}
