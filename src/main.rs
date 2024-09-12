use crate::domain::{ScannerInput, Stats};
use engine::{
    evaluator::{Evaluator, SetBonusMap},
    optimizer::Optimizer,
};
use eyre::Result;
use std::{collections::HashMap, fs};
use tracing_subscriber;

mod domain;
mod engine;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();
    
    let input = load_input_data("scanned_data/HSRScanData_20231126_122032.json").await?;
    let relic_pool = build_relic_pool(&input);
    let evaluator = create_evaluator(&input).await?;
    
    let optimizer = Optimizer {
        relic_pool,
        generation: 10000,
        population_size: 20,
        mutation_rate: 0.01,
        crossover_rate: 0.7,
        evaluator,
    };

    println!("----------------- Optimizing Fu Xuan's HP -----------------");
    let fuxuan = input
        .characters
        .iter()
        .find(|c| c.key == domain::CharacterName::FuXuan)
        .ok_or(eyre::eyre!("Missing Fu Xuan"))?;
    println!("Current stats: {:#?}", fuxuan.stats_panel);

    let res = optimizer.optimize()?;
    println!("Optimized relics: {:?}", res);

    Ok(())
}

/// Sets up logging configuration.
fn setup_logging() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO) // Adjust logging level as needed
        .init();
}

/// Loads and deserializes the input JSON data.
async fn load_input_data(file_path: &str) -> Result<ScannerInput> {
    let file = fs::File::open(file_path)?;
    let json: serde_json::Value = serde_json::from_reader(file)?;
    let mut input: ScannerInput = serde_json::from_value(json)?;
    input.update().await?;
    Ok(input)
}

/// Constructs a relic pool from the input data.
fn build_relic_pool(input: &ScannerInput) -> HashMap<domain::Slot, Vec<domain::Relic>> {
    let mut relic_pool = HashMap::new();
    for relic in &input.relics {
        relic_pool
            .entry(relic.slot.clone())
            .or_insert_with(Vec::new)
            .push(relic.clone());
    }
    relic_pool
}

/// Creates an evaluator instance using the input data.
async fn create_evaluator(input: &ScannerInput) -> Result<Evaluator> {
    let fuxuan = input
        .characters
        .iter()
        .find(|c| c.key == domain::CharacterName::FuXuan)
        .ok_or(eyre::eyre!("Missing Fu Xuan"))?;
    let light_cone = fuxuan.light_cone.clone().unwrap();
    let yaml_content = fs::read_to_string("src/config/set_bonus.yaml")?;
    let set_bonus: SetBonusMap = serde_yaml::from_str(&yaml_content)?;

    let hp_formula = "(Character_HP + LightCone_HP) * (1 + Percentage_HP_Bonus / 100) + Additive_HP_Bonus";
    let atk_formula = "(Character_ATK + LightCone_ATK) * (1 + Percentage_ATK_Bonus / 100) + Additive_ATK_Bonus";
    let def_formula = "(Character_DEF + LightCone_DEF) * (1 + (Percentage_DEF_Bonus - Percentage_DEF_Reduction) / 100) + Additive_DEF_Bonus";
    let spd_formula = "Character_SPD * (1 + Percentage_SPD_Bonus / 100) + Additive_SPD_Bonus";
    let crit_rate_formula = "Character_Base_CRIT_Rate + CRIT_Rate";
    let crit_dmg_formula = "Character_Base_CRIT_DMG + CRIT_DMG";
    let energy_regen_rate_formula = "Energy_Regeneration_Rate";
    let effect_hit_rate_formula = "Effect_Hit_Rate";
    let effect_res_formula = "Effect_RES";
    let break_effect_formula = "Break_Effect";

    Ok(Evaluator::new(
        fuxuan.clone(),
        light_cone,
        HashMap::new(),
        set_bonus,
        fuxuan.traces.total_bonus.clone(),
        HashMap::from([
            (Stats::Hp, hp_formula.to_owned()),
            (Stats::Atk, atk_formula.to_owned()),
            (Stats::Def, def_formula.to_owned()),
            (Stats::Spd, spd_formula.to_owned()),
            (Stats::CritRate_, crit_rate_formula.to_owned()),
            (Stats::CritDmg_, crit_dmg_formula.to_owned()),
            (Stats::EnergyRegenerationRate_, energy_regen_rate_formula.to_owned()),
            (Stats::EffectHitRate_, effect_hit_rate_formula.to_owned()),
            (Stats::EffectRes_, effect_res_formula.to_owned()),
            (Stats::BreakEffect_, break_effect_formula.to_owned()),
        ]),
        hp_formula,
        "Maximum HP",
    ))
}
