// Import necessary modules and crates
use crate::domain::{Character, LightCone, Relic, RelicSetName, Stats};
use eval::Expr;
use eyre::{OptionExt, Result};
use itertools::Itertools;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use tracing::{debug, info, trace};

// Constants for base critical rate and damage
const BASE_CRIT_RATE: f64 = 5.0; // Base critical rate in percentage
const BASE_CRIT_DMG: f64 = 50.0; // Base critical damage in percentage

// Type aliases for complex data structures
type SetBonus = HashMap<u8, Vec<(Stats, f64, Option<(Stats, f64)>)>>;
// `SetBonus` maps a number (u8) to a vector of tuples. Each tuple contains:
// - `Stats`: The stat affected by the bonus.
// - `f64`: The value of the bonus.
// - `Option<(Stats, f64)>`: Optional condition bonus (stat and value).

type SetBonusMap = HashMap<RelicSetName, SetBonus>;
// `SetBonusMap` maps `RelicSetName` to `SetBonus`, representing set bonuses for different relic sets.

type StatBonusMap = HashMap<Stats, f64>;
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
    pub character: Character, // The character being evaluated, containing base stats and other attributes.
    pub light_cone: LightCone, // The light cone equipped by the character, affecting stats and bonuses.
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
        character: Character,
        light_cone: LightCone,
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
        trace!("Starting calculation of stat totals from relics");

        let mut totals = HashMap::new();
        for stat in Stats::iter() {
            // Log the stat being processed
            trace!("Processing stat: {:?}", stat);

            // Calculate the total value for the current stat
            let total = relics
                .iter()
                .map(|relic| {
                    let value = self.relic_stat_value(relic, stat.clone());
                    // Log the individual relic's contribution to the stat
                    trace!(
                        "Relic ID: {}, Stat: {:?}, Value: {}",
                        relic._id,
                        stat,
                        value
                    );
                    value
                })
                .sum::<f64>();

            // Log the total value for the stat
            info!("Total value for stat {:?}: {}", stat, total);

            // Insert the total value into the hashmap
            totals.insert(stat, total);
        }

        trace!("Finished calculation of stat totals from relics");
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
        trace!(
            "Calculating stat value for relic ID: {} and stat: {:?}",
            relic._id,
            stat
        );

        // Calculate the main stat value
        let mainstat_value = if relic.mainstat == stat {
            let value = relic.get_mainstat();
            trace!("Main stat matches. Value: {}", value);
            value
        } else {
            trace!("Main stat does not match. Value: 0.0");
            0.0
        };

        // Calculate the sum of substat values for the specified stat
        let substat_values: f64 = relic
            .substats
            .iter()
            .filter_map(|s| {
                if s.key == stat {
                    trace!("Found substat with key: {:?}, value: {}", s.key, s.value);
                    Some(s.value)
                } else {
                    None
                }
            })
            .sum();

        // Log the total value including both main stat and substats
        let total_value = mainstat_value + substat_values;
        debug!(
            "Total value for stat {:?} from relic ID {}: {}",
            stat, relic._id, total_value
        );

        total_value
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
        trace!("Applying set bonuses for relics");

        let mut totals = HashMap::new();

        // Count the number of relics in each set
        let counts = relics.iter().counts_by(|relic| relic.set.clone());
        trace!("Relic set counts: {:?}", counts);

        for (set, count) in counts {
            trace!("Processing set: {:?} with count: {}", set, count);

            if let Some(bonuses) = self.set_bonus.get(&set).and_then(|s| s.get(&(count as u8))) {
                trace!("Found bonuses for set {:?}: {:?}", set, bonuses);
                self.apply_bonuses(&mut totals, bonuses, first_round_stats)?;
            } else {
                trace!("No bonuses found for set {:?}", set);
            }
        }

        // Apply other bonuses (e.g., from character or external sources)
        self.apply_other_bonuses(&mut totals);
        debug!("Total stats after applying set bonuses: {:?}", totals);

        Ok(totals)
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
        totals: &mut HashMap<Stats, f64>,
        bonuses: &[Bonus],
        first_round_stats: Option<&HashMap<Stats, f64>>,
    ) -> Result<()> {
        trace!("Applying bonuses");

        for (stat, bonus_value, condition_bonus) in bonuses {
            trace!(
                "Applying bonus: stat = {:?}, bonus_value = {}",
                stat,
                bonus_value
            );

            // Apply unconditional bonus
            *totals.entry(stat.clone()).or_default() += bonus_value;

            // Check and apply conditional bonus if applicable
            if let Some((cond_stat, cond_bonus)) = condition_bonus {
                if let Some(&stat_value) = first_round_stats.and_then(|fs| fs.get(cond_stat)) {
                    trace!(
                        "Checking condition for stat {:?}: value = {}, condition_bonus = {}",
                        cond_stat,
                        stat_value,
                        cond_bonus
                    );

                    if stat_value > *cond_bonus {
                        trace!("Condition met. Applying additional bonus: {}", cond_bonus);
                        *totals.entry(stat.clone()).or_default() += cond_bonus;
                    }
                } else {
                    return Err(eyre::eyre!(
                        "Missing base stat {:?} in the first round calculation",
                        cond_stat
                    ));
                }
            }
        }

        debug!("Totals after applying bonuses: {:?}", totals);
        Ok(())
    }

    /// Applies additional bonuses to the total stats from the `other_bonus` map.
    ///
    /// This function updates the `totals` by adding each bonus from `other_bonus` to the corresponding stat.
    ///
    /// # Parameters
    ///
    /// - `totals`: A mutable reference to a `HashMap` where the additional bonuses will be applied.
    ///
    /// # Notes
    ///
    /// - The function assumes that `other_bonus` contains bonuses that are not tied to specific relic sets or conditions.
    fn apply_other_bonuses(&self, totals: &mut HashMap<Stats, f64>) {
        trace!("Applying other bonuses");

        for (stat, bonus) in &self.other_bonus {
            trace!("Applying bonus: stat = {:?}, bonus = {}", stat, bonus);

            // Add the bonus to the corresponding stat in totals
            *totals.entry(stat.clone()).or_default() += bonus;
        }

        debug!("Totals after applying other bonuses: {:?}", totals);
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
        trace!("Building expression with formula: {}", formula);

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
            .value("LightCone_HP", self.light_cone.light_cone_stats.hp)
            .value("LightCone_ATK", self.light_cone.light_cone_stats.atk)
            .value("LightCone_DEF", self.light_cone.light_cone_stats.def);

        // Substitute values for additional stats from totals
        for (stat, value) in totals {
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
                Stats::FireDMGBoost_ => "Fire DMG_Boost",
                Stats::WindDMGBoost_ => "Wind_DMG_Boost",
                Stats::IceDMGBoost_ => "Ice_DMG_Boost",
                Stats::LightningDMGBoost_ => "Lightning_DMG_Boost",
                Stats::PhysicalDMGBoost_ => "Physical_DMG_Boost",
                Stats::QuantumDMGBoost_ => "Quantum_DMG_Boost",
                Stats::ImaginaryDMGBoost_ => "Imaginary_DMG_Boost",
                Stats::CommonDMGBoost_ => "Common_DMG_Boost",
                Stats::CritRate_ => "CRIT_Rate",
                Stats::CritDmg_ => "CRIT_DMG",
                Stats::BreakEffect_ => "Break_Effect",
                Stats::EffectHitRate_ => "Effect_Hit_Rate",
                Stats::EffectRES_ => "Effect_RES",
                Stats::EnergyRegenerationRate_ => "Energy_Regeneration_Rate",
                _ => continue,
            };
            trace!("Substituting value: key = {}, value = {}", key, value);
            expr = expr.value(key, *value);
        }

        // Substitute default values for base crit rate and crit damage
        expr = expr
            .value("Character_Base_CRIT_Rate", BASE_CRIT_RATE)
            .value("Character_Base_CRIT_DMG", BASE_CRIT_DMG);

        debug!("Expression after substitutions: {:?}", expr);

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
        trace!("Applying constraints. Initial result: {}", result);

        // Iterate over each constraint defined in `self.constraint`.
        for (stat, required_value) in &self.constraint {
            // Retrieve the current value of the statistic from `base_stats`.
            let current_stat = base_stats
                .get(stat)
                .ok_or_eyre(format!("Missing stat {:?}", stat))?;

            // Log the current stat and the required value for debugging purposes.
            trace!(
                "Checking constraint for stat {:?}. Current value: {}, Required value: {}",
                stat,
                current_stat,
                required_value
            );

            // Check if the current statistic value is below the required value.
            if current_stat < required_value {
                // Penalize the result by negating it if the constraint is not met.
                adjusted_result = -adjusted_result;
                trace!(
                    "Constraint not met. Penalizing result. New result: {}",
                    adjusted_result
                );
            }
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
        // Log the totals used to build the expression.
        trace!("Building final expression with totals: {:?}", totals);

        // Build the final expression using the target formula and the provided totals.
        let final_expr = self.build_expr(&self.target_formula, totals);
        debug!("Built final expression: {:?}", final_expr);

        // Execute the expression and obtain the result in JSON format.
        let result_value = final_expr.exec()?;
        debug!("Expression executed. Result JSON: {:?}", result_value);

        // Convert the JSON result to a floating-point number.
        let result: f64 = serde_json::from_value(result_value)?;
        debug!("Final result obtained: {}", result);

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
        trace!("Starting evaluation with relics: {:?}", relics);

        // Calculate initial totals from the provided relics.
        let initial_totals = self.calculate_totals(&relics, None)?;
        trace!("Initial totals calculated: {:?}", initial_totals);

        // Evaluate base statistics based on the initial totals.
        let initial_base_stats = self.evaluate_base_stats(&initial_totals)?;
        trace!("Initial base stats evaluated: {:?}", initial_base_stats);

        // Recalculate totals considering the evaluated base statistics.
        let updated_totals = self.calculate_totals(&relics, Some(initial_base_stats))?;
        trace!(
            "Updated totals recalculated with base stats: {:?}",
            updated_totals
        );

        // Evaluate base statistics again based on the updated totals.
        let updated_base_stats = self.evaluate_base_stats(&updated_totals)?;
        trace!("Updated base stats evaluated: {:?}", updated_base_stats);

        // Build and execute the final expression using the updated totals.
        let final_result = self.calculate_final_result(&updated_totals)?;
        trace!("Final result obtained from expression: {}", final_result);

        // Apply constraints to the final result and adjust if necessary.
        let constrained_result = self.apply_constraints(final_result, &updated_base_stats)?;
        trace!(
            "Constrained result after applying constraints: {}",
            constrained_result
        );

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
        trace!("Evaluating base stats with totals: {:?}", totals);

        let result = self
            .base_stats_formulas
            .iter()
            .map(|(stat, formula)| {
                trace!("Processing formula for stat {:?}: {}", stat, formula);
                let expr = self.build_expr(formula, totals);
                let value = serde_json::from_value(expr.exec()?)?;
                debug!("Computed value for stat {:?}: {}", stat, value);
                Ok((stat.clone(), value))
            })
            .collect();

        debug!("Base stats evaluation completed with result: {:?}", result);
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
        trace!("Calculating total stats for relics: {:?}", relics);

        // Calculate base stats from relics
        let mut totals = self.calculate_stat_total_from_relics(relics)?;
        debug!("Base stat totals from relics: {:?}", totals);

        // Apply set bonuses to the calculated totals
        let set_bonus_totals = self.apply_set_bonus(relics, first_round_stats.as_ref())?;
        debug!("Set bonus totals: {:?}", set_bonus_totals);

        // Add set bonuses to the total stats
        for (stat, bonus) in set_bonus_totals {
            *totals.entry(stat).or_default() += bonus;
        }

        debug!("Final total stats after applying set bonuses: {:?}", totals);
        Ok(totals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        CharacterName, CharacterSkills, CharacterTraces, LightConeName, LightConeStats, Slot,
        SubStats,
    };

    #[tokio::test]
    async fn test_evaluation() -> Result<()> {
        // Create a new character instance with specific attributes.
        let mut fu_xuan = Character::new(
            CharacterName::FuXuan,
            80, // Level
            6,  // Ascension
            0,  // Eidolon
            CharacterSkills {
                basic: 1,
                skill: 9,
                ult: 9,
                talent: 9,
            },
            CharacterTraces {
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
                total_bonus: HashMap::new(),
            },
            None,
            vec![],
        )
        .await?;

        // Create a new LightCone instance with specific attributes.
        let mut we_are_wild_fire = LightCone {
            key: LightConeName::WeAreWildfire,
            level: 50,
            ascension: 3,
            superimposition: 0,
            location: None,
            lock: false,
            light_cone_stats: LightConeStats {
                ..Default::default()
            },
            _id: "lightcone_100".to_string(),
        };

        // Create Relic instances for different slots.
        let head = Relic {
            set: RelicSetName::KnightOfPurityPalace,
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
            _id: "relic_1".to_string(),
        };

        let hands = Relic {
            set: RelicSetName::LongevousDisciple,
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
            _id: "relic_2".to_string(),
        };

        let body = Relic {
            set: RelicSetName::LongevousDisciple,
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
            _id: "relic_3".to_string(),
        };

        let feet = Relic {
            set: RelicSetName::KnightOfPurityPalace,
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
            _id: "relic_4".to_string(),
        };

        let sphere = Relic {
            set: RelicSetName::FleetOfTheAgeless,
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
                    key: Stats::EffectRES_,
                    value: 12.0,
                },
                SubStats {
                    key: Stats::BreakEffect_,
                    value: 5.8,
                },
            ],
            location: None,
            lock: false,
            _id: "relic_5".to_string(),
        };

        let rope = Relic {
            set: RelicSetName::FleetOfTheAgeless,
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
            _id: "relic_6".to_string(),
        };

        // Combine all relics into a vector for evaluation.
        let relics = vec![head, hands, body, feet, sphere, rope];

        // Add base stats to the character and fetch the main stat of the light cone.
        fu_xuan.add_base_stats().await?;
        we_are_wild_fire.get_main_stat().await?;

        // Clone the trace bonuses from the character.
        let trace_bonus = fu_xuan.traces.total_bonus.clone();

        // Define set bonuses for relic sets.
        let set_bonus = HashMap::from([
            (
                RelicSetName::KnightOfPurityPalace,
                HashMap::from([(2, vec![(Stats::Def_, 15.0, None)])]),
            ),
            (
                RelicSetName::LongevousDisciple,
                HashMap::from([(2, vec![(Stats::Hp_, 12.0, None)])]),
            ),
            (
                RelicSetName::FleetOfTheAgeless,
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
                (Stats::EffectRES_, effect_res_formula.to_owned()),
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
                (Stats::EffectRES_, effect_res_formula.to_owned()),
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
