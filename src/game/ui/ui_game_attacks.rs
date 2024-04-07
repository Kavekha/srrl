use bevy::prelude::*;

use crate::{engine::asset_loaders::GraphicsAssets, game::{combat::components::AttackType, despawn_component}};

use super::components::{UiAttackIcon, UiMainWindow};


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
                align_items: AlignItems::FlexEnd, 
                justify_content: JustifyContent::FlexEnd, 
                flex_direction: FlexDirection::Row,
                width: Val::Percent(50.),
                height: Val::Percent(20.),
                bottom: Val::Px(10.),
                ..default()
            },
            ..default()
        }).insert(UiAttackIcon).id();
    commands.entity(container).push_children(&[attack_container]);


    // Bouton par icone.
    let mut icon_n=1;
    for image in ["button_attack_melee", "button_attack_ranged"] {
        let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 1, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let border_color: Color;
        if icon_n == 2 {
            border_color = Color::rgba(0.5, 0.5, 0.0, 0.0);
        } else {
            border_color = Color::rgba(0.5, 0.5, 0.0, 1.0);
        }

        let border_icon = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(58.),
                height: Val::Px(58.),
                border: UiRect::all(Val::Px(5.)),                    
                ..default()
            },
            border_color: border_color.into(), //Color::rgba(0.5, 0.5, 0.0, 0.0).into(),
             ..default()
        }).insert(UiAttackIcon).id();

        let icon = commands
        .spawn(AtlasImageBundle {
            style: Style {
                width: Val::Px(48.),
                height: Val::Px(48.),
                border: UiRect::all(Val::Px(2.)),                    
                ..default()
            },                
            texture_atlas: texture_atlas_handle.into(),
            image: UiImage::new(assets.images[image].clone()),
            ..default()
        }).id();
        
        icon_n += 1;

        commands.entity(border_icon).push_children(&[icon]);
        commands.entity(attack_container).push_children(&[border_icon]);
    }
}

//