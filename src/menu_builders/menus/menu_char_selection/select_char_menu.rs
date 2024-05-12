use std::cmp::max;

use bevy::prelude::*;

use crate::{engine::asset_loaders::GraphicsAssets, game::ui::components::UiGameInterface, menu_builders::menus::{components::SelectedOption, NORMAL_BUTTON}, raws::{get_base_attributes, get_job, get_kind, RAWS}};

use super::components::{JobProposition, KindProposition, MenuKindDisplay, PlayerCreation, SelectedOptionJob};


pub fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 24.0,
            color: Color::BLACK,
        },
    ));
}


/// Create a coloured rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take it's size from the size of the grid area it occupies.
pub fn item_rect(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,                
                grid_column: GridPlacement::span(8),
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

/* 
pub fn item_rect_double(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_column: GridPlacement::span(8),                
                grid_row: GridPlacement::span(5),
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

pub fn item_rect_triple(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_column: GridPlacement::span(8),
                grid_row: GridPlacement::span(10),
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}
*/

pub fn item_skills_display(
    builder: &mut ChildBuilder, 
    color: Color, 
    font: Handle<Font>,
    job_ref: &str
) {
    let raw_lock = &RAWS.lock().unwrap();
    let Some(raw_job) = get_job(raw_lock, &job_ref) else { return };
    
    // Skills
    if let Some(skills) = &raw_job.skills {
        builder.spawn(TextBundle::from_section(
            format!("- Skills - \n"),        
            TextStyle {
                font: font.clone(),
                font_size: 16.0,
                ..default()
            },
        ));   

        for skill in skills {
            builder.spawn(TextBundle::from_section(
                format!("{:?}:{:?} \n", skill.0, skill.1),        
                TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
            )); 
        }
    }
}


pub fn item_stat_display(
    builder: &mut ChildBuilder, 
    color: Color, 
    font: Handle<Font>,
    kind_ref: &str,
    job_ref: &str
) {
    // TODO : Utiliser dans la creation de perso?
     println!("kind ref : {:?}, job ref : {:?}", kind_ref, job_ref); 
     let raw_lock = &RAWS.lock().unwrap();
     let Some(raw_job) = get_job(raw_lock, &job_ref) else { return };
     let Some(raw_kind) = get_kind(raw_lock, &kind_ref) else { return };
     let Some(raw_base_attributes) =  get_base_attributes(raw_lock, &kind_ref) else { return };

     // Stats
    let mut strength = 0;
    if let Some(base_str) = raw_base_attributes.strength {
        strength += max(base_str, 1);
    }
    if let Some(job_strength) = raw_job.strength {
        strength += max(job_strength, 1);
    }
    let mut agility = 0;
    if let Some(base_agility) = raw_base_attributes.agility {
        agility += max(base_agility, 1);
    }
    if let Some(job_agility) = raw_job.agility {
        agility += max(job_agility, 1);
    }
    let mut logic = 0;
    if let Some(base_logic) = raw_base_attributes.logic {
        logic += max(base_logic, 1);
    }
    if let Some(job_logic) = raw_job.logic {
        logic += max(job_logic, 1);
    }
    builder.spawn(TextBundle::from_section(
        format!("- Attributes - \n Strength : {:?}/{:?} \n Agility : {:?}/{:?} \n Logic : {:?}/{:?}",
        strength, raw_kind.attributes.strength_max,
        agility, raw_kind.attributes.agility_max,
        logic, raw_kind.attributes.logic_max
    ), //format!("{:?} \n {:?}", final_attributes_text, final_skills_text),        
        TextStyle {
            font: font.clone(),
            font_size: 16.0,
            ..default()
        },
    ));  

}

pub fn item_rect_metatype_selection_title(builder: &mut ChildBuilder, color: Color, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,                
                grid_column: GridPlacement::span(8),
                padding: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {            
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            })
            .with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Choose your Meta-type:",
                    TextStyle {
                        font: font.clone(),
                        font_size: 10.0,
                        color: Color::WHITE,                                                                    
                        ..default()
                    }
                ));
            });               
        });
}

pub fn item_rect_job_selection_title(builder: &mut ChildBuilder, color: Color, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,                
                grid_column: GridPlacement::span(8),
                padding: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {            
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            })
            .with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Choose your Archetype:",
                    TextStyle {
                        font: font.clone(),
                        font_size: 10.0,
                        color: Color::WHITE,                                                                    
                        ..default()
                    }
                ));
            });               
        });
}

pub fn item_kind_illustration(
    builder: &mut ChildBuilder,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    //asset_server: Res<AssetServer>,
    assets: Res<GraphicsAssets>,
    model: String
) {
    // TODO : Passer en mode Option pour que ce soit plus propre.
    println!("Model is {:?}", model);
    if model == "" { return };
    println!("Illustration !");
    let texture_handle = assets.textures[&model as &str].clone(); //asset_server.load("characters/{}.png", model);
    let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    builder.spawn(AtlasImageBundle {
        style: Style {
            width: Val::Px(128.),
            height: Val::Px(128.),
            ..default()
        },
        texture_atlas: texture_atlas_handle.into(),
        image: UiImage::new(texture_handle),
        ..default()
    }).insert(MenuKindDisplay { model: model});
}

pub fn item_rect_metatype_selection_choice(
    builder: &mut ChildBuilder,
     _color: Color, 
     font: Handle<Font>, 
     reference: String,
     name: String, 
     model: String
    ) {
    let button_style = Style {
        //width: Val::Px(125.0),
        //height: Val::Px(32.5),
        //margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,                
                grid_column: GridPlacement::span(8),
                grid_row: GridPlacement::span(1),
                padding: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {            
            /* 
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            });*/
            builder.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },            
                KindProposition { 
                    kind : name.clone(),
                    model : model.clone(),
                    reference: reference.clone()
                },                
            ))
            .with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    name.clone(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 15.0,
                        color: Color::WHITE,                                            
                        ..default()
                    }
                ));
            });               
        });
}


pub fn selecting_kind(
    interaction_q: Query<(&Interaction, &KindProposition, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_q: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,       // Ici on récupère l'element déjà selectionné s'il existe.
    mut display_q: Query<&mut MenuKindDisplay>,
    mut commands: Commands,
    mut player_creation: ResMut<PlayerCreation>,   
) {
    for (interaction, kind_proposal, entity) in &interaction_q {
        if *interaction == Interaction::Pressed && player_creation.kind.1 != kind_proposal.kind {
            //Si je presse un bouton qui concerne un Kind different de celui que j'ia deja selectionné =>
            if !selected_q.is_empty() {            
                println!("Selecting kind");
                let (previous_entity, mut previous_bg) = selected_q.single_mut();
                previous_bg.0 = NORMAL_BUTTON.into();
                commands.entity(previous_entity).remove::<SelectedOption>();
            }
            commands.entity(entity).insert(SelectedOption);
            player_creation.kind = (kind_proposal.reference.clone(), kind_proposal.kind.clone());
            player_creation.model = kind_proposal.model.clone();

            if let Ok(mut display) = display_q.get_single_mut() {
                display.model = kind_proposal.model.clone();
            }
        }        
    }
}

// 0.20h : Pour être honnête, c'est bien degueulasse. Mais ca marche.
pub fn updated_kind_display(   
    display_q: Query<(&Parent, &MenuKindDisplay), Changed<MenuKindDisplay>>,  
    mut img_q: Query<(&Parent, &mut UiImage)>,
    mut texture_q: Query<(&Parent, &mut TextureAtlas)>,
    assets: Res<GraphicsAssets>,    
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
) {
    for (parent, display) in display_q.iter() {
        for (img_parent, mut img) in img_q.iter_mut() {
            if img_parent == parent {
                let texture_handle = assets.textures[&display.model as &str].clone(); 
                img.texture = texture_handle;
                break;
            }            
        }
        for (tex_parent, mut texture) in texture_q.iter_mut() {
            if tex_parent == parent {
                let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 1, 1, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                texture.layout = texture_atlas_handle;
                break;
            }
        }
    }
}

pub fn item_rect_archetype_selection_choice(builder: &mut ChildBuilder, _color: Color, font: Handle<Font>, job_reference: String, job_name: String) {
    let button_style = Style {
        //width: Val::Px(125.0),
        //height: Val::Px(32.5),
        //margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,                
                grid_column: GridPlacement::span(8),
                grid_row: GridPlacement::span(1),
                padding: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {            
            /* 
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            });*/
            builder.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },            
                JobProposition { job: job_name.clone(), reference: job_reference.clone()},
            ))
            .with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    job_name.clone(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 15.0,
                        color: Color::WHITE,                                            
                        ..default()
                    }
                ));
            });               
        });
}



pub fn selecting_job(
    interaction_q: Query<(&Interaction, &JobProposition, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_q: Query<(Entity, &mut BackgroundColor), With<SelectedOptionJob>>,       // Ici on récupère l'element déjà selectionné s'il existe.
    mut commands: Commands,
    mut player_creation: ResMut<PlayerCreation>,   
) {
    for (interaction, job_proposal, entity) in &interaction_q {
        if *interaction == Interaction::Pressed && &player_creation.job.0 != &job_proposal.job {
            if !selected_q.is_empty() {       
                let (previous_entity, mut previous_bg) = selected_q.single_mut();
                previous_bg.0 = NORMAL_BUTTON.into();
                commands.entity(previous_entity).remove::<SelectedOptionJob>();
            }
            commands.entity(entity).insert(SelectedOptionJob);
            player_creation.job = (job_proposal.reference.clone(), job_proposal.job.clone());
            println!("Player job is now : {:?}", player_creation.job);
        }  
    }
}