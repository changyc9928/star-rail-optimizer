#[derive(Clone)]
pub struct Enemy {
    pub level: u8,
    pub resistance: f64,
    pub dmg_mitigation: Vec<f64>,
}
