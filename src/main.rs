use crate::domain::{ScannerInput, Stats};
use eyre::Result;
use std::{collections::HashMap, fs};

mod domain;
mod engine;

#[tokio::main]
async fn main() -> Result<()> {
    let file = fs::File::open("scanned_data/HSRScanData_20231126_122032.json")?;
    let json: serde_json::Value = serde_json::from_reader(file)?;
    let mut input: ScannerInput = serde_json::from_value(json)?;
    input.update().await?;
    // println!("{:#?}", input);

    let fuxuan = input
        .characters
        .iter()
        .find(|c| c.key == domain::CharacterName::FuXuan)
        .ok_or(eyre::eyre!("Missing Fu Xuan"))?;
    println!("----------------- Optimizing Fu Xuan's HP -----------------");
    println!("Current stats: {:#?}", fuxuan.stats_panel);
    let res = input
        .optimize(
            domain::CharacterName::FuXuan,
            domain::Stats::Hp,
            HashMap::from([(Stats::EnergyRegenerationRate_, 110.0), (Stats::Spd, 125.0)]),
            // HashMap::new(),
        )
        .await?;
    println!("Optimized stats: {:#?}\nRelics: {:#?}", res.1, res.0);

    // let head = Relic {
    //     set: domain::RelicSetName::KnightOfPurityPalace,
    //     slot: domain::Slot::Head,
    //     rarity: 5,
    //     level: 15,
    //     mainstat: domain::Stats::HP,
    //     substats: vec![
    //         SubStats {
    //             key: domain::Stats::ATK,
    //             value: 38.0,
    //         },
    //         SubStats {
    //             key: domain::Stats::DEF,
    //             value: 21.0,
    //         },
    //         SubStats {
    //             key: domain::Stats::HP_,
    //             value: 13.8,
    //         },
    //         SubStats {
    //             key: domain::Stats::ATK_,
    //             value: 7.3,
    //         },
    //     ],
    //     location: Some(domain::CharacterName::FuXuan),
    //     lock: false,
    //     _id: "relic_1".to_string(),
    // };
    // let hands = Relic {
    //     set: domain::RelicSetName::LongevousDisciple,
    //     slot: domain::Slot::Hands,
    //     rarity: 5,
    //     level: 0,
    //     mainstat: domain::Stats::ATK,
    //     substats: vec![
    //         SubStats {
    //             key: domain::Stats::HP,
    //             value: 42.0,
    //         },
    //         SubStats {
    //             key: domain::Stats::DEF,
    //             value: 21.0,
    //         },
    //         SubStats {
    //             key: domain::Stats::DEF_,
    //             value: 4.3,
    //         },
    //     ],
    //     location: Some(domain::CharacterName::FuXuan),
    //     lock: false,
    //     _id: "relic_2".to_string(),
    // };
    // let body = Relic {
    //     set: domain::RelicSetName::LongevousDisciple,
    //     slot: domain::Slot::Body,
    //     rarity: 5,
    //     level: 15,
    //     mainstat: domain::Stats::HP_,
    //     substats: vec![
    //         SubStats {
    //             key: domain::Stats::HP,
    //             value: 80.0,
    //         },
    //         SubStats {
    //             key: domain::Stats::ATK,
    //             value: 16.0,
    //         },
    //         SubStats {
    //             key: domain::Stats::DEF,
    //             value: 21.0,
    //         },
    //         SubStats {
    //             key: domain::Stats::DEF_,
    //             value: 19.9,
    //         },
    //     ],
    //     location: Some(domain::CharacterName::FuXuan),
    //     lock: false,
    //     _id: "relic_3".to_string(),
    // };
    // let feet = Relic {
    //     set: domain::RelicSetName::KnightOfPurityPalace,
    //     slot: domain::Slot::Feet,
    //     rarity: 5,
    //     level: 15,
    //     mainstat: domain::Stats::SPD,
    //     substats: vec![
    //         SubStats {
    //             key: domain::Stats::ATK,
    //             value: 19.0,
    //         },
    //         SubStats {
    //             key: domain::Stats::HP_,
    //             value: 7.3,
    //         },
    //         SubStats {
    //             key: domain::Stats::ATK_,
    //             value: 4.3,
    //         },
    //         SubStats {
    //             key: domain::Stats::EffectHitRate_,
    //             value: 16.4,
    //         },
    //     ],
    //     location: Some(domain::CharacterName::FuXuan),
    //     lock: false,
    //     _id: "relic_4".to_string(),
    // };
    // let sphere = Relic {
    //     set: domain::RelicSetName::FleetOfTheAgeless,
    //     slot: domain::Slot::PlanarSphere,
    //     rarity: 5,
    //     level: 15,
    //     mainstat: domain::Stats::HP_,
    //     substats: vec![
    //         SubStats {
    //             key: domain::Stats::DEF_,
    //             value: 9.7,
    //         },
    //         SubStats {
    //             key: domain::Stats::EffectHitRate_,
    //             value: 7.7,
    //         },
    //         SubStats {
    //             key: domain::Stats::EffectRES_,
    //             value: 12.0,
    //         },
    //         SubStats {
    //             key: domain::Stats::BreakEffect_,
    //             value: 5.8,
    //         },
    //     ],
    //     location: Some(domain::CharacterName::FuXuan),
    //     lock: false,
    //     _id: "relic_1".to_string(),
    // };
    // let rope = Relic {
    //     set: domain::RelicSetName::FleetOfTheAgeless,
    //     slot: domain::Slot::LinkRope,
    //     rarity: 5,
    //     level: 15,
    //     mainstat: domain::Stats::EnergyRegenerationRate_,
    //     substats: vec![
    //         SubStats {
    //             key: domain::Stats::HP,
    //             value: 71.0,
    //         },
    //         SubStats {
    //             key: domain::Stats::ATK,
    //             value: 19.0,
    //         },
    //         SubStats {
    //             key: domain::Stats::ATK_,
    //             value: 10.8,
    //         },
    //         SubStats {
    //             key: domain::Stats::EffectRES_,
    //             value: 8.2,
    //         },
    //     ],
    //     location: Some(domain::CharacterName::FuXuan),
    //     lock: false,
    //     _id: "relic_1".to_string(),
    // };

    // let relics = vec![head, hands, body, feet, sphere, rope];

    // let fu_xuan = Character::new(
    //     domain::CharacterName::FuXuan,
    //     80,
    //     6,
    //     0,
    //     CharacterSkills {
    //         basic: 1,
    //         skill: 9,
    //         ult: 9,
    //         talent: 9,
    //     },
    //     CharacterTraces {
    //         ability_1: true,
    //         ability_2: true,
    //         ability_3: true,
    //         stat_1: true,
    //         stat_2: true,
    //         stat_3: false,
    //         stat_4: true,
    //         stat_5: true,
    //         stat_6: true,
    //         stat_7: false,
    //         stat_8: true,
    //         stat_9: true,
    //         stat_10: false,
    //         total_bonus: HashMap::new(),
    //     },
    //     Some(LightCone {
    //         key: domain::LightConeName::WeAreWildfire,
    //         level: 80,
    //         ascension: 6,
    //         superimposition: 0,
    //         location: Some(domain::CharacterName::FuXuan),
    //         lock: false,
    //         _id: "lightcone_100".to_string(),
    //     }),
    //     relics,
    // )
    // .await?;

    // println!("{:#?}", fu_xuan);
    Ok(())
}
