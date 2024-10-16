// Import necessary modules and crates
use crate::domain::{CharacterEntity, LightConeEntity, Relic, Stats};
use eval::Expr;
use eyre::{OptionExt, Result};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use strum::IntoEnumIterator;

// Type aliases for complex data structures
type SetBonus = HashMap<u8, Vec<(Stats, f64, Option<(Stats, f64)>)>>;
// `SetBonus` maps a number (u8) to a vector of tuples. Each tuple contains:
// - `Stats`: The stat affected by the bonus.
// - `f64`: The value of the bonus.
// - `Option<(Stats, f64)>`: Optional condition bonus (stat and value).

pub type SetBonusMap = HashMap<String, SetBonus>;
// `SetBonusMap` maps `RelicSetName` to `SetBonus`, representing set bonuses for different relic sets.

pub type StatBonusMap = HashMap<Stats, f64>;
// `StatBonusMap` maps `Stats` to `f64`, representing various stat bonuses.

type Bonus = (Stats, f64, Option<ConditionBonus>);
// `Bonus` represents a bonus with:
// - `Stats`: The stat affected by the bonus.
// - `f64`: The value of the bonus.
// - `Option<ConditionBonus>`: Optional condition bonus.

type ConditionBonus = (Stats, f64);
// `ConditionBonus` represents a conditional bonus with:
// - `Stats`: The condition stat.
// - `f64`: The bonus value if the condition is met.

/// Represents an evaluator for calculating and assessing character stats in a game.
/// This struct holds various components such as the character, light cone, bonuses, and formulas
/// required for evaluating and calculating the target stat based on equipped relics and other factors.
#[derive(Clone)]
pub struct Evaluator {
    pub character: CharacterEntity, // The character being evaluated, containing base stats and other attributes.
    pub light_cone: LightConeEntity, // The light cone equipped by the character, affecting stats and bonuses.
    pub constraint: StatBonusMap, // A map of constraints for stat bonuses to be considered during evaluation.
    pub set_bonus: SetBonusMap, // A map of bonuses from relic sets, indicating bonuses applied based on equipped sets.
    pub other_bonus: StatBonusMap, // A map of additional stat bonuses not related to relic sets.
    pub base_stats_formulas: HashMap<Stats, String>, // Formulas for calculating base stats of the character and light cone.
    pub target_formula: String, // Formula used to calculate the final target stat based on all bonuses and base stats.
    pub target_name: String,    // Name of the target stat to be calculated and evaluated.
}

impl Evaluator {
    /// Creates a new instance of `Evaluator` with the given parameters.
    ///
    /// # Parameters
    ///
    /// - `character`: The `Character` instance representing the character being evaluated.
    /// - `light_cone`: The `LightCone` instance representing the light cone equipped by the character.
    /// - `constraint`: A `StatBonusMap` containing constraints for stat bonuses.
    /// - `set_bonus`: A `SetBonusMap` containing bonuses from relic sets.
    /// - `other_bonus`: A `StatBonusMap` containing additional stat bonuses.
    /// - `base_stats_formulas`: A `HashMap` mapping `Stats` to their respective formulas for base stats calculation.
    /// - `target_formula`: A string representing the formula used to calculate the final target stat.
    /// - `target_name`: A string representing the name of the target stat.
    ///
    /// # Returns
    ///
    /// Returns a new `Evaluator` instance initialized with the provided parameters.
    #[allow(clippy::too_many_arguments)] // Suppresses the warning for having too many arguments in the constructor
    pub fn new(
        character: CharacterEntity,
        light_cone: LightConeEntity,
        constraint: StatBonusMap,
        set_bonus: SetBonusMap,
        other_bonus: StatBonusMap,
        base_stats_formulas: HashMap<Stats, String>,
        target_formula: &str,
        target_name: &str,
    ) -> Self {
        Self {
            character,
            light_cone,
            constraint,
            set_bonus,
            other_bonus,
            base_stats_formulas,
            target_formula: target_formula.to_owned(),
            target_name: target_name.to_owned(),
        }
    }

    /// Calculates the total value of each stat from the given relics.
    ///
    /// This function iterates over all possible stats and computes the total value for each stat
    /// based on the relics provided. It aggregates the values from the main stats and substats
    /// of each relic to compute the total value for each stat.
    ///
    /// # Parameters
    ///
    /// - `relics`: A slice of `Relic` instances representing the relics equipped by the character.
    ///   Each relic may contribute to the total value of different stats based on its main stat
    ///   and substats.
    ///
    /// # Returns
    ///
    /// Returns a `Result<HashMap<Stats, f64>>`, where the `HashMap` maps each `Stats` variant to
    /// its total calculated value based on the relics. If an error occurs during calculation, the
    /// function returns an `Err`.
    fn calculate_stat_total_from_relics(&self, relics: &[Relic]) -> Result<HashMap<Stats, f64>> {
        // Convert the iterator to a vector
        let stats: Vec<Stats> = Stats::iter().collect();

        // Use rayon's parallel iterator to process each stat in parallel
        let maps: Vec<HashMap<Stats, f64>> = stats
            .into_par_iter()
            .map(|stat| {
                // Calculate the total value for the current stat
                let total = relics
                    .iter()
                    .map(|relic| self.relic_stat_value(relic, stat.clone()))
                    .sum::<f64>();

                // Lock the mutex and update the HashMap
                HashMap::from([(stat, total)])
            })
            .collect();

        // Retrieve the final HashMap from the Arc<Mutex<_>>
        let totals = maps.into_par_iter().flatten().collect();

        Ok(totals)
    }

    /// Calculates the total value of a specific stat from a given relic.
    ///
    /// This function computes the value of a specific stat by summing up the main stat value
    /// and any substat values associated with the relic. If the relic's main stat matches the
    /// queried stat, the main stat's value is obtained and added to the sum of relevant substats.
    ///
    /// # Parameters
    ///
    /// - `relic`: A reference to a `Relic` instance from which the stat value is calculated.
    /// - `stat`: The `Stats` enum variant representing the stat whose value is being calculated.
    ///
    /// # Returns
    ///
    /// Returns a `f64` value representing the total value of the specified stat for the given relic.
    fn relic_stat_value(&self, relic: &Relic, stat: Stats) -> f64 {
        // Calculate the main stat value
        let mainstat_value = if relic.mainstat == stat {
            relic.get_mainstat()
        } else {
            f64::default()
        };

        // Calculate the sum of substat values for the specified stat in parallel
        let substat_values: f64 = relic
            .substats
            .iter() // Use parallel iterator
            .filter_map(|s| if s.key == stat { Some(s.value) } else { None })
            .sum();

        // Calculate the total value including both main stat and substats
        mainstat_value + substat_values
    }

    /// Applies the bonuses from relic sets to the total stats.
    ///
    /// This function calculates and applies bonuses based on the relic sets equipped. It
    /// determines the count of relics in each set and looks up the corresponding bonuses
    /// from the `set_bonus` map. These bonuses are then applied to the total stats. If the
    /// set bonuses include conditions, they are evaluated based on the first round's stats.
    ///
    /// # Parameters
    ///
    /// - `relics`: A slice of `Relic` instances representing the relics equipped.
    /// - `first_round_stats`: An optional reference to a `HashMap` containing the base stats
    ///   from the first round of calculations. This is used to apply conditional bonuses.
    ///
    /// # Returns
    ///
    /// Returns a `Result<HashMap<Stats, f64>>` where the `HashMap` contains the updated stats
    /// with the set bonuses applied.
    fn apply_set_bonus(
        &self,
        relics: &[Relic],
        first_round_stats: Option<&HashMap<Stats, f64>>,
    ) -> Result<HashMap<Stats, f64>> {
        // Create a thread-safe HashMap to store totals
        let mut totals = HashMap::new();

        // Count the number of relics in each set
        let counts: HashMap<String, usize> = relics.iter().counts_by(|relic| relic.set_id.clone());

        // Use Rayon to process each set in parallel
        let partial_totals: Vec<HashMap<Stats, f64>> = counts
            .into_iter()
            .map(|(set, count)| {
                Ok::<HashMap<Stats, f64>, eyre::Report>(
                    if let Some(bonuses) =
                        self.set_bonus.get(&set).and_then(|s| s.get(&(count as u8)))
                    {
                        self.apply_bonuses(bonuses, first_round_stats)?
                    } else {
                        HashMap::new()
                    },
                )
            })
            .collect::<Result<Vec<HashMap<Stats, f64>>>>()?;

        for total in partial_totals {
            for (key, val) in total {
                *totals.entry(key).or_default() += val;
            }
        }

        Ok(totals.clone()) // Return the final results
    }

    /// Applies bonuses to the total stats, considering conditional bonuses.
    ///
    /// This function updates the `totals` with bonuses provided. It handles both unconditional
    /// bonuses and conditional bonuses, which are applied only if certain conditions based on
    /// the first round of stats are met.
    ///
    /// # Parameters
    ///
    /// - `totals`: A mutable reference to a `HashMap` where the bonuses will be applied.
    /// - `bonuses`: A slice of `Bonus` tuples where each tuple contains:
    ///   - `stat`: The stat to which the bonus should be applied.
    ///   - `bonus_value`: The amount of the bonus to apply.
    ///   - `condition_bonus`: An optional tuple containing:
    ///     - `cond_stat`: The stat upon which the condition is based.
    ///     - `cond_bonus`: The bonus to apply if the condition is met.
    /// - `first_round_stats`: An optional reference to a `HashMap` of base stats from the first
    ///   round of calculations. Used to evaluate conditions for conditional bonuses.
    ///
    /// # Returns
    ///
    /// Returns a `Result` which is `Ok` if the bonuses were applied successfully, or an error
    /// if any condition is not met.
    fn apply_bonuses(
        &self,
        bonuses: &[Bonus],
        first_round_stats: Option<&HashMap<Stats, f64>>,
    ) -> Result<HashMap<Stats, f64>> {
        // Process bonuses in parallel
        let partial_totals: Vec<HashMap<Stats, f64>> = bonuses
            .iter()
            .map(|(stat, bonus_value, condition_bonus)| {
                let mut totals = HashMap::new();

                // Apply unconditional bonus
                *totals.entry(stat.clone()).or_default() += bonus_value;

                // Check and apply conditional bonus if applicable
                if let Some((cond_stat, cond_bonus)) = condition_bonus {
                    let stat_value = first_round_stats
                        .and_then(|fs| fs.get(cond_stat).cloned())
                        .unwrap_or_default();
                    if stat_value > *cond_bonus {
                        *totals.entry(stat.clone()).or_default() += cond_bonus;
                    }
                }
                Ok::<HashMap<Stats, f64>, eyre::Report>(totals)
            })
            .collect::<Result<Vec<HashMap<Stats, f64>>>>()?;

        // Update the original totals with the results from the parallel processing
        let mut totals = HashMap::new();
        for total in partial_totals {
            for (key, val) in total {
                *totals.entry(key).or_default() += val;
            }
        }

        Ok(totals)
    }

    /// Builds an expression by substituting values from the `formula` string and `totals` map.
    ///
    /// This function creates an `Expr` object initialized with a given formula and populates it
    /// with values for character stats, light cone stats, and additional stat bonuses.
    ///
    /// # Parameters
    ///
    /// - `formula`: A string containing the formula to evaluate.
    /// - `totals`: A map of stats and their corresponding values to substitute in the formula.
    ///
    /// # Returns
    ///
    /// An `Expr` object with all the substitutions applied.
    ///
    /// # Notes
    ///
    /// - The function uses specific keys for different stats, translating them into the appropriate
    ///   format required by the expression evaluation library.
    /// - Default values for base crit rate and crit damage are included.
    fn build_expr(&self, formula: &str, totals: &HashMap<Stats, f64>) -> Expr {
        // Create a new expression from the given formula
        let mut expr = Expr::new(formula);

        // Substitute values for character stats
        expr = expr
            .value("Character_HP", self.character.base_hp)
            .value("Character_ATK", self.character.base_atk)
            .value("Character_DEF", self.character.base_def)
            .value("Character_SPD", self.character.base_spd);

        // Substitute values for light cone stats
        expr = expr
            .value("LightCone_HP", self.light_cone.base_hp)
            .value("LightCone_ATK", self.light_cone.base_atk)
            .value("LightCone_DEF", self.light_cone.base_def);

        // Use Rayon to process totals in parallel
        let results: Vec<(String, f64)> = totals
            .iter()
            .filter_map(|(stat, value)| {
                let key = match stat {
                    Stats::Hp => "Additive_HP_Bonus",
                    Stats::Hp_ => "Percentage_HP_Bonus",
                    Stats::Atk => "Additive_ATK_Bonus",
                    Stats::Atk_ => "Percentage_ATK_Bonus",
                    Stats::Def => "Additive_DEF_Bonus",
                    Stats::Def_ => "Percentage_DEF_Bonus",
                    Stats::DefReduction_ => "Percentage_DEF_Reduction",
                    Stats::Spd => "Additive_SPD_Bonus",
                    Stats::Spd_ => "Percentage_SPD_Bonus",
                    Stats::FireDmgBoost_ => "Fire DMG_Boost",
                    Stats::WindDmgBoost_ => "Wind_DMG_Boost",
                    Stats::IceDmgBoost_ => "Ice_DMG_Boost",
                    Stats::LightningDmgBoost_ => "Lightning_DMG_Boost",
                    Stats::PhysicalDmgBoost_ => "Physical_DMG_Boost",
                    Stats::QuantumDmgBoost_ => "Quantum_DMG_Boost",
                    Stats::ImaginaryDmgBoost_ => "Imaginary_DMG_Boost",
                    Stats::DmgBoost_ => "Common_DMG_Boost",
                    Stats::CritRate_ => "CRIT_Rate",
                    Stats::CritDmg_ => "CRIT_DMG",
                    Stats::BreakEffect_ => "Break_Effect",
                    Stats::EffectHitRate_ => "Effect_Hit_Rate",
                    Stats::EffectRes_ => "Effect_RES",
                    Stats::EnergyRegenerationRate_ => "Energy_Regeneration_Rate",
                    Stats::OutgoingHealingBoost_ => "Outgoing_Healing_Boost",
                    Stats::BasicAtkDmgBoost_ => "Basic_ATK_DMG_Boost",
                    Stats::SkillDmgBoost_ => "Skill_DMG_Boost",
                    Stats::UltimateDmgBoost_ => "Ultimate_DMG_Boost",
                    Stats::FollowUpAtkDmgBoost_ => "Follow_Up_ATK_DMG_Boost",
                    Stats::ShieldDmgAbsorption_ => "Shield_DMG_Absorption",
                    Stats::DmgReduction_ => "DMG_Reduction",
                    Stats::DefIgnore_ => "DEF_Ignore",
                    Stats::BreakDmgDefIgnore_ => "Break_DMG_DEF_Ignore",
                    Stats::SuperBreakDmgDefIgnore_ => "Super_Break_DMG_DEF_Ignore",
                    Stats::Dummy => return None,
                };
                Some((key.to_string(), *value))
            })
            .collect();

        // Apply results to the expression
        for (key, value) in results {
            expr = expr.value(&key, value);
        }

        // Substitute default values for base crit rate and crit damage
        expr = expr
            .value("Character_Base_CRIT_Rate", self.character.critical_chance)
            .value("Character_Base_CRIT_DMG", self.character.critical_damage);

        expr
    }

    /// Applies constraints to the given result based on the base statistics.
    ///
    /// This function checks if the values in `base_stats` meet the constraints defined in `self.constraint`.
    /// If any constraint is not met (i.e., the current stat is less than the required value), the result is penalized by negating it.
    ///
    /// # Arguments
    ///
    /// * `result`: The current result to be adjusted based on constraints. This is the value that might be penalized.
    /// * `base_stats`: A map of base statistics where the key is a statistic identifier (`Stats`), and the value is the statistic's value.
    ///
    /// # Returns
    ///
    /// Returns a `Result<f64>` where:
    /// * `Ok(f64)` contains the adjusted result if constraints are met or penalized.
    /// * `Err` if there's a missing statistic in `base_stats` that is required by `self.constraint`.
    ///
    /// # Errors
    ///
    /// Returns an error if a statistic required by the constraints is missing in `base_stats`. The error contains a message indicating which statistic is missing.
    ///
    /// # Example
    ///
    /// ```rust
    /// let constraints = HashMap::new();
    /// constraints.insert(Stats::Health, 100.0);
    /// let result = 50.0;
    /// let base_stats = HashMap::new();
    /// base_stats.insert(Stats::Health, 90.0);
    ///
    /// let adjusted_result = apply_constraints(result, &base_stats);
    /// // adjusted_result would be -50.0 because Health is below the constraint
    /// ```
    ///
    fn apply_constraints(&self, result: f64, base_stats: &HashMap<Stats, f64>) -> Result<f64> {
        // Initialize the adjusted result with the original result.
        let mut adjusted_result = result;

        // Process each constraint in parallel.
        let constraints_satisfied: Vec<bool> = self
            .constraint
            .iter()
            .map(|(stat, required_value)| {
                // Retrieve the current value of the statistic from `base_stats`.
                let current_stat = base_stats
                    .get(stat)
                    .ok_or_eyre(format!("Missing stat {:?}", stat))
                    .unwrap(); // Handle the error appropriately

                // Check if the current statistic value is below the required value.
                current_stat < required_value
            })
            .collect();

        // Penalize the result if any constraints were not satisfied.
        let constraints_not_satisfied = constraints_satisfied
            .iter()
            .any(|&constraint_not_met| constraint_not_met);

        if constraints_not_satisfied {
            adjusted_result = -adjusted_result;
        }

        // Return the potentially adjusted result.
        Ok(adjusted_result)
    }

    /// Builds and evaluates the final expression to obtain the result based on provided totals.
    ///
    /// This function constructs an expression using the target formula and the provided `totals`. It then executes
    /// the expression and parses the result from its output. This result is returned as the final evaluated value.
    ///
    /// # Arguments
    ///
    /// * `totals`: A map of statistics where the key is a statistic identifier (`Stats`), and the value is the statistic's value.
    ///   This map is used to build the final expression by substituting these values into the formula.
    ///
    /// # Returns
    ///
    /// Returns a `Result<f64>` where:
    /// * `Ok(f64)` contains the evaluated result of the final expression.
    /// * `Err` if there is an issue building or executing the expression, or parsing the result.
    ///
    /// # Errors
    ///
    /// Returns an error if the expression cannot be built, executed, or if the result cannot be parsed from the expression's output.
    fn calculate_final_result(&self, totals: &HashMap<Stats, f64>) -> Result<f64> {
        // Build the final expression using the target formula and the provided totals.
        let final_expr = self.build_expr(&self.target_formula, totals);

        // Execute the expression and obtain the result in JSON format.
        let result_value = final_expr.exec()?;

        // Convert the JSON result to a floating-point number.
        let result: f64 = serde_json::from_value(result_value)?;

        // Return the evaluated result.
        Ok(result)
    }

    /// Evaluates the final result based on a list of relics and constraints.
    ///
    /// This function performs a multi-step evaluation process:
    /// 1. Calculates initial totals based on the provided relics.
    /// 2. Evaluates base statistics from the initial totals.
    /// 3. Recalculates totals considering the base statistics.
    /// 4. Evaluates base statistics again from the updated totals.
    /// 5. Builds and executes a final expression based on the updated totals.
    /// 6. Applies constraints to the final result and adjusts it if necessary.
    ///
    /// # Arguments
    ///
    /// * `relics`: A vector of `Relic` objects that are used to calculate initial and updated totals.
    ///
    /// # Returns
    ///
    /// Returns a `Result<f64>` where:
    /// * `Ok(f64)` contains the final evaluated and constrained result.
    /// * `Err` if any step of the evaluation process fails, such as calculation errors, missing data, or constraint violations.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the following occurs:
    /// * Calculation of totals fails.
    /// * Evaluation of base statistics fails.
    /// * Final expression cannot be built, executed, or parsed.
    /// * Constraints cannot be applied due to missing or invalid data.
    pub fn evaluate(&self, relics: Vec<Relic>) -> Result<f64> {
        // Calculate initial totals from the provided relics.
        let initial_totals = self.calculate_totals(&relics, None)?;

        // Evaluate base statistics based on the initial totals.
        let initial_base_stats = self.evaluate_base_stats(&initial_totals)?;

        // Recalculate totals considering the evaluated base statistics.
        let updated_totals = self.calculate_totals(&relics, Some(initial_base_stats))?;

        // Evaluate base statistics again based on the updated totals.
        let updated_base_stats = self.evaluate_base_stats(&updated_totals)?;

        // Build and execute the final expression using the updated totals.
        let final_result = self.calculate_final_result(&updated_totals)?;

        // Apply constraints to the final result and adjust if necessary.
        let constrained_result = self.apply_constraints(final_result, &updated_base_stats)?;

        // Return the final constrained result.
        Ok(constrained_result)
    }

    /// Evaluates base stats using predefined formulas and total stat values.
    ///
    /// This function iterates over the `base_stats_formulas` and evaluates each formula
    /// using the provided total stat values. The results are collected into a `HashMap`
    /// where each base stat is associated with its computed value.
    ///
    /// # Parameters
    ///
    /// - `totals`: A `HashMap` containing the total values of various stats, used as inputs
    ///   for evaluating the formulas.
    ///
    /// # Returns
    ///
    /// - `Result<HashMap<Stats, f64>>`: A `HashMap` where keys are `Stats` and values are
    ///   the computed values for those stats. The result is wrapped in a `Result` to handle
    ///   potential errors during evaluation.
    ///
    /// # Errors
    ///
    /// - May return errors from evaluating the formulas or parsing the result.
    ///
    /// # Notes
    ///
    /// - Each formula is evaluated using the `build_expr` method, and results are parsed
    ///   from JSON using `serde_json`.
    pub fn evaluate_base_stats(&self, totals: &HashMap<Stats, f64>) -> Result<HashMap<Stats, f64>> {
        let result: Result<HashMap<Stats, f64>, _> = self
            .base_stats_formulas
            .par_iter() // Convert to a parallel iterator
            .map(|(stat, formula)| {
                let expr = self.build_expr(formula, totals);
                let value = serde_json::from_value(expr.exec()?)?;
                Ok((stat.clone(), value))
            })
            .collect(); // Collect results into a HashMap

        result
    }

    /// Calculates the total stats, including those from relics and set bonuses.
    ///
    /// This function computes the total stats by combining the stat values from relics with
    /// set bonuses. It first calculates the total stats from relics, then applies any set
    /// bonuses. Finally, it aggregates the bonuses into the total stats.
    ///
    /// # Parameters
    ///
    /// - `relics`: A slice of `Relic` objects that contribute to the total stats.
    /// - `first_round_stats`: An optional `HashMap` of base stats calculated in the first round.
    ///   This is used for applying conditional set bonuses.
    ///
    /// # Returns
    ///
    /// - `Result<HashMap<Stats, f64>>`: A `HashMap` where keys are `Stats` and values are the
    ///   aggregated total values of those stats. The result is wrapped in a `Result` to handle
    ///   potential errors during calculation.
    ///
    /// # Errors
    ///
    /// - May return errors from calculating stats from relics or applying set bonuses.
    ///
    /// # Notes
    ///
    /// - The function first calculates stat totals from relics.
    /// - It then applies set bonuses and adds these bonuses to the totals.
    pub fn calculate_totals(
        &self,
        relics: &[Relic],
        first_round_stats: Option<HashMap<Stats, f64>>,
    ) -> Result<HashMap<Stats, f64>> {
        // Use a thread pool to handle tasks in parallel
        let (base_stats_result, set_bonus_result) = rayon::join(
            || self.calculate_stat_total_from_relics(relics),
            || self.apply_set_bonus(relics, first_round_stats.as_ref()),
        );

        // Handle the results from the parallel tasks
        let mut totals = base_stats_result?;
        let set_bonus_totals = set_bonus_result?;

        // Add set bonuses to the total stats
        for (stat, bonus) in set_bonus_totals {
            *totals.entry(stat).or_default() += bonus;
        }

        // Add set bonuses to the total stats
        for (stat, bonus) in &self.other_bonus {
            *totals.entry(stat.clone()).or_default() += bonus;
        }

        Ok(totals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::project_yatta_client::ProjectYattaClient,
        data_fetcher::{project_yatta_data_fetcher::ProjectYattaDataFetcher, DataFetcher},
        domain::{Character, CharacterSkills, CharacterTraces, LightCone, Slot, SubStats},
    };

    #[tokio::test]
    async fn test_evaluation() -> Result<()> {
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
        // Create a new character instance with specific attributes.
        let fu_xuan = Character {
            id: "1208".to_string(),
            name: "Fu Xuan".to_string(),
            path: crate::domain::Path::Preservation,
            level: 80,
            ascension: 6,
            eidolon: 0,
            skills: CharacterSkills {
                basic: 1,
                skill: 9,
                ult: 9,
                talent: 9,
            },
            traces: CharacterTraces {
                ability_1: true,
                ability_2: true,
                ability_3: true,
                stat_1: true,
                stat_2: true,
                stat_3: false,
                stat_4: true,
                stat_5: true,
                stat_6: true,
                stat_7: false,
                stat_8: true,
                stat_9: true,
                stat_10: false,
            },
        };

        // Create a new LightCone instance with specific attributes.
        let we_are_wild_fire = LightCone {
            id: "21023".to_string(),
            name: "We Are Wildfire".to_string(),
            level: 50,
            ascension: 3,
            superimposition: 0,
            location: None,
            lock: false,
            _uid: "lightcone_100".to_string(),
        };

        // Create Relic instances for different slots.
        let head = Relic {
            set_id: "103".to_string(),
            name: "Knight's Forgiving Casque".to_string(),
            slot: Slot::Head,
            rarity: 5,
            level: 15,
            mainstat: Stats::Hp,
            substats: vec![
                SubStats {
                    key: Stats::Atk,
                    value: 38.0,
                },
                SubStats {
                    key: Stats::Def,
                    value: 21.0,
                },
                SubStats {
                    key: Stats::Hp_,
                    value: 13.8,
                },
                SubStats {
                    key: Stats::Atk_,
                    value: 7.3,
                },
            ],
            location: None,
            lock: false,
            discard: false,
            _uid: "relic_1".to_string(),
        };

        let hands = Relic {
            set_id: "113".to_string(),
            name: "Disciple's Ingenium Hand".to_string(),
            slot: Slot::Hands,
            rarity: 5,
            level: 15,
            mainstat: Stats::Atk,
            substats: vec![
                SubStats {
                    key: Stats::Hp_,
                    value: 8.2,
                },
                SubStats {
                    key: Stats::Atk_,
                    value: 7.7,
                },
                SubStats {
                    key: Stats::CritDmg_,
                    value: 11.6,
                },
                SubStats {
                    key: Stats::EffectHitRate_,
                    value: 7.3,
                },
            ],
            location: None,
            lock: false,
            discard: false,
            _uid: "relic_2".to_string(),
        };

        let body = Relic {
            set_id: "113".to_string(),
            name: "Disciple's Dewy Feather Garb".to_string(),
            slot: Slot::Body,
            rarity: 5,
            level: 15,
            mainstat: Stats::Hp_,
            substats: vec![
                SubStats {
                    key: Stats::Hp,
                    value: 80.0,
                },
                SubStats {
                    key: Stats::Atk,
                    value: 16.0,
                },
                SubStats {
                    key: Stats::Def,
                    value: 21.0,
                },
                SubStats {
                    key: Stats::Def_,
                    value: 19.9,
                },
            ],
            location: None,
            lock: false,
            discard: false,
            _uid: "relic_3".to_string(),
        };

        let feet = Relic {
            set_id: "103".to_string(),
            name: "Knight's Iron Boots of Order".to_string(),
            slot: Slot::Feet,
            rarity: 5,
            level: 15,
            mainstat: Stats::Spd,
            substats: vec![
                SubStats {
                    key: Stats::Atk,
                    value: 19.0,
                },
                SubStats {
                    key: Stats::Hp_,
                    value: 7.3,
                },
                SubStats {
                    key: Stats::Atk_,
                    value: 4.3,
                },
                SubStats {
                    key: Stats::EffectHitRate_,
                    value: 16.4,
                },
            ],
            location: None,
            lock: false,
            discard: false,
            _uid: "relic_4".to_string(),
        };

        let sphere = Relic {
            set_id: "302".to_string(),
            name: "The Xianzhou Luofu's Celestial Ark".to_string(),
            slot: Slot::PlanarSphere,
            rarity: 5,
            level: 15,
            mainstat: Stats::Hp_,
            substats: vec![
                SubStats {
                    key: Stats::Def_,
                    value: 9.7,
                },
                SubStats {
                    key: Stats::EffectHitRate_,
                    value: 7.7,
                },
                SubStats {
                    key: Stats::EffectRes_,
                    value: 12.0,
                },
                SubStats {
                    key: Stats::BreakEffect_,
                    value: 5.8,
                },
            ],
            location: None,
            lock: false,
            discard: false,
            _uid: "relic_5".to_string(),
        };

        let rope = Relic {
            set_id: "302".to_string(),
            name: "The Xianzhou Luofu's Ambrosial Arbor Vines".to_string(),
            slot: Slot::LinkRope,
            rarity: 5,
            level: 15,
            mainstat: Stats::Hp_,
            substats: vec![
                SubStats {
                    key: Stats::Def,
                    value: 35.0,
                },
                SubStats {
                    key: Stats::Def_,
                    value: 4.3,
                },
                SubStats {
                    key: Stats::Spd,
                    value: 6.0,
                },
                SubStats {
                    key: Stats::CritRate_,
                    value: 5.8,
                },
            ],
            location: None,
            lock: false,
            discard: false,
            _uid: "relic_6".to_string(),
        };

        // Combine all relics into a vector for evaluation.
        let relics = vec![head, hands, body, feet, sphere, rope];

        // Add base stats to the character and fetch the main stat of the light cone.
        let fu_xuan = fetcher.fetch_character_data(&fu_xuan).await?;
        let we_are_wild_fire = fetcher.fetch_light_cone_data(&we_are_wild_fire).await?;

        // Clone the trace bonuses from the character.
        let trace_bonus = fu_xuan.stat_bonus.clone();

        // Define set bonuses for relic sets.
        let set_bonus = HashMap::from([
            (
                "103".to_string(),
                HashMap::from([(2, vec![(Stats::Def_, 15.0, None)])]),
            ),
            (
                "113".to_string(),
                HashMap::from([(2, vec![(Stats::Hp_, 12.0, None)])]),
            ),
            (
                "302".to_string(),
                HashMap::from([(2, vec![(Stats::Hp_, 12.0, None)])]),
            ),
        ]);

        // Define the formulas used for calculations.
        let hp_formula =
            "(Character_HP + LightCone_HP) * (1 + Percentage_HP_Bonus / 100) + Additive_HP_Bonus";
        let atk_formula =
            "(Character_ATK + LightCone_ATK) * (1 + Percentage_ATK_Bonus / 100) + Additive_ATK_Bonus";
        let def_formula =
            "(Character_DEF + LightCone_DEF) * (1 + (Percentage_DEF_Bonus - Percentage_DEF_Reduction) / 100) + Additive_DEF_Bonus";
        let spd_formula = "Character_SPD * (1 + Percentage_SPD_Bonus / 100) + Additive_SPD_Bonus";
        let crit_rate_formula = "Character_Base_CRIT_Rate + CRIT_Rate";
        let crit_dmg_formula = "Character_Base_CRIT_DMG + CRIT_DMG";
        let energy_regen_rate_formula = "Energy_Regeneration_Rate";
        let effect_hit_rate_formula = "Effect_Hit_Rate";
        let effect_res_formula = "Effect_RES";
        let break_effect_formula = "Break_Effect";

        // Create an Evaluator instance with the defined attributes and formulas.
        let evaluator = Evaluator::new(
            fu_xuan.clone(),
            we_are_wild_fire.clone(),
            HashMap::new(),
            set_bonus.clone(),
            trace_bonus.clone(),
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
        );

        // Evaluate the result with the current relics setup and check if it matches the expected value.
        let result = evaluator.evaluate(relics.clone())?;
        assert_eq!(result, 6512.039936000001);

        // Create a new Evaluator instance with a constraint on EnergyRegenerationRate_.
        let evaluator = Evaluator::new(
            fu_xuan,
            we_are_wild_fire,
            HashMap::from([(Stats::EnergyRegenerationRate_, 160.0)]), // Expecting no more than 160 energy regeneration rate
            set_bonus,
            trace_bonus,
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
        );

        // Evaluate the result with the new setup and check if it matches the expected penalized value.
        let result = evaluator.evaluate(relics)?;
        assert_eq!(result, -6512.039936000001);

        Ok(())
    }
}
