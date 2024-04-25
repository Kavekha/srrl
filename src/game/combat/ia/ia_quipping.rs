// ==> 0.20s
/*
Un peu bidon.
Selon le Planning a un instant X, on fait un jet pour voir si l'action est commentée.
Si c'est le cas, on envoie un message et on pose un composant "has_talked".
Il est retiré quand ce n'est plus le tour pour éviter qu'il se repete.
Les messages sont très specifiques.

A repenser pour un vrai systeme, lorsque les choses seront plus modulables.

*/

use bevy::prelude::*;
use rand::Rng;

use crate::game::{combat::{combat_system::components::IsDead, events::Turn}, pieces::components::Npc, ui::display_text::TextEvent};

use super::{components::HasTalked, Planning};

const IA_CHANCES_TO_TALK: u32 = 25;

/* 
in_sight: false,
know_target_position: false,
ap_for_range: false,
melee_range: false,
ap_for_melee: false,
low_health: false,
has_allies_nearby: false,
can_move: false,
*/

pub fn ia_quipping_actions(
    mut commands: Commands, 
    npc_entity_fighter_q: Query<(Entity, &Planning), (With<Npc>, With<Turn>, Without<IsDead>, Without<HasTalked>)>,
    //mut ev_log: EventWriter<LogEvent>,
    mut ev_box: EventWriter<TextEvent>,
){
    for (entity, planning) in npc_entity_fighter_q.iter() {
        let mut rng = rand::thread_rng();
        let rand_to_talk = rng.gen_range(0..100);
        println!("----------------RAND IS {:?}", rand_to_talk);

        if rand_to_talk > IA_CHANCES_TO_TALK {
            let mut available_texts = Vec::new();
            if planning.in_sight { available_texts.push("Come over here!") }
            if !planning.in_sight { available_texts.push("Did you saw them?") }
            if !planning.know_target_position { available_texts.push("Where are they?!")} 
            if planning.know_target_position { available_texts.push("I know where you are...")} 
            if planning.ap_for_range { available_texts.push("got them in my crosshair!") }
            if planning.ap_for_melee { available_texts.push("Time to gutt you.")}
            if planning.melee_range { available_texts.push("Cut cut cut!") }
            if planning.low_health { available_texts.push("Doesn't feel good....")}
            if planning.has_allies_nearby { available_texts.push("Let's go guys!!!")}
            if planning.know_target_position && planning.has_allies_nearby { available_texts.push("They are here, go get them!!!")};
            available_texts.push("Report!");

            let index = rng.gen_range(0..(available_texts.len()-1));

            //ev_log.send(LogEvent { entry: format!("{:?} says: {:?}", name, final_entry) });
            println!("talk is {:?}", available_texts[index]);
            ev_box.send(TextEvent { entry: format!("{}", available_texts[index]), entity: entity });

            commands.entity(entity).insert(HasTalked);
        } else {
            println!("No talk");
        }     
    }
}


pub fn cleaning_has_talked_status(
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &HasTalked), Without<Turn>>,
) {
    let mut to_remove = Vec::new();
    for (entity, _has_talked) in npc_entity_fighter_q.iter() {
        to_remove.push(entity);
    }
    for entity in to_remove {
        commands.entity(entity).remove::<HasTalked>();
    }
}