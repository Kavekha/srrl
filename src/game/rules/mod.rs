use rand::prelude::*;

use super::combat::components::ActionPoints;


pub struct DiceRollResult{
    pub success: u32,
    pub fail: u32,
    pub glitch: u32 
}


/// Return the dice_roll_results for the user.
pub fn roll_dices_against(
    user: u32,
    against: u32
) -> DiceRollResult {
    let mut user_result = roll_dices(user);
    let against_result = roll_dices(against);
    println!("roll_dices_against: User: {:?}", (user_result.success, user_result.fail, user_result.glitch));
    println!("roll_dices_against: Against: {:?}", (against_result.success, against_result.fail, against_result.glitch));

    user_result.success = user_result.success.saturating_sub(against_result.success);   //REMEMBER: saturating_sub => Ne depasse pas la limite, qui est de 0 en u32.
    user_result.fail = user_result.fail.saturating_add(against_result.success);
    println!("roll_dices_against: Final User Result: {:?}", (user_result.success, user_result.fail, user_result.glitch));
    user_result
}


pub fn roll_dices(
    nb_dices : u32
) -> DiceRollResult {
    let mut result = DiceRollResult {success: 0, fail: 0, glitch : 0};
    for _roll in 0..nb_dices {
        let roll_result = roll_dice();
        println!("Roll is : {:?}", roll_result);
        match roll_result {
            6 => result.success += 1,
            5 => result.success += 1,
            1 => result.glitch += 1,
            _ => result.fail += 1
        }        
    }
    result
}

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
