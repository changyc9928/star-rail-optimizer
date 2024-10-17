use crate::domain::Stats;

pub fn title_mapper(title: &str) -> Stats {
    let title: String = title
        .chars()
        .map(|c| if c == '\u{00A0}' { ' ' } else { c })
        .collect();
    match title.as_str() {
        "HP Boost" => Stats::Hp_,
        "ATK Boost" => Stats::Atk_,
        "DEF Boost" => Stats::Def_,
        "SPD Boost" => Stats::Spd_,
        "CRIT Rate Boost" => Stats::CritRate_,
        "CRIT DMG Boost" => Stats::CritDmg_,
        "Effect RES Boost" => Stats::EffectRes_,
        "Break Boost" | "Break Enhance" => Stats::BreakEffect_,
        "Energy Regeneration Boost" => Stats::EnergyRegenerationRate_,
        "Effect Hit Rate Boost" => Stats::EffectHitRate_,
        "DMG Boost" => Stats::DmgBoost_,
        "DMG Boost: Ice" => Stats::IceDmgBoost_,
        "DMG Boost: Fire" | "DMG Boost Fire" => Stats::FireDmgBoost_,
        "DMG Boost: Wind" => Stats::WindDmgBoost_,
        "DMG Boost: Lightning" => Stats::LightningDmgBoost_,
        "DMG Boost: Imaginary" => Stats::ImaginaryDmgBoost_,
        "DMG Boost: Quantum" => Stats::QuantumDmgBoost_,
        "DMG Boost: Physical" => Stats::PhysicalDmgBoost_,
        _ => todo!(),
    }
}
