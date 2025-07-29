#[derive(Clone)]
pub struct Enemy {
    pub level: u8,
    pub resistance: f64,
    pub def_bonus: f64,
    pub dmg_mitigation: Vec<f64>,
    pub vulnerability: f64,
    pub toughness_break: bool,
    pub weaken: f64,
}
