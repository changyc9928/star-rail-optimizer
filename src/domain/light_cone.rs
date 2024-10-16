use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct LightConeEntity {
    pub base_hp: f64,
    pub base_atk: f64,
    pub base_def: f64,
    pub _light_cone: LightCone,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightCone {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub ascension: u8,
    pub superimposition: u8,
    pub location: Option<String>,
    pub lock: bool,
    pub _uid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LightConeStats {
    pub hp: f64,
    pub atk: f64,
    pub def: f64,
}
