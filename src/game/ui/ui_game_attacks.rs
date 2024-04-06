use bevy::prelude::*;

use crate::{engine::asset_loaders::GraphicsAssets, game::{combat::components::AttackType, despawn_component}};

use super::components::{UiAttackIcon, UiMainWindow};


pub fn clear_ui_game_attack_icons(
    commands: &mut Commands,    
    attack_icon_q: Query<Entity, With<UiAttackIcon>>,
) {
    //println!("DEBUG: Clear Attack Icons ui.");    
    despawn_component(attack_icon_q, commands);
}

pub fn draw_ui_game_attack_icons(
    mut commands: Commands,
    assets: Res<GraphicsAssets>,
    ui_main_q: Query<(Entity, &UiMainWindow)>,
    ui_attack_icon_q: Query<Entity, With<UiAttackIcon>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,

/* 
    camera_q: Query<(&Camera, &GlobalTransform)>, 
    query_game_cursor: Query<&mut Transform, With<GameCursorRender>>,
    interface_query: Query<Entity, With<UiActionPointsOnCursor>>,
    player_q: Query<Entity, With<Player>>,
    action_infos: Res<ActionInfos>,
    mut cursor_moved_events: EventReader<CursorMoved>,

*/
){
    //clear_ui_game_attack_icons(&mut commands, ui_attack_icon_q);

    // Interface container. 0.19f : fenetre globale dans mod.rs.
    let Ok(main_window) = ui_main_q.get_single() else { 
        println!("No main Window, can't display anything.");
        return ;
    };
    let (container, _main_window_component) = main_window;

    // Attack containers.
    let attack_container = commands 
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(50.),
                height: Val::Percent(10.),

                ..default()
            },
            ..default()
        }).insert(UiAttackIcon).id();
    commands.entity(container).push_children(&[attack_container]);


    // Bouton par icone.
     for _attack in [AttackType::MELEE, AttackType::RANGED] {
        let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 1, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let icon = commands
            .spawn(AtlasImageBundle {
                style: Style {
                    width: Val::Px(32.),
                    height: Val::Px(32.),
                    ..default()
                },
                texture_atlas: texture_atlas_handle.into(),
                image: UiImage::new(assets.images["button_attack_melee"].clone()),
                ..default()
            }).id();
        commands.entity(attack_container).push_children(&[icon]);
    }
}

