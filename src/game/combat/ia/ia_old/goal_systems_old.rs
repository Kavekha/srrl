//v0.19h
//https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://citeseerx.ist.psu.edu/document%3Frepid%3Drep1%26type%3Dpdf%26doi%3D012ef03d0f951092b8645b69aebdbce900ac03e4&ved=2ahUKEwingo_qkrKFAxWsTaQEHYTTAFIQFnoECCMQAQ&usg=AOvVaw3spa-hKcVtGhhaO5QmYsWT


use bevy::prelude::*;


use crate::game::{combat::{combat_system::components::{IsDead, WantToForfeit}, events::Turn, ia::components::{Goal, GoalType, Planning}}, pieces::components::Npc, player::Player};

use super::components::CheckGoal;



// Donne à chaque NPC le but de tuer le joueur.
pub fn npc_initialise_goals(
    mut commands: Commands,
    entity_npc_q: Query<Entity, With<Npc>>,// Joué en Setup, pas besoin de verifier si vivant etcs.
    entity_player_q: Query<Entity, With<Player>>,  // Notre cible.
) {
    let Ok(player_entity) = entity_player_q.get_single() else { println!("WARNING: No player found for initialise goals"); return;};
    for npc_entity in entity_npc_q.iter() {
        commands.entity(npc_entity).insert(Goal { id: GoalType::KillEntity { id: player_entity}});
        //println!("Kill Goal initialized for NPC {:?}", npc_entity);
    }
}

// 0.19h : Verifie si ce goal est tjrs d'actualité. A ce stade on ne fait rien de pluss, car on a qu'un seul goal. Sera utile plus tard. 
// A voir comment industrialiser la requête. => Donner un composant que l'on veut sur une entité par exemple.
pub fn npc_goal_reached(
    mut commands: Commands,
    npc_entity_goal_q: Query<(Entity, &Goal), (With<Npc>, With<CheckGoal>, With<Turn>, Without<IsDead>)>,    // Si pas de Turn, ca tournera en boucle.
    entity_killed_q: Query<&IsDead>,
) {
    let mut to_remove = Vec::new();
    for (npc_entity, npc_goal) in npc_entity_goal_q.iter() {
        match npc_goal.id {
            GoalType::KillEntity{id} => {
                if let Ok(_entity_dead) = entity_killed_q.get(id) {
                    println!("Goal {:?} for NPC {:?} is resolved.", npc_goal.id, npc_entity);
                    // Ici on retire le Planning car on a un seul goal. 
                    to_remove.push(npc_entity);
                    commands.entity(npc_entity).insert(WantToForfeit);                      
                } else {
                    commands.entity(npc_entity).insert(Planning);
                    //println!("Goal {:?} for NPC {:?} is still not true and need to be accomplished.", npc_goal.id, npc_entity);                    
                }
            },
            GoalType::None => {}
        };
    };
    for entity in to_remove {
        commands.entity(entity).remove::<CheckGoal>();
    }
}