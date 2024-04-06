use bevy::prelude::*;

use crate::{
    engine::asset_loaders::GraphicsAssets, 
    game::{combat::components::ActionPoints, despawn_component, pieces::components::Health, player::Player, ui::{INTERFACE_HP_CHUNK_HEIGHT, INTERFACE_HP_CHUNK_MAX, INTERFACE_HP_CHUNK_WIDTH}}, 
    globals::INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE
};

use super::{components::{UiCharacterInfos, UiGameInterface, UiMainWindow}, ReloadUiEvent};


pub fn clear_ui_game_character_infos(
    interface_query: Query<Entity, With<UiCharacterInfos>>,
    commands: &mut Commands,
) {
    //println!("DEBUG: Clear interface ui.");
    despawn_component(interface_query, commands);
}


#[derive(Component)]
pub struct UiChunk;

// Utile?
#[derive(Component)]
pub struct UiChunkContainer;

#[derive(Component)]
pub struct UiActionPoints;


pub fn update_ui_character_health(
    mut ev_ui: EventReader<ReloadUiEvent>
){
    for event in ev_ui.read() {
        println!("Je dois mettre à jour les Chunks.");
    }
}

pub fn update_ui_character_action_points(
    mut ev_ui: EventReader<ReloadUiEvent>,
    //mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor, &Children,),(Changed<Interaction>, With<Button>)>,
    //mut ui_ap_q: Query<(Entity, &UiActionPoints, &Children)>,
    //mut text_query: Query<&mut Text>,
    player_actions_query: Query<(Entity, &ActionPoints), With<Player>>,
    mut ap_text_q: Query<&mut Text, With<UiActionPoints>>,
){
    for _event in ev_ui.read() {
        println!("Je dois mettre à jour les Action Points.");

        let mut action_points = 0;
        if let Ok(player_action_points) = player_actions_query.get_single() {
            println!("Points d'action du joueur récupéré!");
            let (_p_entity_action, p_action) = player_action_points;
            action_points = p_action.current;
        } else {
            println!("Pas de points récupérés via action. On mets 99 pour voir si ca marche.");
            action_points = 99;
        }
        // On modifie le contenu.
        for mut text in &mut ap_text_q {
            text.sections[0].value = format!("{action_points}");
        }


        /* 
        for (entity, ui_action_points, children) in &mut ui_ap_q {
            //let mut text = text_query.get_mut(children[0]).unwrap();
            let mut text = text_query.get_mut(entity).unwrap();
            text.sections[0].value = action_points.to_string();
        }
        */
    }
}

// Refacto 0.19f : Avant on faisait du update, draw_ui_game en boucle. Maintenant on passe par un autre moyen pour faire la mise à jour globale.
pub fn draw_ui_game_character_infos(
    mut commands: Commands,
    assets: Res<GraphicsAssets>,
    player_info_query: Query<(Entity, Option<&Name>, &Health), With<Player>>,       //player_info_query: Query<(Entity, &Name, &Health), With<Player>>  // Retrait du Name car au load Save on le perds.
    player_actions_query: Query<(Entity, &ActionPoints), With<Player>>,
    ui_main_q: Query<(Entity, &UiMainWindow)>,
) {
    println!("DEBUG: draw ui_game_character_infos");

    // == Recuperation des infos necessaires ==
    let mut player_name = "Unkwnown Runner";
    let mut player_health_max = INTERFACE_HP_CHUNK_MAX;
    let mut player_health_current = INTERFACE_HP_CHUNK_MAX;

    if let Ok(player_infos) = player_info_query.get_single() {
        //println!("DEBUG : draw interface: Player info OK");
        let (_p_entity, p_name, p_health) = player_infos;   //let (_p_entity, p_name, p_health) = player_infos; // Retrait du name car perdu au save.
        if let Some(name) = p_name {
            player_name = name.as_str();
        }
        //player_name = p_name.as_str();
        player_health_max = p_health.max;
        player_health_current = p_health.current;
        println!("DEBUG: Interface: Player health current is {} and max {}", player_health_current, player_health_max);
    } else {
        //println!("DEBUG : draw: not player");
    }
    let mut action_points = 0;
    if let Ok(player_action_points) = player_actions_query.get_single() {
        let (_p_entity_action, p_action) = player_action_points;
        action_points = p_action.current;
    }

    // === Construction de l'UI. =====

    // Interface container. 0.19f : Move to mods, car fenetre globale.
    let Ok(main_window) = ui_main_q.get_single() else { 
        println!("No main Window, can't display anything.");
        return ;
    };
    let (container, _main_window_component) = main_window;

    // Interface UI Character
    let interface = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(10.0),
            align_content: AlignContent::FlexEnd,       // Added 0.19f
            justify_content: JustifyContent::FlexStart, 
            align_items: AlignItems::FlexEnd,
            align_self: AlignSelf::FlexEnd,
            bottom: Val::Px(0.),
            ..default()
        },
        ..default()
    }).insert(UiGameInterface).id();  

    //== Points d'action.       // TODO : Cette info se mets à jour en Update.
    let player_action_display = commands.spawn(
        TextBundle::from_section(
            format!("{}",action_points),
            TextStyle { 
                font: assets.font.clone(),  
                font_size: INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE,
                color: Color::YELLOW,
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(8.)),            
            ..default()
        }),
    ).insert(UiCharacterInfos).insert(UiActionPoints).id();

    //== Nom du personnage.
    let player_name_tag = commands.spawn((
        TextBundle::from_section(
            player_name,
            TextStyle {
                font: assets.font.clone(),  
                font_size: INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(8.)),            
            ..default()
        }),
        // Because this is a distinct label widget and
        // not button/list item text, this is necessary
        // for accessibility to treat the text accordingly.
        Label,
        //UiGameInterface,
    )).id();

    // === L'endroit où sont mis les Chunks.    //TODO : fonction a part?
    let chunk_container = commands.spawn(NodeBundle {
        style: Style {
            ..default()
        },
        ..default()
    }).insert(UiChunkContainer).id();     // ChunkContainer utile?

    // TODO : Ici on fait une mise à jour des PV.
    let mut chunk_list:Vec<Entity> = Vec::new();

    // Autant de Chunk que de PV.
    for i in 1..=player_health_max {
        // Couleur des HP 
        let mut border_color = Color::rgb(0.5, 0.0, 0.0);
        let mut background_color = Color::rgb(0.9, 0.0, 0.0 );

        // Si on affiche des Chunks au delà du Health Current, on les assombri: ils sont vides.
        if i > player_health_current {
            border_color = Color::rgb(0.1, 0.1, 0.1);
            background_color = Color::rgba(0.0, 0.0, 0.0, 1.0 );
        }

        // Le chunk en lui-même.
        // Le cadre.
        let chunk = commands.spawn(NodeBundle {
            style: Style {
                width: Val::Px(INTERFACE_HP_CHUNK_WIDTH),//(8.0),
                height: Val::Px(INTERFACE_HP_CHUNK_HEIGHT), //(16.0),
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
            //Le contenant.
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
        }).insert(UiChunk).id();
        chunk_list.push(chunk);
    }
    

    for chunk in chunk_list {
        commands.entity(chunk_container).add_child(chunk);
    }

    // == On mets tout les uns dans les autres.    
    commands.entity(container).add_child(interface);
    commands.entity(interface).add_child(player_action_display);
    commands.entity(interface).add_child(player_name_tag);
    commands.entity(interface).add_child(chunk_container);
    
  
}


pub fn draw_ui_game_character_infos_old(
    mut commands: Commands,
    assets: Res<GraphicsAssets>,
    interface_query: Query<Entity, With<UiCharacterInfos>>,
    player_info_query: Query<(Entity, Option<&Name>, &Health), With<Player>>,       //player_info_query: Query<(Entity, &Name, &Health), With<Player>>  // Retrait du Name car au load Save on le perds.
    player_actions_query: Query<(Entity, &ActionPoints), With<Player>>,
    ui_main_q: Query<(Entity, &UiMainWindow)>,
) {
    //println!("DEBUG: draw ui_game_character_infos");
    clear_ui_game_character_infos(interface_query, &mut commands);       

    let mut player_name = "Unkwnown Runner";
    let mut player_health_max = INTERFACE_HP_CHUNK_MAX;
    let mut player_health_current = INTERFACE_HP_CHUNK_MAX;
    if let Ok(player_infos) = player_info_query.get_single() {
        //println!("DEBUG : draw interface: Player info OK");
        let (_p_entity, p_name, p_health) = player_infos;   //let (_p_entity, p_name, p_health) = player_infos; // Retrait du name car perdu au save.
        if let Some(name) = p_name {
            player_name = name.as_str();
        }
        //player_name = p_name.as_str();
        player_health_max = p_health.max;
        player_health_current = p_health.current;
        println!("DEBUG: Interface: Player health current is {} and max {}", player_health_current, player_health_max);
    } else {
        //println!("DEBUG : draw: not player");
    }
    let mut action_points = 0;
    if let Ok(player_action_points) = player_actions_query.get_single() {
        let (_p_entity_action, p_action) = player_action_points;
        action_points = p_action.current;
    }


    let container = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexEnd,
            bottom: Val::Px(0.),
            ..default()
        },
        ..default()
    }).insert(UiGameInterface).insert(UiCharacterInfos).id(); 

    let player_action_display = commands.spawn(
        TextBundle::from_section(
            format!("{}",action_points),
            TextStyle { 
                //font: asset_server.load("fonts/PressStart2P-vaV7.ttf"),
                font: assets.font.clone(),  
                font_size: INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE,
                color: Color::YELLOW,
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(8.)),            
            ..default()
        }),
    ).insert(UiCharacterInfos).id();

    let player_name_tag = commands.spawn((
        TextBundle::from_section(
            player_name,
            TextStyle {
                //font: asset_server.load("fonts/PressStart2P-vaV7.ttf"),
                font: assets.font.clone(),  
                font_size: INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(8.)),            
            ..default()
        }),
        // Because this is a distinct label widget and
        // not button/list item text, this is necessary
        // for accessibility to treat the text accordingly.
        Label,
        //UiGameInterface,
    )).id();

    let chunk_container = commands.spawn(NodeBundle {
        style: Style {
            ..default()
        },
        ..default()
    }).id();  
 

    let mut chunk_list:Vec<Entity> = Vec::new();
    for i in 1..=player_health_max {
        let mut border_color = Color::rgb(0.5, 0.0, 0.0);
        let mut background_color = Color::rgb(0.9, 0.0, 0.0 );
        if i > player_health_current {
            border_color = Color::rgb(0.1, 0.1, 0.1);
            background_color = Color::rgba(0.0, 0.0, 0.0, 1.0 );
        }

        let chunk = commands.spawn(NodeBundle {
            style: Style {
                width: Val::Px(INTERFACE_HP_CHUNK_WIDTH),//(8.0),
                height: Val::Px(INTERFACE_HP_CHUNK_HEIGHT), //(16.0),
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
    
    commands.entity(container).add_child(player_action_display);
    commands.entity(container).add_child(player_name_tag);
    commands.entity(container).add_child(chunk_container);
    
  
}
