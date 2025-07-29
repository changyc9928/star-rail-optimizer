use crate::{
    character::AcheronEvaluationTarget,
    domain::{RelicSetConfig, ScannerInput},
};
use character::{Acheron, Evaluator};
use client::project_yatta_client::ProjectYattaClient;
use data_fetcher::project_yatta_data_fetcher::ProjectYattaDataFetcher;
use domain::Enemy;
use engine::{optimizer::Optimizer, simulated_annealing::SimulatedAnnealing};
use eyre::{eyre, Result};
use service::scanner_parser_service::ScannerParserService;
use std::{collections::HashMap, fs, sync::Arc};
use tokio::sync::Mutex;

mod character;
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
    // let evaluator = create_evaluator(
    //     characters
    //         .get("1308")
    //         .ok_or_else(|| eyre!("Acheron not found"))?,
    //     light_cones
    //         .get("light_cone_10")
    //         .ok_or_else(|| eyre!("Acheron's light cone not found"))?,
    // )
    // .await?;
    let a_evaluator: Arc<dyn Evaluator<Target = AcheronEvaluationTarget> + Send + Sync> =
        Arc::new(Acheron {
            character: characters
                .get("1308")
                .ok_or_else(|| eyre!("Acheron not found"))?
                .clone(),
            light_cone: Some(
                light_cones
                    .get("light_cone_10")
                    .ok_or_else(|| eyre!("Acheron's light cone not found"))?
                    .clone(),
            ),
            crimson_knot: 9,
            crit: domain::CritEnum::Avg,
            thunder_core_bonus_stack: 3,
            activate_eidolon_1: false,
        });
    let enemy = Enemy {
        level: 80,
        resistance: 0.0,
        dmg_mitigation: vec![],
        def_bonus: 0.0,
        vulnerability: 0.0,
        toughness_break: false,
        weaken: 0.0,
    };

    // let battle_conditions = vec![
    //     BattleConditionEnum::AfterUsingSkill {
    //         number_of_turns_since_using_the_skill: 1,
    //     },
    //     BattleConditionEnum::AfterUsingUltimate {
    //         next_attack_after_ultimate: false,
    //         next_skill_after_ultimate: false,
    //         number_of_turns_since_using_ultimate: 1,
    //     },
    //     BattleConditionEnum::AfterWearerAttack { number_of_times: 3 },
    //     BattleConditionEnum::AfterWearerIsHit {
    //         number_of_times: 2,
    //         within_number_of_turns: 1,
    //     },
    //     BattleConditionEnum::AfterAttackingDebuffedEnemy,
    //     BattleConditionEnum::AfterWearerInflictingDebuffs {
    //         number_of_times: 1,
    //         within_number_of_turns: 1,
    //     },
    //     BattleConditionEnum::WhenAttackingEnemyWithDebuff {
    //         number_of_debuffs_enemy_has: 3,
    //         within_number_of_turns: 1,
    //     },
    //     BattleConditionEnum::TeammatesSamePathWithWearer {
    //         number_of_teammates_having_same_path: 1,
    //     },
    //     BattleConditionEnum::HittingEnemyWithCrimsonKnot {
    //         number_of_crinsom_knot_enemy_has: 3,
    //     },
    //     BattleConditionEnum::CriticalHit(CritEnum::Avg),
    //     BattleConditionEnum::ToughnessBreak(true),
    //     BattleConditionEnum::AfterHittingEnemyWithCrinsomKnot { number_of_times: 3 },
    // ];

    let simulated_annealing = SimulatedAnnealing {
        initial_temp: 1000.0,
        cooling_rate: 0.99,
        min_temp: 0.1,
        aggresive_factor: 0.9,
        relic_pool: relic_pool.clone(),
        evaluator: a_evaluator.clone(),
        teammates: vec![],
        enemy: enemy.clone(),
        target: AcheronEvaluationTarget::UltimateAoe,
    };

    let optimizer = Optimizer {
        relic_pool,
        generation: 25,
        population_size: 1000,
        mutation_rate: 0.1,
        crossover_rate: 0.7,
        evaluator: a_evaluator,
        enable_sa: false,
        simulated_annealing,
        enemy,
        target: AcheronEvaluationTarget::UltimateAoe,
        teammates: vec![],
        relic_set_config: RelicSetConfig {
            activate_102: true,
            activate_104: true,
            stack_105: 5,
            activate_107: true,
            activate_108: true,
            activate_109: true,
            activate_112_1: true,
            activate_112_2: true,
            stack_113: 5,
            stack_115: 5,
            stack_116: 5,
            activate_117_2pcs: true,
            stack_117: 5,
            activate_117_4pcs_extra: true,
            activate_120: true,
            activate_122: true,
            activate_305: true,
            stack_313: 5,
            stack_315: 5,
            activate_316: true,
            activate_318: true,
        },
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

// / Creates an evaluator instance using the input data.
// async fn create_evaluator(
//     character: &CharacterEntity,
//     light_cone: &LightConeEntity,
// ) -> Result<Evaluator> {
//     let yaml_content = fs::read_to_string("src/config/set_bonus.yaml")?;
//     let mut set_bonus: SetBonusMap = serde_yaml::from_str(&yaml_content)?;

//     let hp_formula =
//         "(Character_HP + LightCone_HP) * (1 + Percentage_HP_Bonus / 100) + Additive_HP_Bonus";
//     let atk_formula =
//         "(Character_ATK + LightCone_ATK) * (1 + Percentage_ATK_Bonus / 100) + Additive_ATK_Bonus";
//     let def_formula = "(Character_DEF + LightCone_DEF) * (1 + (Percentage_DEF_Bonus - Percentage_DEF_Reduction) / 100) + Additive_DEF_Bonus";
//     let spd_formula = "Character_SPD * (1 + Percentage_SPD_Bonus / 100) + Additive_SPD_Bonus";
//     let crit_rate_formula = "Character_Base_CRIT_Rate + CRIT_Rate";
//     let crit_dmg_formula = "Character_Base_CRIT_DMG + CRIT_DMG";
//     let energy_regen_rate_formula = "Energy_Regeneration_Rate";
//     let effect_hit_rate_formula = "Effect_Hit_Rate";
//     let effect_res_formula = "Effect_RES";
//     let break_effect_formula = "Break_Effect";

//     let enemy_level = "80";
//     let enemy_resistance = "20";
//     let acheron_ult_resistance_reduction = "19";
//     let avg_crit_formula = format!("({crit_rate_formula}) / 100 * ({crit_dmg_formula}) / 100");
//     let total_dmg_boost = "(Lightning_DMG_Boost + Common_DMG_Boost + Ultimate_DMG_Boost) / 100";
//     let def_multiplier = format!("(Level + 20) / (({enemy_level} + 20) * (1 - (DMG_Reduction - DEF_Ignore) / 100) + Level + 20)");
//     let resistance_multipler =
//         format!("1 - ({enemy_resistance} / 100 - ({acheron_ult_resistance_reduction}) / 100)");
//     let weakness_break = false;
//     let toughness = if weakness_break { "1" } else { "0.9" };
//     let independent_multiplier = "1.15"; // With only one nihility teammate

//     let acheron_ultimate_final_dmg = format!(
//         "((1.14 * 1.9 + 6 * 0.25) * ({atk_formula})) \
//         * (1 + ({avg_crit_formula})) \
//         * (1 + ({total_dmg_boost})) \
//         * ({def_multiplier}) \
//         * ({resistance_multipler}) \
//         * ({toughness}) \
//         * ({independent_multiplier})"
//     );
//     // let acheron_ultimate_final_dmg_with_sparkle = "((1.14 * 1.9 + 6 * 0.25) * ((Character_ATK + LightCone_ATK) * (1 + (Percentage_ATK_Bonus + 15) / 100) + Additive_ATK_Bonus)) * (1 + (Character_Base_CRIT_Rate + CRIT_Rate) / 100 * (Character_Base_CRIT_DMG + CRIT_DMG + 79.115) / 100) * (1 + Lightning_DMG_Boost / 100 + Common_DMG_Boost / 100 + Ultimate_DMG_Boost / 100 + 0.453) * ((Level + 20) / ((80 + 20) * (1 - DMG_Reduction / 100 - DEF_Ignore / 100) + Level + 20)) * (1 - (20 / 100 - 20 / 100)) * 0.9 * 1.6";
//     let mut other_bonus = HashMap::from([
//         (Stats::Atk_, 15.0),
//         (Stats::CritDmg_, 79.115),
//         (Stats::DmgBoost_, 45.3),
//     ]); // Assuming Sparkle's support

//     let activated_set_bonus = HashMap::from([
//         (
//             "104",
//             HashMap::from([(4, vec![(Stats::CritDmg_, 25.0, None::<(Stats, f64)>)])]), // After the wearer uses their Ultimate, their CRIT DMG increases by 25% for 2 turn(s).
//         ),
//         (
//             "105",
//             HashMap::from([(4, vec![(Stats::Atk_, 25.0, None::<(Stats, f64)>)])]), // (5 stacks) After the wearer attacks or is hit, their ATK increases by 5% for the rest of the battle. This effect can stack up to 5 time(s).
//         ),
//         (
//             "108",
//             HashMap::from([(4, vec![(Stats::DefIgnore_, 10.0, None::<(Stats, f64)>)])]), // (Assuming no Quantum weakness) When the wearer deals DMG to the target enemy, ignores 10% DEF. If the target enemy has Quantum Weakness, the wearer additionally ignores 10% DEF.
//         ),
//         (
//             "109",
//             HashMap::from([(4, vec![(Stats::Atk_, 20.0, None::<(Stats, f64)>)])]), // (Assuming Ult right after Skill) When the wearer uses their Skill, increases the wearer's ATK by 20% for 1 turn(s).
//         ),
//         (
//             "112",
//             HashMap::from([(4, vec![(Stats::CritRate_, 10.0, None::<(Stats, f64)>)])]), // (Assuming no Imaginary teammate) When attacking debuffed enemies, the wearer's CRIT Rate increases by 10%, and their CRIT DMG increases by 20% against Imprisoned enemies.
//         ),
//         (
//             "117",
//             HashMap::from([
//                 (2, vec![(Stats::DmgBoost_, 12.0, None::<(Stats, f64)>)]), // (Considering Acheron's team comp, assuming enemies always get debuffs) Increases DMG dealt to enemies with debuffs by 12%.
//                 (4, vec![(Stats::CritDmg_, 24.0, None::<(Stats, f64)>)]), // (Assuming 3 stacks and Ult after Skill (inflicting Crimson Knot)) The wearer deals 8%/12% increased CRIT DMG to enemies with at least 2/3 debuffs. After the wearer inflicts a debuff on enemy targets, the aforementioned effects increase by 100%, lasting for 1 turn(s).
//             ]),
//         ),
//         (
//             "313",
//             HashMap::from([
//                 (2, vec![(Stats::CritRate_, 4.0, None::<(Stats, f64)>)]), // (Assuming only 1 enemy get defeated) When an enemy target gets defeated, the wearer's CRIT DMG increases by 4.00%, stacking up to 10 time(s).
//             ]),
//         ),
//         (
//             "314",
//             HashMap::from([
//                 (2, vec![(Stats::CritRate_, 12.0, None::<(Stats, f64)>)]), // (Considering Acheron's team comp, at least one nihility teammate will present) When entering battle, if at least one teammate follows the same Path as the wearer, then the wearer's CRIT Rate increases by 12.00%.
//             ]),
//         ),
//     ]);

//     for (key, val) in activated_set_bonus {
//         for (num_items, mut bonus) in val {
//             set_bonus
//                 .entry(key.to_owned())
//                 .or_default()
//                 .entry(num_items)
//                 .or_default()
//                 .append(&mut bonus);
//         }
//     }
//     for (key, val) in &character.stat_bonus {
//         *other_bonus.entry(key.clone()).or_default() += val;
//     }

//     Ok(Evaluator::new(
//         character.clone(),
//         light_cone.clone(),
//         HashMap::new(),
//         other_bonus,
//         HashMap::from([
//             (Stats::Hp, hp_formula.to_owned()),
//             (Stats::Atk, atk_formula.to_owned()),
//             (Stats::Def, def_formula.to_owned()),
//             (Stats::Spd, spd_formula.to_owned()),
//             (Stats::CritRate_, crit_rate_formula.to_owned()),
//             (Stats::CritDmg_, crit_dmg_formula.to_owned()),
//             (
//                 Stats::EnergyRegenerationRate_,
//                 energy_regen_rate_formula.to_owned(),
//             ),
//             (Stats::EffectHitRate_, effect_hit_rate_formula.to_owned()),
//             (Stats::EffectRes_, effect_res_formula.to_owned()),
//             (Stats::BreakEffect_, break_effect_formula.to_owned()),
//         ]),
//         &acheron_ultimate_final_dmg,
//         "AVG Ultimate AoE DMG",
//         vec![Tag::Lightning, Tag::Ultimate],
//         vec![
//             BattleConditionEnum::AfterUsingUltimate,
//             BattleConditionEnum::AfterUsingSkill,
//             BattleConditionEnum::AfterWearerAttack { number_of_times: 3 },
//             BattleConditionEnum::AfterWearerInflictingDebuffs,
//             BattleConditionEnum::AfterAttackingDebuffedEnemy,
//             BattleConditionEnum::TeammatesSamePathWithWearer {
//                 number_of_teammates_having_same_path: 2,
//             },
//             BattleConditionEnum::WhenAttackingEnemyWithDebuff {
//                 number_of_debuffs_enemy_has: 2,
//             },
//         ],
//     ))
// }
