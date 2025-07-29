use crate::engine::StatBonusMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct CharacterEntity {
    pub base_hp: f64,
    pub base_atk: f64,
    pub base_def: f64,
    pub base_spd: f64,
    pub _base_aggro: u64,
    pub critical_chance: f64,
    pub critical_damage: f64,
    pub stat_bonus: StatBonusMap,
    pub _character: Character,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub path: Path,
    pub level: u8,
    pub ascension: u8,
    pub eidolon: u8,
    pub skills: CharacterSkills,
    pub traces: CharacterTraces,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterSkills {
    pub basic: u8,
    pub skill: u8,
    pub ult: u8,
    pub talent: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterTraces {
    pub ability_1: bool,
    pub ability_2: bool,
    pub ability_3: bool,
    pub stat_1: bool,
    pub stat_2: bool,
    pub stat_3: bool,
    pub stat_4: bool,
    pub stat_5: bool,
    pub stat_6: bool,
    pub stat_7: bool,
    pub stat_8: bool,
    pub stat_9: bool,
    pub stat_10: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Path {
    #[serde(alias = "The Hunt", alias = "hunt")]
    TheHunt,
    #[serde(alias = "harmony")]
    Harmony,
    #[serde(alias = "preservation")]
    Preservation,
    #[serde(alias = "abundance")]
    Abundance,
    #[serde(alias = "nihility")]
    Nihility,
    #[serde(alias = "destruction")]
    Destruction,
    #[serde(alias = "erudition")]
    Erudition,
}

impl CharacterTraces {
    pub fn get_stat(&self, index: usize) -> bool {
        match index {
            1 => self.stat_1,
            2 => self.stat_2,
            3 => self.stat_3,
            4 => self.stat_4,
            5 => self.stat_5,
            6 => self.stat_6,
            7 => self.stat_7,
            8 => self.stat_8,
            9 => self.stat_9,
            10 => self.stat_10,
            _ => false, // or handle invalid indices as needed
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum AttackType {
    Lightning,
    Physical,
    Wind,
    Fire,
    Ice,
    Imaginary,
    Quantum,
}

#[derive(PartialEq, Eq)]
pub enum SkillType {
    BasicAttack,
    Skill,
    Ultimate,
    FollowUpAttack,
}

#[derive(PartialEq, Eq)]
pub enum DamageType {
    Normal,
    RealDamage,
    ExtraDamage,
    DamageOnTime,
    BreakDamage,
    SuperBreakDamage,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum CritEnum {
    Crit,
    NoCrit,
    Avg,
}
