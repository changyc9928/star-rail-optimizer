#[derive(Debug, Clone, PartialEq)]
pub enum BattleConditionEnum {
    MemospriteTurnSinceSummoned {
        number_of_turns: u64,
    },
    NumberOfTurnsSinceStartOfBattle {
        number_of_turns: u64,
    },
    BeforeFirstAttack,
    AfterUsingSkill {
        number_of_turns_since_using_the_skill: u64,
    },
    AfterUsingUltimate {
        number_of_turns_since_using_ultimate: u64,
        next_attack_after_ultimate: bool,
        next_skill_after_ultimate: bool,
    },
    AfterWearerAttack {
        number_of_times: u64,
    },
    AfterWearerIsHit {
        number_of_times: u64,
        within_number_of_turns: u64,
    },
    WhenWearerLosingHp {
        number_of_times: u64,
        within_number_of_turns: u64,
        percentage_of_hp_lost_in_one_turn: f64,
    },
    WhenWearerGainingHp {
        within_number_of_turns: u64,
    },
    WhenMemospriteIsOnField {
        on_field: bool,
    },
    AfterAttackingDebuffedEnemy,
    AfterWearerInflictingDebuffs {
        number_of_times: u64,
        within_number_of_turns: u64,
    },
    AfterFollowUpAtkDealtDmg {
        number_of_hit_in_one_move: u64,
        number_of_times_allies_used_follow_up_atk: u64,
    },
    AfterDealingBreakDmg {
        within_number_of_turns: u64,
    },
    AfterEnemyDefeated {
        number_of_enemies_defeated: u64,
        number_of_turns_after_enemy_defeated: u64,
    },
    WhenAttackingImprisonedEnemy,
    WhenAttackingEnemyWithDot {
        number_of_dots_enemy_has: u64,
        within_number_of_turns: u64,
        debuffs: Vec<DebuffStack>,
    },
    WhenAttackingSlowedDownEnemy,
    WhenAttackingReducedDefEnemy,
    WhenAttackingEnemyWithDebuff {
        number_of_debuffs_enemy_has: u64,
        within_number_of_turns: u64,
    },
    EnemyHasQuantumWeakness,
    EnemyHasFireWeakness,
    WhenAttackingEnemyWithCorrespondingWeakness {
        number_of_enemies: u64,
        within_number_of_turns: u64,
    },
    TeammatesSamePathWithWearer {
        number_of_teammates_having_same_path: u64,
    },
    WearerSummonOnField,
    HittingEnemyWithCrimsonKnot {
        number_of_crinsom_knot_enemy_has: u64,
    },
    CriticalHit(CritEnum),
    ToughnessBreak(bool),
    AfterHittingEnemyWithCrinsomKnot {
        number_of_times: u64,
    },
    WearerCurrentHpPercentage {
        percentage: f64,
    },
    TargetEnemyCurrentHpPercentage {
        percentage: f64,
    },
    NumberOfCharactersHavingShield {
        number_of_characters: u64,
    },
    CarveTheMoonWeaveTheCloudEffect {
        effect: CarveTheMoonWeaveTheCloudEffect,
    },
    NumberOfCriticalAttackDealtToEnemiesBeforeTurnEnds {
        number_of_crits: u64,
    },
    EnemyEnsnared {
        within_number_of_turns: u64,
    },
    WearerEnergyReachesMax,
    AttackingSameEnemy {
        number_of_times: u64,
    },
    EnemyHpPercentHigherThanWearerHpPercent(bool),
    NumberOfEnemiesOnField(u8),
    Mischievous(u64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CritEnum {
    Crit,
    NoCrit,
    Avg,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CarveTheMoonWeaveTheCloudEffect {
    Atk,
    CritDmg,
    EnergyRegen,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DebuffStack {
    pub debuff: String,
    pub stack: u64,
}
