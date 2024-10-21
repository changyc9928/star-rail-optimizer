use crate::domain::{ScannerInput, Stats};
use client::project_yatta_client::ProjectYattaClient;
use data_fetcher::project_yatta_data_fetcher::ProjectYattaDataFetcher;
use domain::{CharacterEntity, LightConeEntity};
use engine::{
    evaluator::{Evaluator, SetBonusMap},
    optimizer::Optimizer,
    simulated_annealing::SimulatedAnnealing,
};
use eyre::{eyre, Result};
use service::scanner_parser_service::ScannerParserService;
use std::{collections::HashMap, fs, sync::Arc};
use tokio::sync::Mutex;

mod client;
mod data_fetcher;
mod domain;
mod engine;
mod service;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();

    // let data_fetcher = Arc::new(HoyowikiDataFetcherService {
    //     client: HoyowikiClient {
    //         base_url: "https://sg-wiki-api-static.hoyolab.com/hoyowiki/hsr/wapi".to_string(),
    //         language: "en-us".to_string(),
    //         wiki_app: "hsr".to_string(),
    //     },
    // });
    let data_fetcher = Arc::new(Mutex::new(ProjectYattaDataFetcher {
        client: ProjectYattaClient {
            url: "https://sr.yatta.moe/api/v2/en/".to_string(),
            light_cone_cache: HashMap::new(),
        },
    }));
    let mut scanner_parser_service = ScannerParserService { data_fetcher };
    let input = load_input_data("scanned_data/HSRScanData_20241014_152542.json").await?;
    let (characters, light_cones, relic_pool) =
        scanner_parser_service.parse_scanner_input(&input).await?;
    let evaluator = create_evaluator(
        characters
            .get("1308")
            .ok_or_else(|| eyre!("Acheron not found"))?,
        light_cones
            .get("light_cone_10")
            .ok_or_else(|| eyre!("Acheron's light cone not found"))?,
    )
    .await?;

    let simulated_annealing = SimulatedAnnealing {
        initial_temp: 1000.0,
        cooling_rate: 0.99,
        min_temp: 0.1,
        aggresive_factor: 0.9,
        relic_pool: relic_pool.clone(),
        evaluator: evaluator.clone(),
    };

    let optimizer = Optimizer {
        relic_pool,
        generation: 100,
        population_size: 1000,
        mutation_rate: 0.1,
        crossover_rate: 0.7,
        evaluator,
        enable_sa: false,
        simulated_annealing,
    };

    println!("----------------- Optimizing Character -----------------");

    let res = optimizer.optimize()?;
    println!("Optimized relics: {:#?}", res);

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

/// Creates an evaluator instance using the input data.
async fn create_evaluator(
    character: &CharacterEntity,
    light_cone: &LightConeEntity,
) -> Result<Evaluator> {
    let yaml_content = fs::read_to_string("src/config/set_bonus.yaml")?;
    let set_bonus: SetBonusMap = serde_yaml::from_str(&yaml_content)?;

    let hp_formula =
        "(Character_HP + LightCone_HP) * (1 + Percentage_HP_Bonus / 100) + Additive_HP_Bonus";
    let atk_formula =
        "(Character_ATK + LightCone_ATK) * (1 + Percentage_ATK_Bonus / 100) + Additive_ATK_Bonus";
    let def_formula = "(Character_DEF + LightCone_DEF) * (1 + (Percentage_DEF_Bonus - Percentage_DEF_Reduction) / 100) + Additive_DEF_Bonus";
    let spd_formula = "Character_SPD * (1 + Percentage_SPD_Bonus / 100) + Additive_SPD_Bonus";
    let crit_rate_formula = "Character_Base_CRIT_Rate + CRIT_Rate";
    let crit_dmg_formula = "Character_Base_CRIT_DMG + CRIT_DMG";
    let energy_regen_rate_formula = "Energy_Regeneration_Rate";
    let effect_hit_rate_formula = "Effect_Hit_Rate";
    let effect_res_formula = "Effect_RES";
    let break_effect_formula = "Break_Effect";

    let enemy_level = "80";
    let enemy_resistance = "20";
    let acheron_ult_resistance_reduction = "19";
    let avg_crit_formula = format!("({crit_rate_formula}) / 100 * ({crit_dmg_formula}) / 100");
    let total_dmg_boost = "(Lightning_DMG_Boost + Common_DMG_Boost + Ultimate_DMG_Boost) / 100";
    let def_multiplier = format!("(Level + 20) / (({enemy_level} + 20) * (1 - (DMG_Reduction - DEF_Ignore) / 100) + Level + 20)");
    let resistance_multipler =
        format!("1 - ({enemy_resistance} / 100 - ({acheron_ult_resistance_reduction}) / 100)");
    let toughness_break = false;
    let toughness = if toughness_break { "1" } else { "0.9" };
    let independent_multiplier = "1.15"; // With only one nihility teammate

    let acheron_ultimate_final_dmg = format!(
        "((1.14 * 1.9 + 6 * 0.25) * ({atk_formula})) \
        * (1 + ({avg_crit_formula})) \
        * (1 + ({total_dmg_boost})) \
        * ({def_multiplier}) \
        * ({resistance_multipler}) \
        * ({toughness}) \
        * ({independent_multiplier})"
    );
    // let acheron_ultimate_final_dmg_with_sparkle = "((1.14 * 1.9 + 6 * 0.25) * ((Character_ATK + LightCone_ATK) * (1 + (Percentage_ATK_Bonus + 15) / 100) + Additive_ATK_Bonus)) * (1 + (Character_Base_CRIT_Rate + CRIT_Rate) / 100 * (Character_Base_CRIT_DMG + CRIT_DMG + 79.115) / 100) * (1 + Lightning_DMG_Boost / 100 + Common_DMG_Boost / 100 + Ultimate_DMG_Boost / 100 + 0.453) * ((Level + 20) / ((80 + 20) * (1 - DMG_Reduction / 100 - DEF_Ignore / 100) + Level + 20)) * (1 - (20 / 100 - 20 / 100)) * 0.9 * 1.6";
    let mut other_bonus = HashMap::from([
        (Stats::Atk_, 15.0),
        (Stats::CritDmg_, 79.115),
        (Stats::DmgBoost_, 45.3),
    ]); // Assuming Sparkle's support
    for (key, val) in &character.stat_bonus {
        *other_bonus.entry(key.clone()).or_default() += val;
    }

    Ok(Evaluator::new(
        character.clone(),
        light_cone.clone(),
        HashMap::new(),
        set_bonus,
        other_bonus,
        HashMap::from([
            (Stats::Hp, hp_formula.to_owned()),
            (Stats::Atk, atk_formula.to_owned()),
            (Stats::Def, def_formula.to_owned()),
            (Stats::Spd, spd_formula.to_owned()),
            (Stats::CritRate_, crit_rate_formula.to_owned()),
            (Stats::CritDmg_, crit_dmg_formula.to_owned()),
            (
                Stats::EnergyRegenerationRate_,
                energy_regen_rate_formula.to_owned(),
            ),
            (Stats::EffectHitRate_, effect_hit_rate_formula.to_owned()),
            (Stats::EffectRes_, effect_res_formula.to_owned()),
            (Stats::BreakEffect_, break_effect_formula.to_owned()),
        ]),
        &acheron_ultimate_final_dmg,
        "AVG Ultimate AoE DMG",
    ))
}
