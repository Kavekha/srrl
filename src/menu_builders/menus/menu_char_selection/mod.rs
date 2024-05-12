use bevy::prelude::*;

pub mod select_char_menu;
pub mod components;

use crate::{
    engine::asset_loaders::GraphicsAssets, menu_builders::menus::{components::{MenuButtonAction, OnScreenMenu}, 
    menu_char_selection::select_char_menu::{item_kind_illustration, item_rect, item_rect_archetype_selection_choice, item_rect_job_selection_title, item_rect_metatype_selection_choice, item_rect_metatype_selection_title, item_skills_display, item_stat_display, spawn_nested_text_bundle}, 
    NORMAL_BUTTON, TEXT_COLOR}, raws::{get_job, get_kind, get_playable_jobs, get_playable_kinds, load_raws, RAWS}};

use self::components::PlayerCreation;


//https://bevyengine.org/examples/UI%20(User%20Interface)/grid/
pub fn spawn_selection_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut player_creation: ResMut<PlayerCreation>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<GraphicsAssets>,
) {
    load_raws();
    let playable_kinds = get_playable_kinds(&RAWS.lock().unwrap());
    if playable_kinds.is_empty() { panic!("No playable kinds available.")};

    println!("Playable kinds are : {:?}", playable_kinds);
    let mut names_n_models = Vec::new();
    for kind in playable_kinds {
        if let Some(raw) = get_kind(&RAWS.lock().unwrap(), &kind) {
            if let Some(renderable) = &raw.renderable {
                names_n_models.push(((raw.reference.clone(), raw.name.clone()),renderable.model.clone()));
            }
        }
    }
    let ref_name = &names_n_models[0].0;
    player_creation.kind = (ref_name.0.clone(), ref_name.1.clone());
    player_creation.model = names_n_models[0].1.clone();

    let playable_jobs = get_playable_jobs(&RAWS.lock().unwrap());
    if playable_jobs.is_empty() { panic!("No playable job available.")};
    println!("Playable jobs are : {:?}", playable_jobs);
    let mut jobs = Vec::new();
    for job in playable_jobs {
        if let Some(raw) = get_job(&RAWS.lock().unwrap(), &job) {
            jobs.push((raw.reference.clone(), raw.name.clone()));
        }
    }
    player_creation.job = (jobs[0].0.clone(), jobs[0].1.clone());   // reference:name

    let font = asset_server.load("fonts/PressStart2P-vaV7.ttf"); 

    let button_style = Style {
        width: Val::Px(125.0),
        height: Val::Px(32.5),
        margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 20.0,    //40
        color: TEXT_COLOR,      // AMELIORATION : Mettre dan sle Menu Builder
        ..default()
    };


    // Top-level grid (app frame)
        commands
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    // Make node fill the entirety it's parent (in this case the window)
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
                    //   - The first column will size to the size of it's contents
                    //   - The second column will take up the remaining available space
                    grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                    // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
                    //  - The first row will size to the size of it's contents
                    //  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
                    //  - The third row will be exactly 20px high
                    grid_template_rows: vec![
                        GridTrack::auto(),  // title
                        GridTrack::flex(1.0),   // grids
                        GridTrack::px(40.), // footer.
                    ],
                    ..default()
                },
                background_color: BackgroundColor(Color::ANTIQUE_WHITE),
                ..default()            
            })
            .insert(OnScreenMenu)          
            
            .with_children(|builder| {
                // Header
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            // Make this node span two grid columns so that it takes up the entire top tow
                            grid_column: GridPlacement::span(2),
                            padding: UiRect::all(Val::Px(6.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        spawn_nested_text_bundle(builder, font.clone(), "Character selection");
                    });

                // Main
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            aspect_ratio: Some(1.0),
                            display: Display::Grid,
                            padding: UiRect::all(Val::Px(8.0)),
                            grid_template_columns: RepeatedGridTrack::flex(8, 1.0),
                            grid_template_rows: RepeatedGridTrack::flex(24, 1.0),
                            row_gap: Val::Px(4.0),
                            column_gap: Val::Px(4.0),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::DARK_GRAY),
                        ..default()
                    })
                    .with_children(|builder| {
                        // Note there is no need to specify the position for each grid item. Grid items that are
                        // not given an explicit position will be automatically positioned into the next available
                        // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
                        // style property.
                        item_rect(builder, Color::GRAY);     // title : choose your name. 
                        item_rect(builder, Color::BLACK);     // name chosen. 

                        item_rect(builder, Color::GRAY);     // title : choose your gender. 
                        item_rect(builder, Color::BLACK);     // gender chosen. 

                        item_rect_metatype_selection_title(builder, Color::GRAY, font.clone());     // title: choose your meta-type.
                        // Liste de meta type.                                    
                        for (refname, model) in names_n_models {
                            // TODO : Ajouter la selection par défaut. Pas simple d'inserer un SelectedOption...
                            let (reference, name) = refname;
                            item_rect_metatype_selection_choice(builder, Color::BLACK, font.clone(), reference.to_string(), name.to_string(), model.to_string());                               
                        }                        

                        item_rect_job_selection_title(builder, Color::GRAY, font.clone());     // title : choose your archetype.                        
                        for (job_reference, job_name) in jobs {
                            println!("Job reference added for {:?}, {:?}", job_reference, job_name);
                            // TODO : Ajouter la selection par défaut. Pas simple d'inserer un SelectedOption...
                            item_rect_archetype_selection_choice(builder, Color::BLACK, font.clone(), job_reference.to_string(), job_name.to_string());                               
                        }  
                    });

                // Right side bar (auto placed in row 2, column 2)
                builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        // Align content towards the start (top) in the vertical axis
                        align_items: AlignItems::Start,
                        // Align content towards the center in the horizontal axis
                        justify_items: JustifyItems::Center,
                        // Add 10px padding
                        padding: UiRect::all(Val::Px(10.)),
                        // Add an fr track to take up all the available space at the bottom of the column so that the text nodes
                        // can be top-aligned. Normally you'd use flexbox for this, but this is the CSS Grid example so we're using grid.
                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(),GridTrack::auto(),GridTrack::auto(), GridTrack::auto(), GridTrack::auto(),GridTrack::auto(),GridTrack::fr(1.0)],
                        // Add a 10px gap between rows
                        row_gap: Val::Px(10.),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                })

                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Statistics",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                    ));
                    item_stat_display(builder, Color::BLUE, font.clone(), &player_creation.kind.0, &player_creation.job.0);

                    item_skills_display(builder, Color::BLUE, font.clone(), &player_creation.job.0);

                    // Illustration de la Kind choisie.
                    println!("Je demande un item_kind_illustration pour {:?}", player_creation.model);
                    item_kind_illustration(builder, texture_atlases, assets, player_creation.model.clone());      
            });
            // Footer
            //builder.spawn(NodeBundle::default());    
            builder.spawn(NodeBundle {
                style: Style {
                    // Make this node span two grid column so that it takes up the entire bottom row
                    grid_column: GridPlacement::span(2),                    
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                background_color: BackgroundColor(Color::ANTIQUE_WHITE),
                ..default()
            })
            .with_children(|builder| {
                builder.spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::StartGame
                ))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "START GAME",  //text,
                        button_text_style.clone(),
                    ));
                });
            });
        });

}