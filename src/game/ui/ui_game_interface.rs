use bevy::prelude::*;

use crate::{
    engine::asset_loaders::GraphicsAssets, 
    game::{combat::combat_system::components::ActionPoints, pieces::components::Health, player::Player, ui::{INTERFACE_GLOBAL_PLAYER_NAME_FONT_SIZE, INTERFACE_HP_CHUNK_HEIGHT, INTERFACE_HP_CHUNK_MAX, INTERFACE_HP_CHUNK_WIDTH}}, 
};

use super::{components::{UiCharacterInfos, UiGameInterface, UiMainWindow}, ReloadUiEvent};


// ====== CONST FOR UI CHARACTERS =====
const COLOR_BORDER_CHUNK_HEALTH_FULL:Color = Color::rgb(0.5, 0.0, 0.0);
const COLOR_BACKGROUND_CHUNK_HEALTH_FULL:Color = Color::rgb(0.9, 0.0, 0.0 );
const COLOR_BORDER_CHUNK_HEALTH_EMPTY:Color = Color::rgb(0.1, 0.1, 0.1);
const COLOR_BACKGROUND_CHUNK_HEALTH_EMPTY:Color = Color::rgba(0.0, 0.0, 0.0, 1.0 );


#[derive(Component)]
pub struct UiChunk;

// Utile?
#[derive(Component)]
pub struct UiChunkContainer;

#[derive(Component)]
pub struct UiActionPoints;


pub fn update_ui_character_health(
    mut ev_ui: EventReader<ReloadUiEvent>,
    //mut ui_border_q: Query<
    mut ui_border_n_background_q: Query<(&mut BackgroundColor, &mut BorderColor), With<UiChunk>>,
    player_health_q: Query<&Health, With<Player>>, 
){
    //REMINDER : Pour note, cela ne change pas le color_border, car: component Chunk sur Border & sur son enfant Background. Background est considéré comme ayant les deux dans la Query.
    for _event in ev_ui.read() {
        //println!("Je dois mettre à jour les Chunks.");
        let Ok(player_health) = player_health_q.get_single() else { continue;};

        let mut nb_chunks = 1;
        for (mut background_color, mut border_color) in &mut ui_border_n_background_q {
            if nb_chunks <= player_health.current {
                *border_color = COLOR_BORDER_CHUNK_HEALTH_FULL.into();
                *background_color = COLOR_BACKGROUND_CHUNK_HEALTH_FULL.into();
            } else {
                *border_color =  COLOR_BORDER_CHUNK_HEALTH_EMPTY.into();
                *background_color = COLOR_BACKGROUND_CHUNK_HEALTH_EMPTY.into();
            }
            nb_chunks += 1;
        }
    }
}

pub fn update_ui_character_action_points(
    mut ev_ui: EventReader<ReloadUiEvent>,
    player_actions_query: Query<(Entity, &ActionPoints), With<Player>>,
    mut ap_text_q: Query<&mut Text, With<UiActionPoints>>,
){
    for _event in ev_ui.read() {
        //println!("Je dois mettre à jour les Action Points dans Ui Character.");

        let mut action_points = 0;
        if let Ok(player_action_points) = player_actions_query.get_single() {
            //println!("Points d'action du joueur récupéré!");
            let (_p_entity_action, p_action) = player_action_points;
            action_points = p_action.current;
        } 
        // On modifie le contenu.
        for mut text in &mut ap_text_q {
            //println!("J'ai du texte à modifier pour mon Ui Character: les action points dépensés.");
            text.sections[0].value = format!("{action_points}");    // ATTENTION: Si la section change, ca fout vite la merde avec du gros crash panic....
        }
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
            width: Val::Percent(40.0),
            height: Val::Percent(10.0),
            align_content: AlignContent::FlexEnd,   
            justify_content: JustifyContent::FlexStart, 
            align_items: AlignItems::FlexEnd,
            flex_direction: FlexDirection::Row,            
            bottom: Val::Px(5.),
            ..default()
        },
        ..default()
    }).insert(UiGameInterface).id();  

    //== Points d'action.
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

    // === L'endroit où sont mis les Chunks.
    let chunk_container = commands.spawn(NodeBundle {
        style: Style {
            ..default()
        },
        ..default()
    }).insert(UiChunkContainer).id();     // ChunkContainer utile?

    let mut chunk_list:Vec<Entity> = Vec::new();

    // Autant de Chunk que de PV.
    for i in 1..=player_health_max {
        // Couleur des HP 
        let mut border_color = COLOR_BORDER_CHUNK_HEALTH_FULL;
        let mut background_color = COLOR_BACKGROUND_CHUNK_HEALTH_FULL;

        // Si on affiche des Chunks au delà du Health Current, on les assombri: ils sont vides.
        if i > player_health_current {
            border_color = COLOR_BORDER_CHUNK_HEALTH_EMPTY;
            background_color = COLOR_BACKGROUND_CHUNK_HEALTH_EMPTY;
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
            }).insert(UiChunk);  
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
