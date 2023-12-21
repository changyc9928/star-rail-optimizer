use crate::domain::CharacterName;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum Slot {
    Head,
    Feet,
    Body,
    Hands,
    #[serde(alias = "Link Rope")]
    LinkRope,
    #[serde(alias = "Planar Sphere")]
    PlanarSphere,
    #[default]
    Dummy,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum Stats {
    #[serde(alias = "ATK")]
    Atk,
    #[serde(alias = "ATK_")]
    Atk_,
    #[serde(alias = "DEF")]
    Def,
    #[serde(alias = "DEF_")]
    Def_,
    #[serde(alias = "HP")]
    Hp,
    #[serde(alias = "HP_")]
    Hp_,
    #[serde(alias = "CRIT Rate_", alias = "CRIT Rate")]
    CritRate_,
    #[serde(alias = "CRIT DMG_", alias = "CRIT DMG")]
    CritDmg_,
    #[serde(alias = "SPD")]
    Spd,
    #[serde(alias = "SPD_")]
    Spd_,
    #[serde(alias = "Energy Regeneration Rate")]
    EnergyRegenerationRate_,
    #[serde(alias = "Effect Hit Rate_", alias = "Effect Hit Rate")]
    EffectHitRate_,
    #[serde(alias = "Effect RES_")]
    EffectRES_,
    #[serde(alias = "Break Effect_", alias = "Break Effect")]
    BreakEffect_,
    #[serde(alias = "Outgoing Healing Boost")]
    OutgoingHealingBoost_,
    #[serde(alias = "Fire DMG Boost")]
    FireDMGBoost_,
    #[serde(alias = "Ice DMG Boost")]
    IceDMGBoost_,
    #[serde(alias = "Wind DMG Boost")]
    WindDMGBoost_,
    #[serde(alias = "Lightning DMG Boost")]
    LightningDMGBoost_,
    #[serde(alias = "Quantum DMG Boost")]
    QuantumDMGBoost_,
    #[serde(alias = "Imaginary DMG Boost")]
    ImaginaryDMGBoost_,
    #[serde(alias = "Physical DMG Boost")]
    PhysicalDMGBoost_,
    #[default]
    Dummy,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum RelicSetName {
    #[serde(alias = "Passerby of Wandering Cloud")]
    PasserbyOfWanderingCloud,
    #[serde(alias = "Musketeer of Wild Wheat")]
    MusketeerOfWildWheat,
    #[serde(alias = "Knight of Purity Palace")]
    KnightOfPurityPalace,
    #[serde(alias = "Hunter of Glacial Forest")]
    HunterOfGlacialForest,
    #[serde(alias = "Champion of Streetwise Boxing")]
    ChampionOfStreetwiseBoxing,
    #[serde(alias = "Guard of Wuthering Snow")]
    GuardOfWutheringSnow,
    #[serde(alias = "Firesmith of Lava-Forging")]
    FiresmithOfLavaForging,
    #[serde(alias = "Genius of Brilliant Stars")]
    GeniusOfBrilliantStars,
    #[serde(alias = "Band of Sizzling Thunder")]
    BandOfSizzlingThunder,
    #[serde(alias = "Eagle of Twilight Line")]
    EagleOfTwilightLine,
    #[serde(alias = "Thief of Shooting Meteor")]
    ThiefOfShootingMeteor,
    #[serde(alias = "Wastelander of Banditry Desert")]
    WastelanderOfBanditryDesert,
    #[serde(alias = "Space Sealing Station")]
    SpaceSealingStation,
    #[serde(alias = "Fleet of the Ageless")]
    FleetOfTheAgeless,
    #[serde(alias = "Pan-Cosmic Commercial Enterprise")]
    PanCosmicCommercialEnterprise,
    #[serde(alias = "Belobog of the Architects")]
    BelobogOfTheArchitects,
    #[serde(alias = "Celestial Differentiator")]
    CelestialDifferentiator,
    #[serde(alias = "Inert Salsotto")]
    InertSalsotto,
    #[serde(alias = "Talia: Kingdom of Banditry")]
    TaliaKingdomOfBanditry,
    #[serde(alias = "Sprightly Vonwacq")]
    SprightlyVonwacq,
    #[serde(alias = "Rutilant Arena")]
    RutilantArena,
    #[serde(alias = "Broken Keel")]
    BrokenKeel,
    #[serde(alias = "Longevous Disciple")]
    LongevousDisciple,
    #[serde(alias = "Messenger Traversing Hackerspace")]
    MessengerTraversingHackerspace,
    #[serde(alias = "The Ashblazing Grand Duke")]
    TheAshblazingGrandDuke,
    #[serde(alias = "Prisoner in Deep Confinement")]
    PrisonerInDeepConfinement,
    #[serde(alias = "Firmament Frontline: Glamoth")]
    FirmamentFrontlineGlamoth,
    #[serde(alias = "Penacony, Land of the Dreams")]
    PenaconyLandOfTheDreams,
    #[default]
    Dummy,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Relic {
    pub set: RelicSetName,
    pub slot: Slot,
    pub rarity: u8,
    pub level: u8,
    pub mainstat: Stats,
    pub substats: Vec<SubStats>,
    pub location: Option<CharacterName>,
    pub lock: bool,
    pub _id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SubStats {
    pub key: Stats,
    pub value: f64,
}

impl Relic {
    pub fn get_mainstat(&self) -> f64 {
        let stat = match &self.mainstat {
            &Stats::Atk if self.slot != Slot::Hands => Stats::Atk_,
            &Stats::Hp if self.slot != Slot::Head => Stats::Hp_,
            other => other.clone(),
        };
        match (self.rarity, &stat) {
            (5, Stats::Spd) => 4.032 + 1.4 * self.level as f64,
            (5, Stats::Hp) => 112.896 + 39.5136 * self.level as f64,
            (5, Stats::Atk) => 56.448 + 19.7568 * self.level as f64,
            (5, Stats::Hp_) | (5, Stats::Atk_) | (5, Stats::EffectHitRate_) => {
                6.9120 + 2.4192 * self.level as f64
            }
            (5, Stats::Def) => 8.64 + 3.024 * self.level as f64, // Special case
            (5, Stats::BreakEffect_) => 10.3680 + 3.6277 * self.level as f64,
            (5, Stats::EnergyRegenerationRate_) => 3.1104 + 1.0886 * self.level as f64,
            (5, Stats::OutgoingHealingBoost_) => 5.5296 + 1.9354 * self.level as f64,
            (5, Stats::PhysicalDMGBoost_)
            | (5, Stats::FireDMGBoost_)
            | (5, Stats::IceDMGBoost_)
            | (5, Stats::WindDMGBoost_)
            | (5, Stats::LightningDMGBoost_)
            | (5, Stats::QuantumDMGBoost_)
            | (5, Stats::ImaginaryDMGBoost_) => 6.2208 + 2.1773 * self.level as f64,
            (5, Stats::CritRate_) => 5.184 + 1.8144 * self.level as f64,
            (5, Stats::CritDmg_) => 10.368 + 3.6288 * self.level as f64,
            (4, Stats::Spd) => 3.2256 + 1.1 * self.level as f64,
            (4, Stats::Hp) => 90.3168 + 31.61088 * self.level as f64,
            (4, Stats::Atk) => 45.1584 + 15.80544 * self.level as f64,
            (4, Stats::Hp_) | (4, Stats::Atk_) | (4, Stats::EffectHitRate_) => {
                5.5296 + 1.9354 * self.level as f64
            }
            (4, Stats::Def) => 6.912 + 2.4192 * self.level as f64, // Special case
            (4, Stats::BreakEffect_) => 8.2944 + 2.9030 * self.level as f64,
            (4, Stats::EnergyRegenerationRate_) => 2.4883 + 0.8709 * self.level as f64,
            (4, Stats::OutgoingHealingBoost_) => 4.4237 + 1.5483 * self.level as f64,
            (4, Stats::PhysicalDMGBoost_)
            | (4, Stats::FireDMGBoost_)
            | (4, Stats::IceDMGBoost_)
            | (4, Stats::WindDMGBoost_)
            | (4, Stats::LightningDMGBoost_)
            | (4, Stats::QuantumDMGBoost_)
            | (4, Stats::ImaginaryDMGBoost_) => 4.9766 + 1.7418 * self.level as f64,
            (4, Stats::CritRate_) => 4.1472 + 1.4515 * self.level as f64,
            (4, Stats::CritDmg_) => 8.2944 + 2.9030 * self.level as f64,
            _ => 0.0, // TODO
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Relics {
    pub head: Option<Relic>,
    pub hands: Option<Relic>,
    pub body: Option<Relic>,
    pub feet: Option<Relic>,
    pub planar_sphere: Option<Relic>,
    pub link_rope: Option<Relic>,
    pub total: HashMap<Stats, f64>,
}

impl Relics {
    pub fn new(
        head: Option<Relic>,
        hands: Option<Relic>,
        body: Option<Relic>,
        feet: Option<Relic>,
        planar_sphere: Option<Relic>,
        link_rope: Option<Relic>,
    ) -> Self {
        let mut total = HashMap::new();
        let mut substats = vec![];
        if let Some(head) = &head {
            substats.push(head.substats.clone());
        }
        if let Some(hands) = &hands {
            substats.push(hands.substats.clone());
        }
        if let Some(body) = &body {
            substats.push(body.substats.clone());
        }
        if let Some(feet) = &feet {
            substats.push(feet.substats.clone());
        }
        if let Some(planar_sphere) = &planar_sphere {
            substats.push(planar_sphere.substats.clone());
        }
        if let Some(link_rope) = &link_rope {
            substats.push(link_rope.substats.clone());
        }
        for substat in substats
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<SubStats>>()
        {
            let s = total.get_mut(&substat.key);
            match s {
                Some(s) => *s += substat.value,
                None => {
                    total.insert(substat.key, substat.value);
                }
            }
        }
        for relic in &[&head, &hands, &body, &feet, &planar_sphere, &link_rope] {
            match relic {
                Some(relic) => {
                    let mainstat = match &relic.mainstat {
                        Stats::Atk if relic.slot != Slot::Hands => Stats::Atk_,
                        Stats::Hp if relic.slot != Slot::Head => Stats::Hp_,
                        other => other.clone(),
                    };
                    let s = total.get_mut(&mainstat);
                    match s {
                        Some(s) => *s += relic.get_mainstat(),
                        None => {
                            total.insert(mainstat, relic.get_mainstat());
                        }
                    }
                }
                None => {}
            }
        }
        Self {
            head,
            hands,
            body,
            feet,
            planar_sphere,
            link_rope,
            total,
        }
    }

    pub fn get_set_bonus(
        &self,
        base_spd: f64,
        combat_type: &Stats,
        bonus: HashMap<Stats, f64>,
    ) -> eyre::Result<HashMap<Stats, f64>> {
        let mut set = HashMap::new();
        match &self.head {
            Some(relic) => {
                set.insert(relic.set.clone(), 1);
            }
            None => (),
        }
        match &self.hands {
            Some(relic) => {
                if !set.contains_key(&relic.set) {
                    set.insert(relic.set.clone(), 1);
                } else {
                    *set.get_mut(&relic.set).ok_or(eyre::eyre!("Missing set"))? += 1;
                }
            }
            None => (),
        }
        match &self.body {
            Some(relic) => {
                if !set.contains_key(&relic.set) {
                    set.insert(relic.set.clone(), 1);
                } else {
                    *set.get_mut(&relic.set).ok_or(eyre::eyre!("Missing set"))? += 1;
                }
            }
            None => (),
        }
        match &self.feet {
            Some(relic) => {
                if !set.contains_key(&relic.set) {
                    set.insert(relic.set.clone(), 1);
                } else {
                    *set.get_mut(&relic.set).ok_or(eyre::eyre!("Missing set"))? += 1;
                }
            }
            None => (),
        }
        match &self.planar_sphere {
            Some(relic) => {
                if !set.contains_key(&relic.set) {
                    set.insert(relic.set.clone(), 1);
                } else {
                    *set.get_mut(&relic.set).ok_or(eyre::eyre!("Missing set"))? += 1;
                }
            }
            None => (),
        }
        match &self.link_rope {
            Some(relic) => {
                if !set.contains_key(&relic.set) {
                    set.insert(relic.set.clone(), 1);
                } else {
                    *set.get_mut(&relic.set).ok_or(eyre::eyre!("Missing set"))? += 1;
                }
            }
            None => (),
        }
        let mut ret = HashMap::new();
        for (k, v) in &set {
            match k {
                RelicSetName::PasserbyOfWanderingCloud => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::OutgoingHealingBoost_)
                        {
                            e.insert(10.0);
                        } else {
                            *ret.get_mut(&Stats::OutgoingHealingBoost_)
                                .ok_or(eyre::eyre!("Missing set"))? += 10.0;
                        }
                    }
                }
                RelicSetName::MusketeerOfWildWheat => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) = ret.entry(Stats::Atk_)
                        {
                            e.insert(12.0);
                        } else {
                            *ret.get_mut(&Stats::Atk_)
                                .ok_or(eyre::eyre!("Missing set"))? += 12.0;
                        }
                    }
                    if v >= &4 {
                        // TODO
                    }
                }
                RelicSetName::KnightOfPurityPalace => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) = ret.entry(Stats::Def_)
                        {
                            e.insert(15.0);
                        } else {
                            *ret.get_mut(&Stats::Def_)
                                .ok_or(eyre::eyre!("Missing set"))? += 15.0;
                        }
                    }
                }
                RelicSetName::HunterOfGlacialForest => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::IceDMGBoost_)
                        {
                            e.insert(10.0);
                        } else {
                            *ret.get_mut(&Stats::IceDMGBoost_)
                                .ok_or(eyre::eyre!("Missing set"))? += 10.0;
                        }
                    }
                }
                RelicSetName::ChampionOfStreetwiseBoxing => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::PhysicalDMGBoost_)
                        {
                            e.insert(10.0);
                        } else {
                            *ret.get_mut(&Stats::PhysicalDMGBoost_)
                                .ok_or(eyre::eyre!("Missing set"))? += 10.0;
                        }
                    }
                }
                RelicSetName::GuardOfWutheringSnow => (),
                RelicSetName::FiresmithOfLavaForging => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::FireDMGBoost_)
                        {
                            e.insert(10.0);
                        } else {
                            *ret.get_mut(&Stats::FireDMGBoost_)
                                .ok_or(eyre::eyre!("Missing set"))? += 10.0;
                        }
                    }
                }
                RelicSetName::GeniusOfBrilliantStars => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::QuantumDMGBoost_)
                        {
                            e.insert(10.0);
                        } else {
                            *ret.get_mut(&Stats::QuantumDMGBoost_)
                                .ok_or(eyre::eyre!("Missing set"))? += 10.0;
                        }
                    }
                }
                RelicSetName::BandOfSizzlingThunder => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::LightningDMGBoost_)
                        {
                            e.insert(10.0);
                        } else {
                            *ret.get_mut(&Stats::LightningDMGBoost_)
                                .ok_or(eyre::eyre!("Missing set"))? += 10.0;
                        }
                    }
                }
                RelicSetName::EagleOfTwilightLine => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::WindDMGBoost_)
                        {
                            e.insert(10.0);
                        } else {
                            *ret.get_mut(&Stats::WindDMGBoost_)
                                .ok_or(eyre::eyre!("Missing set"))? += 10.0;
                        }
                    }
                }
                RelicSetName::ThiefOfShootingMeteor => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::BreakEffect_)
                        {
                            e.insert(16.0);
                        } else {
                            *ret.get_mut(&Stats::BreakEffect_)
                                .ok_or(eyre::eyre!("Missing set"))? += 16.0;
                        }
                    }
                }
                RelicSetName::WastelanderOfBanditryDesert => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::ImaginaryDMGBoost_)
                        {
                            e.insert(10.0);
                        } else {
                            *ret.get_mut(&Stats::ImaginaryDMGBoost_)
                                .ok_or(eyre::eyre!("Missing set"))? += 10.0;
                        }
                    }
                }
                RelicSetName::LongevousDisciple => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) = ret.entry(Stats::Hp_)
                        {
                            e.insert(12.0);
                        } else {
                            *ret.get_mut(&Stats::Hp_).ok_or(eyre::eyre!("Missing set"))? += 12.0;
                        }
                    }
                }
                RelicSetName::MessengerTraversingHackerspace => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) = ret.entry(Stats::Spd_)
                        {
                            e.insert(6.0);
                        } else {
                            *ret.get_mut(&Stats::Spd_)
                                .ok_or(eyre::eyre!("Missing set"))? += 6.0;
                        }
                    }
                }
                RelicSetName::TheAshblazingGrandDuke => (),
                RelicSetName::PrisonerInDeepConfinement => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) = ret.entry(Stats::Atk_)
                        {
                            e.insert(12.0);
                        } else {
                            *ret.get_mut(&Stats::Atk_)
                                .ok_or(eyre::eyre!("Missing set"))? += 12.0;
                        }
                    }
                }
                RelicSetName::SpaceSealingStation => (),
                RelicSetName::FleetOfTheAgeless => (),
                RelicSetName::PanCosmicCommercialEnterprise => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::EffectHitRate_)
                        {
                            e.insert(10.0);
                        } else {
                            *ret.get_mut(&Stats::EffectHitRate_)
                                .ok_or(eyre::eyre!("Missing set"))? += 10.0;
                        }
                    }
                }
                RelicSetName::BelobogOfTheArchitects => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) = ret.entry(Stats::Def_)
                        {
                            e.insert(15.0);
                        } else {
                            *ret.get_mut(&Stats::Def_)
                                .ok_or(eyre::eyre!("Missing set"))? += 15.0;
                        }
                    }
                }
                RelicSetName::CelestialDifferentiator => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::CritDmg_)
                        {
                            e.insert(16.0);
                        } else {
                            *ret.get_mut(&Stats::CritDmg_)
                                .ok_or(eyre::eyre!("Missing set"))? += 16.0;
                        }
                    }
                }
                RelicSetName::InertSalsotto => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::CritRate_)
                        {
                            e.insert(8.0);
                        } else {
                            *ret.get_mut(&Stats::CritRate_)
                                .ok_or(eyre::eyre!("Missing set"))? += 8.0;
                        }
                    }
                }
                RelicSetName::TaliaKingdomOfBanditry => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::BreakEffect_)
                        {
                            e.insert(16.0);
                        } else {
                            *ret.get_mut(&Stats::BreakEffect_)
                                .ok_or(eyre::eyre!("Missing set"))? += 16.0;
                        }
                    }
                }
                RelicSetName::SprightlyVonwacq => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::EnergyRegenerationRate_)
                        {
                            e.insert(5.0);
                        } else {
                            *ret.get_mut(&Stats::EnergyRegenerationRate_)
                                .ok_or(eyre::eyre!("Missing set"))? += 5.0;
                        }
                    }
                }
                RelicSetName::RutilantArena => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::CritRate_)
                        {
                            e.insert(8.0);
                        } else {
                            *ret.get_mut(&Stats::CritRate_)
                                .ok_or(eyre::eyre!("Missing set"))? += 8.0;
                        }
                    }
                }
                RelicSetName::BrokenKeel => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::EffectRES_)
                        {
                            e.insert(10.0);
                        } else {
                            *ret.get_mut(&Stats::EffectRES_)
                                .ok_or(eyre::eyre!("Missing set"))? += 10.0;
                        }
                    }
                }
                RelicSetName::FirmamentFrontlineGlamoth => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) = ret.entry(Stats::Atk_)
                        {
                            e.insert(12.0);
                        } else {
                            *ret.get_mut(&Stats::Atk_)
                                .ok_or(eyre::eyre!("Missing set"))? += 12.0;
                        }
                    }
                }
                RelicSetName::PenaconyLandOfTheDreams => {
                    if v >= &2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::EnergyRegenerationRate_)
                        {
                            e.insert(5.0);
                        } else {
                            *ret.get_mut(&Stats::EnergyRegenerationRate_)
                                .ok_or(eyre::eyre!("Missing set"))? += 5.0;
                        }
                    }
                }
                RelicSetName::Dummy => todo!(),
            }
        }
        for (k, v) in set {
            match k {
                RelicSetName::SpaceSealingStation => {
                    if v >= 2 {
                        let spd = base_spd
                            * (1.0
                                + ((self.total.get(&Stats::Spd_).cloned().unwrap_or_default()
                                    + ret.get(&Stats::Spd_).cloned().unwrap_or_default()
                                    + bonus.get(&Stats::Spd_).cloned().unwrap_or_default())
                                    / 100.0))
                            + (self.total.get(&Stats::Spd).cloned().unwrap_or_default()
                                + ret.get(&Stats::Spd).cloned().unwrap_or_default());
                        if let std::collections::hash_map::Entry::Vacant(e) = ret.entry(Stats::Atk_)
                        {
                            e.insert(12.0);
                        } else {
                            *ret.get_mut(&Stats::Atk_)
                                .ok_or(eyre::eyre!("Missing set"))? += 12.0;
                        }
                        if spd >= 120.0 {
                            *ret.get_mut(&Stats::Atk_)
                                .ok_or(eyre::eyre!("Missing set"))? += 12.0;
                        }
                    }
                }
                RelicSetName::FleetOfTheAgeless => {
                    if v >= 2 {
                        let spd = base_spd
                            * (1.0
                                + ((self.total.get(&Stats::Spd_).cloned().unwrap_or_default()
                                    + ret.get(&Stats::Spd_).cloned().unwrap_or_default()
                                    + bonus.get(&Stats::Spd_).cloned().unwrap_or_default())
                                    / 100.0))
                            + (self.total.get(&Stats::Spd).cloned().unwrap_or_default()
                                + ret.get(&Stats::Spd).cloned().unwrap_or_default());
                        if let std::collections::hash_map::Entry::Vacant(e) = ret.entry(Stats::Hp_)
                        {
                            e.insert(12.0);
                        } else {
                            *ret.get_mut(&Stats::Hp_).ok_or(eyre::eyre!("Missing set"))? += 12.0;
                        }
                        if spd >= 120.0 {
                            if let std::collections::hash_map::Entry::Vacant(e) =
                                ret.entry(Stats::Atk_)
                            {
                                e.insert(8.0);
                            } else {
                                *ret.get_mut(&Stats::Atk_)
                                    .ok_or(eyre::eyre!("Missing set"))? += 8.0;
                            }
                        }
                    }
                }
                RelicSetName::PanCosmicCommercialEnterprise => {
                    if v >= 2 {
                        let effect_hit_rate = self
                            .total
                            .get(&Stats::EffectHitRate_)
                            .cloned()
                            .unwrap_or_default()
                            + bonus
                                .get(&Stats::EffectHitRate_)
                                .cloned()
                                .unwrap_or_default()
                            + ret.get(&Stats::EffectHitRate_).cloned().unwrap_or_default();
                        if let std::collections::hash_map::Entry::Vacant(e) = ret.entry(Stats::Atk_)
                        {
                            e.insert(f64::min(effect_hit_rate * 25.0, 25.0));
                        } else {
                            *ret.get_mut(&Stats::Atk_)
                                .ok_or(eyre::eyre!("Missing set"))? +=
                                f64::min(effect_hit_rate * 25.0, 25.0);
                        }
                    }
                }
                RelicSetName::BelobogOfTheArchitects => {
                    let effect_hit_rate = self
                        .total
                        .get(&Stats::EffectHitRate_)
                        .cloned()
                        .unwrap_or_default()
                        + bonus
                            .get(&Stats::EffectHitRate_)
                            .cloned()
                            .unwrap_or_default()
                        + ret.get(&Stats::EffectHitRate_).cloned().unwrap_or_default();
                    if effect_hit_rate >= 50.0 && v >= 2 {
                        if let std::collections::hash_map::Entry::Vacant(e) = ret.entry(Stats::Def_)
                        {
                            e.insert(15.0);
                        } else {
                            *ret.get_mut(&Stats::Def_)
                                .ok_or(eyre::eyre!("Missing set"))? += 15.0;
                        }
                    }
                }
                RelicSetName::CelestialDifferentiator => {
                    if v >= 2 {
                        // TODO
                    }
                }
                RelicSetName::InertSalsotto => {
                    if v >= 2 {
                        // TODO
                    }
                }
                RelicSetName::TaliaKingdomOfBanditry => {
                    let spd = base_spd
                        * (1.0
                            + ((self.total.get(&Stats::Spd_).cloned().unwrap_or_default()
                                + ret.get(&Stats::Spd_).cloned().unwrap_or_default()
                                + bonus.get(&Stats::Spd_).cloned().unwrap_or_default())
                                / 100.0))
                        + (self.total.get(&Stats::Spd).cloned().unwrap_or_default()
                            + ret.get(&Stats::Spd).cloned().unwrap_or_default());
                    if spd >= 145.0 && v >= 2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::BreakEffect_)
                        {
                            e.insert(20.0);
                        } else {
                            *ret.get_mut(&Stats::BreakEffect_)
                                .ok_or(eyre::eyre!("Missing set"))? += 20.0;
                        }
                    }
                }
                RelicSetName::RutilantArena => {
                    if v >= 2 {
                        // TODO
                    }
                }
                RelicSetName::BrokenKeel => {
                    let effect_res = self
                        .total
                        .get(&Stats::EffectRES_)
                        .cloned()
                        .unwrap_or_default()
                        + bonus.get(&Stats::EffectRES_).cloned().unwrap_or_default()
                        + ret.get(&Stats::EffectRES_).cloned().unwrap_or_default();
                    if effect_res >= 30.0 && v >= 2 {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            ret.entry(Stats::CritDmg_)
                        {
                            e.insert(10.0);
                        } else {
                            *ret.get_mut(&Stats::CritDmg_)
                                .ok_or(eyre::eyre!("Missing set"))? += 10.0;
                        }
                    }
                }
                RelicSetName::FirmamentFrontlineGlamoth => {
                    let spd = base_spd
                        * (1.0
                            + ((self.total.get(&Stats::Spd_).cloned().unwrap_or_default()
                                + ret.get(&Stats::Spd_).cloned().unwrap_or_default()
                                + bonus.get(&Stats::Spd_).cloned().unwrap_or_default())
                                / 100.0))
                        + (self.total.get(&Stats::Spd).cloned().unwrap_or_default()
                            + ret.get(&Stats::Spd).cloned().unwrap_or_default());
                    let increment = if spd >= 160.0 {
                        18.0
                    } else if spd >= 135.0 {
                        12.0
                    } else {
                        0.0
                    };
                    if v >= 2 {
                        if !ret.contains_key(combat_type) {
                            ret.insert(combat_type.clone(), increment);
                        } else {
                            *ret.get_mut(combat_type).ok_or(eyre::eyre!("Missing set"))? +=
                                increment;
                        }
                    }
                }
                _ => (),
            }
        }
        Ok(ret)
    }
}
