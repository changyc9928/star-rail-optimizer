use eyre::{bail, Result};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, EnumIter, Hash)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
    EffectRes_,
    #[serde(alias = "Break Effect_", alias = "Break Effect")]
    BreakEffect_,
    #[serde(alias = "Outgoing Healing Boost")]
    OutgoingHealingBoost_,
    #[serde(alias = "Fire DMG Boost")]
    FireDmgBoost_,
    #[serde(alias = "Ice DMG Boost")]
    IceDmgBoost_,
    #[serde(alias = "Wind DMG Boost")]
    WindDmgBoost_,
    #[serde(alias = "Lightning DMG Boost")]
    LightningDmgBoost_,
    #[serde(alias = "Quantum DMG Boost")]
    QuantumDmgBoost_,
    #[serde(alias = "Imaginary DMG Boost")]
    ImaginaryDmgBoost_,
    #[serde(alias = "Physical DMG Boost")]
    PhysicalDmgBoost_,
    #[serde(alias = "DMG Boost")]
    DmgBoost_,
    #[serde(alias = "Basic ATK DMG Boost")]
    BasicAtkDmgBoost_,
    #[serde(alias = "Skill DMG Boost")]
    SkillDmgBoost_,
    #[serde(alias = "Ultimate DMG Boost")]
    UltimateDmgBoost_,
    #[serde(alias = "Follow-up ATK DMG Boost")]
    FollowUpAtkDmgBoost_,
    #[serde(alias = "Shield DMG Absorption")]
    ShieldDmgAbsorption_,
    #[serde(alias = "DMG Reduction")]
    DmgMitigation_,
    #[serde(alias = "DEF Reduction")]
    DefReduction_,
    #[serde(alias = "DEF Ignore")]
    DefIgnore_,
    #[serde(alias = "Break DMG DEF Ignore")]
    BreakDmgDefIgnore_,
    #[serde(alias = "Super Break DMG DEF Ignore")]
    SuperBreakDmgDefIgnore_,
    ResPenalty_,
    Vulnerebility_,
    Weaken_,
    #[default]
    Dummy,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
    #[serde(alias = "Pioneer Diver of Dead Waters")]
    PioneerDiverOfDeadWaters,
    #[serde(alias = "Watchmaker, Master of Dream Machinations")]
    WatchmakerMasterOfDreamMachinations,
    #[serde(alias = "Iron Cavalry Against the Scourge")]
    IronCavalryAgainstTheScourge,
    #[serde(alias = "The Wind-Soaring Valorous")]
    TheWindSoaringValorous,
    #[default]
    Dummy,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Relic {
    pub set_id: String,
    pub name: String,
    pub slot: Slot,
    pub rarity: u8,
    pub level: u8,
    pub mainstat: Stats,
    pub substats: Vec<SubStats>,
    pub location: Option<String>,
    pub lock: bool,
    pub discard: bool,
    pub _uid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SubStats {
    pub key: Stats,
    pub value: f64,
}

impl Relic {
    pub fn get_mainstat(&self) -> Result<f64> {
        let stat = match &self.mainstat {
            &Stats::Atk if self.slot != Slot::Hands => Stats::Atk_,
            &Stats::Hp if self.slot != Slot::Head => Stats::Hp_,
            other => other.clone(),
        };
        Ok(match (self.rarity, &stat) {
            (5, Stats::Spd) => 4.032 + 1.4 * self.level as f64,
            (5, Stats::Hp) => 112.896 + 39.5136 * self.level as f64,
            (5, Stats::Atk) => 56.448 + 19.7568 * self.level as f64,
            (5, Stats::Hp_) | (5, Stats::Atk_) | (5, Stats::EffectHitRate_) => {
                6.9120 + 2.4192 * self.level as f64
            }
            (5, Stats::Def_) => 8.64 + 3.024 * self.level as f64, // Special case
            (5, Stats::BreakEffect_) => 10.3680 + 3.6277 * self.level as f64,
            (5, Stats::EnergyRegenerationRate_) => 3.1104 + 1.0886 * self.level as f64,
            (5, Stats::OutgoingHealingBoost_) => 5.5296 + 1.9354 * self.level as f64,
            (5, Stats::PhysicalDmgBoost_)
            | (5, Stats::FireDmgBoost_)
            | (5, Stats::IceDmgBoost_)
            | (5, Stats::WindDmgBoost_)
            | (5, Stats::LightningDmgBoost_)
            | (5, Stats::QuantumDmgBoost_)
            | (5, Stats::ImaginaryDmgBoost_) => 6.2208 + 2.1773 * self.level as f64,
            (5, Stats::CritRate_) => 5.184 + 1.8144 * self.level as f64,
            (5, Stats::CritDmg_) => 10.368 + 3.6288 * self.level as f64,
            (4, Stats::Spd) => 3.2256 + 1.1 * self.level as f64,
            (4, Stats::Hp) => 90.3168 + 31.61088 * self.level as f64,
            (4, Stats::Atk) => 45.1584 + 15.80544 * self.level as f64,
            (4, Stats::Hp_) | (4, Stats::Atk_) | (4, Stats::EffectHitRate_) => {
                5.5296 + 1.9354 * self.level as f64
            }
            (4, Stats::Def_) => 6.912 + 2.4192 * self.level as f64, // Special case
            (4, Stats::BreakEffect_) => 8.2944 + 2.9030 * self.level as f64,
            (4, Stats::EnergyRegenerationRate_) => 2.4883 + 0.8709 * self.level as f64,
            (4, Stats::OutgoingHealingBoost_) => 4.4237 + 1.5483 * self.level as f64,
            (4, Stats::PhysicalDmgBoost_)
            | (4, Stats::FireDmgBoost_)
            | (4, Stats::IceDmgBoost_)
            | (4, Stats::WindDmgBoost_)
            | (4, Stats::LightningDmgBoost_)
            | (4, Stats::QuantumDmgBoost_)
            | (4, Stats::ImaginaryDmgBoost_) => 4.9766 + 1.7418 * self.level as f64,
            (4, Stats::CritRate_) => 4.1472 + 1.4515 * self.level as f64,
            (4, Stats::CritDmg_) => 8.2944 + 2.9030 * self.level as f64,
            (3, Stats::Spd) => 2.4192 + 1.0 * self.level as f64,
            (3, Stats::Hp) => 67.7376 + 23.70816 * self.level as f64,
            (3, Stats::Atk) => 33.8688 + 11.85408 * self.level as f64,
            (3, Stats::Hp_) | (3, Stats::Atk_) | (3, Stats::EffectHitRate_) => {
                4.1472 + 1.4515 * self.level as f64
            }
            (3, Stats::Def_) => 5.184 + 1.8144 * self.level as f64, // Special case
            (3, Stats::BreakEffect_) => 6.2208 + 2.1773 * self.level as f64,
            (3, Stats::EnergyRegenerationRate_) => 1.8662 + 0.6532 * self.level as f64,
            (3, Stats::OutgoingHealingBoost_) => 3.3178 + 1.1612 * self.level as f64,
            (3, Stats::PhysicalDmgBoost_)
            | (3, Stats::FireDmgBoost_)
            | (3, Stats::IceDmgBoost_)
            | (3, Stats::WindDmgBoost_)
            | (3, Stats::LightningDmgBoost_)
            | (3, Stats::QuantumDmgBoost_)
            | (3, Stats::ImaginaryDmgBoost_) => 3.7325 + 1.3064 * self.level as f64,
            (3, Stats::CritRate_) => 3.1104 + 1.0886 * self.level as f64,
            (3, Stats::CritDmg_) => 6.2208 + 2.1773 * self.level as f64,
            (2, Stats::Spd) => 1.6128 + 1.0 * self.level as f64,
            (2, Stats::Hp) => 45.1584 + 15.80544 * self.level as f64,
            (2, Stats::Atk) => 22.5792 + 7.90272 * self.level as f64,
            (2, Stats::Hp_) | (2, Stats::Atk_) | (2, Stats::EffectHitRate_) => {
                2.7648 + 0.9677 * self.level as f64
            }
            (2, Stats::Def_) => 3.456 + 1.2096 * self.level as f64, // Special case
            (2, Stats::BreakEffect_) => 4.1472 + 1.4515 * self.level as f64,
            (2, Stats::EnergyRegenerationRate_) => 1.2442 + 0.4355 * self.level as f64,
            (2, Stats::OutgoingHealingBoost_) => 2.2118 + 0.7741 * self.level as f64,
            (2, Stats::PhysicalDmgBoost_)
            | (2, Stats::FireDmgBoost_)
            | (2, Stats::IceDmgBoost_)
            | (2, Stats::WindDmgBoost_)
            | (2, Stats::LightningDmgBoost_)
            | (2, Stats::QuantumDmgBoost_)
            | (2, Stats::ImaginaryDmgBoost_) => 2.4883 + 0.8709 * self.level as f64,
            (2, Stats::CritRate_) => 2.0736 + 0.7258 * self.level as f64,
            (2, Stats::CritDmg_) => 4.1472 + 1.4515 * self.level as f64,
            other => bail!("Invalid rarity or stats: {other:?}"),
        })
    }
}
