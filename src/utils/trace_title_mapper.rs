use crate::domain::Stats;

pub fn title_mapper(title: &str) -> Stats {
    match title {
        "HP Boost" => Stats::Hp_,
        "ATK Boost" => Stats::Atk_,
        "DEF Boost" => Stats::Def_,
        "SPD Boost" => Stats::Spd_,
        "CRIT Rate Boost" => Stats::CritRate_,
        "CRIT DMG Boost" => Stats::CritDmg_,
        "Effect RES Boost" => Stats::EffectRes_,
        "Beak Effect Boost" => Stats::BreakEffect_,
        "Energy Regeneration Boost" => Stats::EnergyRegenerationRate_,
        "Effect Hit Rate Boost" => Stats::EffectHitRate_,
        _ => todo!(),
    }
}
