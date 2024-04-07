use bevy::prelude::*;

use crate::{engine::asset_loaders::GraphicsAssets, game::combat::{action_infos::ActionInfos, components::AttackType}, globals::INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE};

use super::{components::{UiAttackIcon, UiGameInterface, UiMainWindow}, ReloadUiEvent};


const UI_ATTACK_BORDER_SELECTED: Color = Color::rgba(0.5, 0.5, 0.0, 1.0);
const UI_ATTACK_BORDER_INVISIBLE: Color = Color::rgba(0.5, 0.5, 0.0, 0.0);


pub fn update_ui_game_attack_icons(
    mut ev_ui: EventReader<ReloadUiEvent>,
    mut ui_attack_border_q: Query<(&mut BorderColor, &UiAttackIcon)>,
    action_infos: Res<ActionInfos>
){
    for _event in ev_ui.read() {
        println!("Je dois mettre à jour les cadres d'icone.");
        
        let Some(action_attack) = action_infos.attack.clone() else { continue;};

        for (mut border_color, attack_icon) in &mut ui_attack_border_q {
            if attack_icon.attack_type == action_attack {
                *border_color = UI_ATTACK_BORDER_SELECTED.into();
            } else {
                *border_color = UI_ATTACK_BORDER_INVISIBLE.into();
            }
        }
    }
}

pub fn draw_ui_game_attack_icons(
    mut commands: Commands,
    assets: Res<GraphicsAssets>,
    ui_main_q: Query<(Entity, &UiMainWindow)>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    // Interface container. 0.19f : fenetre globale dans mod.rs.
    let Ok(main_window) = ui_main_q.get_single() else { 
        println!("No main Window, can't display anything.");
        return ;
    };
    let (container, _main_window_component) = main_window;
    let mut attack_button= 0;

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
        }).insert(UiGameInterface).id();
    commands.entity(container).push_children(&[attack_container]);


    // Bouton par icone.    // TODO: Ce sera a changer, car beaucoup encore en dur. 

    for attack in [AttackType::MELEE, AttackType::RANGED] {

        attack_button += 1;
        let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 1, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let image: &str;
        match attack {
            AttackType::MELEE => image = "button_attack_melee",
            AttackType::RANGED => image = "button_attack_ranged",
        }

        let border_icon = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(58.),
                height: Val::Px(58.),
                border: UiRect::all(Val::Px(5.)),                    
                ..default()
            },
            border_color: UI_ATTACK_BORDER_INVISIBLE.into(), //Color::rgba(0.5, 0.5, 0.0, 0.0).into(),
             ..default()
        }).insert(UiAttackIcon {attack_type: attack} ).id();

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
        
        //TODO : Ici aussi c'est en dur, c'est bof.
        let attack_button_display = commands.spawn(
            TextBundle::from_section(
                format!("{}",attack_button),
                TextStyle { 
                    font: assets.font.clone(),  
                    font_size: INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE,
                    color: Color::WHITE,
                },
            )
        ).id();

        commands.entity(icon).push_children(&[attack_button_display]);
        commands.entity(border_icon).push_children(&[icon]);
        commands.entity(attack_container).push_children(&[border_icon]);
    }

    
}

//