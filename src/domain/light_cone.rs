use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{CharacterName, ProjectYattaCharacterType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LightConeName {
    Arrows,
    Cornucopia,
    #[serde(alias = "Collapsing Sky")]
    CollapsingSky,
    Amber,
    Void,
    Chorus,
    #[serde(alias = "Data Bank")]
    DataBank,
    #[serde(alias = "Darting Arrow")]
    DartingArrow,
    #[serde(alias = "Fine Fruit")]
    FineFruit,
    #[serde(alias = "Shattered Home")]
    ShatteredHome,
    Defense,
    Loop,
    #[serde(alias = "Meshing Cogs")]
    MeshingCogs,
    Passkey,
    Adversarial,
    Multiplication,
    #[serde(alias = "Mutual Demise")]
    MutualDemise,
    Pioneering,
    #[serde(alias = "Hidden Shadow")]
    HiddenShadow,
    Mediation,
    Sagacity,
    #[serde(alias = "Post-Op Conversation")]
    PostOpConversation,
    #[serde(alias = "Good Night and Sleep Well")]
    GoodNightAndSleepWell,
    #[serde(alias = "Day One of My New Life")]
    DayOneOfMyNewLife,
    #[serde(alias = "Only Silence Remains")]
    OnlySilenceRemains,
    #[serde(alias = "Memories of the Past")]
    MemoriesOfThePast,
    #[serde(alias = "The Moles Welcome You")]
    TheMolesWelcomeYou,
    #[serde(alias = "The Birth of the Self")]
    TheBirthoftheSelf,
    #[serde(alias = "Eyes of the Prey")]
    EyesOfThePrey,
    #[serde(alias = "Landau's Choice")]
    LandausChoice,
    Swordplay,
    #[serde(alias = "Planetary Rendezvous")]
    PlanetaryRendezvous,
    #[serde(alias = "A Secret Vow")]
    ASecretVow,
    #[serde(alias = "Make the World Clamor")]
    MakeTheWorldClamor,
    #[serde(alias = "Perfect Timing")]
    PerfectTiming,
    #[serde(alias = "Resolution Shines As Pearls of Sweat")]
    ResolutionShinesAsPearlsOfSweat,
    #[serde(alias = "Trend of the Universal Market")]
    TrendOfTheUniversalMarket,
    #[serde(alias = "Subscribe for More!")]
    SubscribeForMore,
    #[serde(alias = "Dance! Dance! Dance!")]
    DanceDanceDance,
    #[serde(alias = "Under the Blue Sky")]
    UnderTheBlueSky,
    #[serde(alias = "Geniuses' Repose")]
    GeniusesRepose,
    #[serde(alias = "Quid Pro Quo")]
    QuidProQuo,
    Fermata,
    #[serde(alias = "We Are Wildfire")]
    WeAreWildfire,
    #[serde(alias = "River Flows in Spring")]
    RiverFlowsInSpring,
    #[serde(alias = "Woof! Walk Time!")]
    WoofWalkTime,
    #[serde(alias = "The Seriousness of Breakfast")]
    TheSeriousnessOfBreakfast,
    #[serde(alias = "armth Shortens Cold Nights")]
    WarmthShortensColdNights,
    #[serde(alias = "We Will Meet Again")]
    WeWillMeetAgain,
    #[serde(alias = "his Is Me!")]
    ThisIsMe,
    #[serde(alias = "Return to Darkness")]
    ReturnToDarkness,
    #[serde(alias = "Carve the Moon, Weave the Clouds")]
    CarveTheMoonWeaveTheClouds,
    #[serde(alias = "Nowhere to Run")]
    NowhereToRun,
    #[serde(alias = "Today Is Another Peaceful Day")]
    TodayIsAnotherPeacefulDay,
    #[serde(alias = "Before the Tutorial Mission Starts")]
    BeforeTheTutorialMissionStarts,
    #[serde(alias = "Night on the Milky Way")]
    NightOnTheMilkyWay,
    #[serde(alias = "In the Night")]
    InTheNight,
    #[serde(alias = "Something Irreplaceable")]
    SomethingIrreplaceable,
    #[serde(alias = "But the Battle Isn't Over")]
    ButTheBattleIsntOver,
    #[serde(alias = "In the Name of the World")]
    InTheNameOfTheWorld,
    #[serde(alias = "Moment of Victory")]
    MomentOfVictory,
    #[serde(alias = "Patience Is All You Need")]
    PatienceIsAllYouNeed,
    #[serde(alias = "Incessant Rain")]
    IncessantRain,
    #[serde(alias = "Echoes of the Coffin")]
    EchoesOfTheCoffin,
    #[serde(alias = "The Unreachable Side")]
    TheUnreachableSide,
    #[serde(alias = "Before Dawn")]
    BeforeDawn,
    #[serde(alias = "She Already Shut Her Eyes")]
    SheAlreadyShutHerEyes,
    #[serde(alias = "Sleep Like the Dead")]
    SleepLikeTheDead,
    #[serde(alias = "Time Waits for No One")]
    TimeWaitsForNoOne,
    #[serde(alias = "I Shall Be My Own Sword")]
    IShallBeMyOwnSword,
    #[serde(alias = "Brighter Than the Sun")]
    BrighterThanTheSun,
    #[serde(alias = "Worrisome, Blissful")]
    WorrisomeBlissful,
    #[serde(alias = "Past Self in Mirror")]
    PastSelfInMirror,
    #[serde(alias = "Baptism of Pure Thought")]
    BaptismOfPureThought,
    #[serde(alias = "On the Fall of an Aeon")]
    OnTheFallOfAnAeon,
    #[serde(alias = "Cruising in the Stellar Sea")]
    CruisingInTheStellarSea,
    #[serde(alias = "Texture of Memories")]
    TextureOfMemories,
    #[serde(alias = "Past and Future")]
    PastAndFuture,
    #[serde(alias = "Night of Fright")]
    NightOfFright,
    #[serde(alias = "An Instant Before A Gaze")]
    AnInstantBeforeAGaze,
    #[serde(alias = "Hey, Over Here")]
    HeyOverHere,
    #[serde(alias = "Solitary Healing")]
    SolitaryHealing,
    #[serde(alias = "Shared Feeling")]
    SharedFeeling,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LightCone {
    pub key: LightConeName,
    pub level: u8,
    pub ascension: u8,
    pub superimposition: u8,
    pub location: Option<CharacterName>,
    pub lock: bool,
    pub _id: String,
    #[serde(skip_deserializing)]
    pub light_cone_stats: LightConeStats,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LightConeStats {
    pub hp: f64,
    pub atk: f64,
    pub def: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectYattaLightConeResponse {
    response: u64,
    data: ProjectYattaLightConeData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectYattaLightConeData {
    id: u64,
    name: String,
    rank: u8,
    types: ProjectYattaLightConeTypes,
    icon: String,
    #[serde(alias = "isSellable")]
    is_sellable: bool,
    route: String,
    description: String,
    upgrade: Vec<ProjectYattaLightConeUpgrade>,
    skill: ProjectYattaLightConeSkill,
    ascension: HashMap<String, u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectYattaLightConeTypes {
    #[serde(alias = "pathType")]
    path_type: ProjectYattaCharacterType,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProjectYattaLightConeUpgrade {
    level: u8,
    #[serde(alias = "costItems")]
    cost_items: Option<HashMap<String, u64>>,
    #[serde(alias = "maxLevel")]
    max_level: u8,
    #[serde(alias = "playerLevelRequire")]
    player_level_require: Option<u8>,
    #[serde(alias = "worldLevelRequire")]
    world_level_require: Option<u8>,
    #[serde(alias = "skillBase")]
    skill_base: ProjectYattaLightConeSkillBase,
    #[serde(alias = "skillAdd")]
    skill_add: ProjectYattaLightConeSkillAdd,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProjectYattaLightConeSkillBase {
    #[serde(alias = "attackBase")]
    attack_base: f64,
    #[serde(alias = "defenceBase")]
    defence_base: f64,
    #[serde(alias = "hPBase")]
    hp_base: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProjectYattaLightConeSkillAdd {
    #[serde(alias = "attackAdd")]
    attack_add: f64,
    #[serde(alias = "defenceAdd")]
    defence_add: f64,
    #[serde(alias = "hPAdd")]
    hp_add: f64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProjectYattaLightConeSkill {
    name: String,
    description: String,
    params: HashMap<String, Vec<f64>>,
}

impl LightConeName {
    pub fn get_yatta_id(&self) -> String {
        match self {
            Self::Arrows => "20000",
            Self::Cornucopia => "20001",
            Self::CollapsingSky => "20002",
            Self::Amber => "20003",
            Self::Void => "20004",
            Self::Chorus => "20005",
            Self::DataBank => "20006",
            Self::DartingArrow => "20007",
            Self::FineFruit => "20008",
            Self::ShatteredHome => "20009",
            Self::Defense => "20010",
            Self::Loop => "20011",
            Self::MeshingCogs => "20012",
            Self::Passkey => "20013",
            Self::Adversarial => "20014",
            Self::Multiplication => "20015",
            Self::MutualDemise => "20016",
            Self::Pioneering => "20017",
            Self::HiddenShadow => "20018",
            Self::Mediation => "20019",
            Self::Sagacity => "20020",
            Self::PostOpConversation => "21000",
            Self::GoodNightAndSleepWell => "21001",
            Self::DayOneOfMyNewLife => "21002",
            Self::OnlySilenceRemains => "21003",
            Self::MemoriesOfThePast => "21004",
            Self::TheMolesWelcomeYou => "21005",
            Self::TheBirthoftheSelf => "21006",
            Self::SharedFeeling => "21007",
            Self::EyesOfThePrey => "21008",
            Self::LandausChoice => "21009",
            Self::Swordplay => "21010",
            Self::PlanetaryRendezvous => "21011",
            Self::ASecretVow => "21012",
            Self::MakeTheWorldClamor => "21013",
            Self::PerfectTiming => "21014",
            Self::ResolutionShinesAsPearlsOfSweat => "21015",
            Self::TrendOfTheUniversalMarket => "21016",
            Self::SubscribeForMore => "21017",
            Self::DanceDanceDance => "21018",
            Self::UnderTheBlueSky => "21019",
            Self::GeniusesRepose => "21020",
            Self::QuidProQuo => "21021",
            Self::Fermata => "21022",
            Self::WeAreWildfire => "21023",
            Self::RiverFlowsInSpring => "21024",
            Self::PastAndFuture => "21025",
            Self::WoofWalkTime => "21026",
            Self::TheSeriousnessOfBreakfast => "21027",
            Self::WarmthShortensColdNights => "21028",
            Self::WeWillMeetAgain => "21029",
            Self::ThisIsMe => "21030",
            Self::ReturnToDarkness => "21031",
            Self::CarveTheMoonWeaveTheClouds => "21032",
            Self::NowhereToRun => "21033",
            Self::TodayIsAnotherPeacefulDay => "21034",
            Self::BeforeTheTutorialMissionStarts => "22000",
            Self::HeyOverHere => "22001",
            Self::NightOnTheMilkyWay => "23000",
            Self::InTheNight => "23001",
            Self::SomethingIrreplaceable => "23002",
            Self::ButTheBattleIsntOver => "23003",
            Self::InTheNameOfTheWorld => "23004",
            Self::MomentOfVictory => "23005",
            Self::PatienceIsAllYouNeed => "23006",
            Self::IncessantRain => "23007",
            Self::EchoesOfTheCoffin => "23008",
            Self::TheUnreachableSide => "23009",
            Self::BeforeDawn => "23010",
            Self::SheAlreadyShutHerEyes => "23011",
            Self::SleepLikeTheDead => "23012",
            Self::TimeWaitsForNoOne => "23013",
            Self::IShallBeMyOwnSword => "23014",
            Self::BrighterThanTheSun => "23015",
            Self::WorrisomeBlissful => "23016",
            Self::NightOfFright => "23017",
            Self::AnInstantBeforeAGaze => "23018",
            Self::PastSelfInMirror => "23019",
            Self::BaptismOfPureThought => "23020",
            Self::OnTheFallOfAnAeon => "24000",
            Self::CruisingInTheStellarSea => "24001",
            Self::TextureOfMemories => "24002",
            Self::SolitaryHealing => "24003",
        }
        .to_string()
    }
}

impl LightCone {
    pub async fn get_main_stat(&mut self) -> eyre::Result<LightConeStats> {
        if self.light_cone_stats.atk != 0.0 {
            return Ok(self.light_cone_stats.clone());
        }
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "https://api.yatta.top/hsr/v2/en/equipment/{}",
                self.key.get_yatta_id()
            ))
            .send()
            .await?
            .text()
            .await?;
        let character_data: ProjectYattaLightConeResponse = serde_json::from_str(&response)?;
        let upgrade_data = character_data.data.upgrade;
        let base_atk = upgrade_data[self.ascension as usize].skill_base.attack_base
            + (self.level - 1) as f64 * upgrade_data[0].skill_add.attack_add;
        let base_def = upgrade_data[self.ascension as usize]
            .skill_base
            .defence_base
            + (self.level - 1) as f64 * upgrade_data[0].skill_add.defence_add;
        let base_hp = upgrade_data[self.ascension as usize].skill_base.hp_base
            + (self.level - 1) as f64 * upgrade_data[0].skill_add.hp_add;
        self.light_cone_stats = LightConeStats {
            atk: base_atk,
            def: base_def,
            hp: base_hp,
        };
        Ok(self.light_cone_stats.clone())
    }
}
