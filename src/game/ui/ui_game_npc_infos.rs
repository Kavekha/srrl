use bevy::prelude::*;

use crate::{game::{combat::combat_system::components::IsDead, despawn_component, pieces::components::{Health, Npc}, player::Player, tileboard::components::BoardPosition, visibility::components::View}, 
    globals::STANDARD_TILE_SIZE
};

use super::{components::{ UiEnemyHp, UiGameInterface, UiHpBar}, INTERFACE_HP_CHUNK_HEIGHT, INTERFACE_HP_CHUNK_WIDTH};


fn clear_ui_game_enemy_hp(
    commands: &mut Commands,    
    interface_query: Query<Entity, With<UiEnemyHp>>,
) {
    //println!("DEBUG: Clear enemy hp ui.");    
    despawn_component(interface_query, commands);
}

// Quand jouée "OnEnter", le resultat ne corresponds pas.
// C'est parce que la camera commence en 0,0 mais que le PJ pas forcement car ca depends de la map.
// OnEnter, il est peu probable que le personnage soit à 0,0 et donc que l'affichage soit juste. 
// La camera se déplace rapidement pour se centrer vers le perso mais ce ne sera que dans les Update de cette fonction que le PJ sera vraiment dans le Champ de la Camera.
pub fn draw_ui_game_enemy_hp(
    mut commands: Commands,
    enemies_q: Query<(Entity, &Health, &Transform), (With<Npc>, Without<IsDead>)>,
    camera_q: Query<(&Camera, &GlobalTransform)>,    
    interface_query: Query<Entity, With<UiEnemyHp>>,
    view_q: Query<&View, With<Player>>,
    npc_position_q: Query<&BoardPosition, With<Npc>>,
){
    clear_ui_game_enemy_hp(&mut commands, interface_query);

    let (camera, camera_transform) = camera_q.single();
    let Some(screen_size) = camera.logical_viewport_size() else { return };  

    let Ok(view) = view_q.get_single() else { return };
    for (npc_entity, health, transform) in enemies_q.iter() {
        let Ok(npc_position) = npc_position_q.get(npc_entity) else { continue };
        if !view.visible_tiles.contains(&npc_position.v) {
            // On ne voit pas le npc.
            continue;
        }
        let Some(screen_position) = camera.world_to_viewport(camera_transform, transform.translation)  else { continue };
        //If not in screen, we don't display.
        if screen_position.x < 0.0 || screen_position.x > screen_size.x || screen_position.y < 0.0 || screen_position.y > screen_size.y { continue};
    
        let left =screen_position.x - ((STANDARD_TILE_SIZE / 2) as f32);
        //let right =screen_size.x - screen_position.x;
        let top =screen_position.y - ((STANDARD_TILE_SIZE / 2) as f32); // REMEMBER : world = y goes from bottom to top (++)
        //let bottom = screen_size.y - screen_position.y;        
        let width = (health.max as f32 * INTERFACE_HP_CHUNK_WIDTH) / 2.0; //INTERFACE_HP_CHUNK_WIDTH * (health.max as f32) / 2.0;
        let height = INTERFACE_HP_CHUNK_HEIGHT/ 2.0;
        let grow = (health.max as f32 * INTERFACE_HP_CHUNK_WIDTH) / 2.0;

        let chunk_container = commands.spawn(NodeBundle {
            style: Style {                
                left: Val::Px(left),
                //right: Val::Px(right),
                top: Val::Px(top),
                //bottom: Val::Px(bottom),
                width: Val::Px(width),
                height: Val::Px(height),
                flex_grow: grow,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            //background_color: Color::rgba(0.0, 0.0, 1.0, 0.5 ).into(),
            ..default()
        }).id();  
        commands.entity(chunk_container)
        .insert(UiEnemyHp)
        .insert(UiHpBar {entity: npc_entity})
        .insert(UiGameInterface);

        let mut chunk_list:Vec<Entity> = Vec::new();
        for i in 1..=health.max {
            let mut border_color = Color::rgb(0.5, 0.0, 0.0);
            let mut background_color = Color::rgb(0.9, 0.0, 0.0 );
            if i > health.current {
                border_color = Color::rgb(0.1, 0.1, 0.1);
                background_color = Color::rgba(0.0, 0.0, 0.0, 1.0 );
            }
    
            let chunk = commands.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(INTERFACE_HP_CHUNK_WIDTH / 2.0),//(8.0),
                    height: Val::Px(INTERFACE_HP_CHUNK_HEIGHT/ 2.0), //(16.0),
                    margin: UiRect::all(Val::Px(1.)),   
                    flex_grow: 8.0,
                    bottom: Val::Px(8.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                border_color: border_color.into(), 
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_grow: 8.0,
                        ..default()
                    },
                    background_color: background_color.into(),
                    ..default()
                });  
            }).id();
            chunk_list.push(chunk);
        }
        
    
        for chunk in chunk_list {
            commands.entity(chunk_container).add_child(chunk);
        }
    }
}
