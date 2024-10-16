use crate::domain::{ScannerInput, Stats};
use client::project_yatta_client::ProjectYattaClient;
use data_fetcher::{project_yatta_data_fetcher::ProjectYattaDataFetcher, DataFetcher};
use engine::{
    evaluator::{Evaluator, SetBonusMap},
    optimizer::Optimizer,
    simulated_annealing::SimulatedAnnealing,
};
use eyre::Result;
use std::{collections::HashMap, fs};

mod client;
mod data_fetcher;
mod domain;
mod engine;
mod service;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();

    let input = load_input_data("scanned_data/HSRScanData_20241014_152542.json").await?;
    let relic_pool = build_relic_pool(&input);
    let evaluator = create_evaluator(&input).await?;

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
        generation: 30,
        population_size: 1000,
        mutation_rate: 0.1,
        crossover_rate: 0.7,
        evaluator,
        enable_sa: false,
        simulated_annealing,
    };

    println!("----------------- Optimizing Fu Xuan's HP -----------------");
    // let fuxuan = input
    //     .characters
    //     .iter()
    //     .find(|c| c.name == "Fu Xuan")
    //     .ok_or(eyre::eyre!("Missing Fu Xuan"))?;

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
        .find(|c| c.name == "Fu Xuan")
        .ok_or(eyre::eyre!("Missing Fu Xuan"))?;
    // let fetcher = HoyowikiDataFetcherService {
    //     client: HoyowikiClient {
    //         base_url: "https://sg-wiki-api-static.hoyolab.com/hoyowiki/hsr/wapi".to_string(),
    //         language: "en-us".to_string(),
    //         wiki_app: "hsr".to_string(),
    //     },
    // };
    let fetcher = ProjectYattaDataFetcher {
        client: ProjectYattaClient {
            url: "https://sr.yatta.moe/api/v2/en/".to_string(),
        },
    };
    let light_cone = input
        .light_cones
        .iter()
        .find(|l| l.location == Some("1208".to_owned()))
        .ok_or_else(|| eyre::eyre!("Missing Fu Xuan's light cone"))?;
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

    let fuxuan = fetcher.fetch_character_data(fuxuan).await?;
    let light_cone = fetcher.fetch_light_cone_data(light_cone).await?;

    println!("Character: {:#?}", fuxuan);
    println!("Light cone: {:#?}", light_cone);

    Ok(Evaluator::new(
        fuxuan.clone(),
        light_cone.clone(),
        HashMap::new(),
        set_bonus,
        fuxuan.stat_bonus.clone(),
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
        hp_formula,
        "Maximum HP",
    ))
}
