// == DOCUMENTATION
/*
Ici seront gérées les règles "gameplay" diverses et variées.
L'idée est aussi qu'il soit possible de les configurer.

 */

use rand::prelude::*;

use crate::game::pieces::components::Stats;

use super::combat_system::components::{ActionPoints, AttackType};

/// ============================================================================
/// Action Point COST
pub const AP_COST_MOVE:u32 = 1;
pub const AP_COST_MELEE:u32 = 3;
pub const AP_COST_RANGED:u32 = 5;
//pub const AP_COST_NO_VALUE:u32 = 999;   // Pour couvrir un cas non supporté

/// Ranged.
pub const RANGED_ATTACK_RANGE_MAX: i32 = 10;

/// NPC 
/// pub const NPC_VISION_RANGE_MAX: i32 = 25;
pub const NPC_RANGED_ATTACK_RANGE_MAX: i32 = 10;
pub const NPC_CHANCE_TO_BE_RANGED:i32 = 25; // Percent

/// Si NPC depasse cette distance du PJ, on ignore son tour / lui retire.
pub const NPC_MAX_DISTANCE_RANGE_FROM_PLAYER_FOR_TURN:i32 = 50; 

// View Range
pub const VISIBILITY_RANGE_PLAYER:i32 = 3;  //20;
///==============================================================================


pub struct RuleCombatResult{
    pub success: bool,
    pub dmg: u32    
}
pub struct RuleDamageResist{
    pub success: bool,
    pub dmg_reduction: u32
}

pub fn dmg_resist_test(
    attack_type:&AttackType,
    defender_stats: &Stats,
) -> RuleDamageResist {
    let dice_roll :DiceRollResult;
    match attack_type {
        AttackType::MELEE => dice_roll = roll_dices_against(defender_stats.strength, 0),
        AttackType::RANGED => dice_roll = roll_dices_against(defender_stats.logic, 0)        // Pas d'opposant ni difficulté : On encaisse X dmg.
    }
    
    //let dmg = get_hit.dmg.saturating_sub(dice_roll.success); 
    let (success, nb_success) = dice_roll.result();
    RuleDamageResist { success: success, dmg_reduction: nb_success}
}

pub fn combat_test(
    attack_type:&AttackType, 
    attacker_stats: &Stats,
    defender_stats: &Stats
) -> RuleCombatResult {
    let dice_roll:DiceRollResult;
    let mut dmg=0;
    let success:bool;
    let nb_success:u32;
    match attack_type {
        AttackType::MELEE => {
            dice_roll = roll_dices_against(attacker_stats.agility + attacker_stats.melee, defender_stats.logic + defender_stats.agility);   
            (success, nb_success) = dice_roll.result();
            if success {
                dmg = nb_success.saturating_add(attacker_stats.strength as u32);
            }            
        },
        AttackType::RANGED => {
            dice_roll = roll_dices_against(attacker_stats.agility + attacker_stats.firearms, defender_stats.logic + defender_stats.agility);   
            (success, nb_success) = dice_roll.result();
            if success {
                dmg = nb_success.saturating_add(attacker_stats.logic as u32);
            }              
        }
    }
    //println!("Combat test: Result is : success {}, dmg {}", success, dmg);
    return RuleCombatResult { success:success, dmg: dmg }
}


// Retourne True + nb de success si réussite, sinon False et 0.
// Fail & Glitch ne servent pas pour le moment.
pub struct DiceRollResult{
    pub success: u32,
    pub fail: u32,
    pub glitch: u32 
}
impl DiceRollResult{
    pub fn result(&self) -> (bool, u32) {        
        if self.success > 0 {
            return (true, self.success)
        } else {
            return (false, 0)
        }
    }
}


/// On jette des dés pour l'Attaquant & le Defendeur. On fait Succes - Succes.
pub fn roll_dices_against(
    user: u32,
    against: u32
) -> DiceRollResult {
    let mut user_result = roll_dices(user);
    let against_result = roll_dices(against);
    //println!("roll_dices_against: User: {:?}", (user_result.success, user_result.fail, user_result.glitch));
    //println!("roll_dices_against: Against: {:?}", (against_result.success, against_result.fail, against_result.glitch));

    user_result.success = user_result.success.saturating_sub(against_result.success);   //on retire les succès du Defendeur / difficulté. REMEMBER: saturating_sub => Ne depasse pas la limite, qui est de 0 en u32.
    //user_result.fail = user_result.fail.saturating_add(against_result.success);   // On s'en fout des fails.
    //println!("roll_dices_against: Final User Result: {:?}", user_result.success);
    user_result
}


// On jette plusieurs dés à 6 faces.
// Succes si 5+, glitch si 1. Sinon Fail.
// Fail & glitch n'ont pas d'impact: ce sont juste des "non succès".
pub fn roll_dices(
    nb_dices : u32
) -> DiceRollResult {
    let mut result = DiceRollResult {success: 0, fail: 0, glitch : 0};
    for _roll in 0..nb_dices {
        let roll_result = roll_dice();
        //println!("Roll is : {:?}", roll_result);
        match roll_result {
            6 | 5 => result.success += 1,
            1 => result.glitch += 1,
            _ => result.fail += 1
        }        
    }
    //println!("Dices roll result: success: {}, glitch: {}, fail: {}", result.success, result.glitch, result.fail);
    result
}

// On jete un dé à 6 faces.
pub fn roll_dice() -> u32 {
    let mut rng = thread_rng();
    let dice_roll: u32 = rng.gen_range(0..=6);
    dice_roll

}


pub fn consume_actionpoints(
    actionpoints_component: &mut ActionPoints,
    lost_value: u32,
) {
    actionpoints_component.current = actionpoints_component.current.saturating_sub(lost_value);
}


pub fn enough_ap_for_action(
    actionpoints_component: &ActionPoints,
    attack_type: &AttackType
) -> Result<bool, bool> {
    let ap_cost: u32;
    match attack_type {
        AttackType::MELEE => ap_cost = AP_COST_MELEE,
        AttackType::RANGED => ap_cost = AP_COST_RANGED,
        /*
        _ => { 
            ap_cost = AP_COST_NO_VALUE;
            println!("No ap cost verification for {:?}", attack_type);
        }, */
    };
    if actionpoints_component.current >= ap_cost {
        return Ok(true)
    } else {
        return Err(false)
    }
}