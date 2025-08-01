#[derive(Clone, Debug, Default)]
pub struct BaseStats {
    pub hp: f64,
    pub hp_percentage: f64,
    pub atk: f64,
    pub atk_percentage: f64,
    pub def: f64,
    pub def_percentage: f64,
    pub spd: f64,
    pub spd_percentage: f64,
    pub crit_rate: f64,
    pub crit_damage: f64,
    pub energy_regeneration_rate: f64,
    pub break_effect: f64,
    pub effect_resistance: f64,
    pub effect_hit_rate: f64,
    pub ougoing_healing_boost: f64,

    pub lightning_damage_boost: f64,
    pub ice_damage_boost: f64,
    pub fire_damage_boost: f64,
    pub wind_damage_boost: f64,
    pub physical_damage_boost: f64,
    pub quantum_damage_boost: f64,
    pub imaginary_damage_boost: f64,

    pub lightning_resistance: f64,
    pub ice_resistance: f64,
    pub fire_resistance: f64,
    pub wind_resistance: f64,
    pub physical_resistance: f64,
    pub quantum_resistance: f64,
    pub imaginary_resistance: f64,
}
