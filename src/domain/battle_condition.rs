#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BattleConditionEnum {
    BeforeFirstAttack,
    AfterUsingSkill,
    AfterUsingUltimate,
    AfterWearerAttack {
        number_of_times: u64,
    },
    AfterWearerIsHit {
        number_of_times: u64,
    },
    AfterWearerHpConsumedByAllyOrThemselves {
        number_of_times: u64,
    },
    AfterAttackingDebuffedEnemy,
    AfterWearerInflictingDebuffs,
    AfterFollowUpAtkDealtDmg {
        number_of_hit_in_one_move: u64,
        number_of_times_allies_used_follow_up_atk: u64,
    },
    WhenEnemyDefeated {
        number_of_enemies_defeated: u64,
    },
    WhenAttackingImprisonedEnemy,
    WhenAttackingEnemyWithDot {
        number_of_dots_enemy_has: u64,
    },
    WhenAttackingEnemyWithDebuff {
        number_of_debuffs_enemy_has: u64,
    },
    EnemyHasQuantumWeakness,
    EnemyHasFireWeakness,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CritEnum {
    Crit,
    NoCrit,
    Avg,
}
