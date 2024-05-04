use bevy::prelude::*;

use super::{components::{MenuButtonAction, OnScreenMenu}, NORMAL_BUTTON, TEXT_COLOR};



fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
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
fn item_rect(builder: &mut ChildBuilder, color: Color) {
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

fn item_rect_double(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_column: GridPlacement::span(8),                
                grid_row: GridPlacement::span(2),
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

fn item_rect_triple(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_column: GridPlacement::span(8),
                grid_row: GridPlacement::span(3),
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


//https://bevyengine.org/examples/UI%20(User%20Interface)/grid/
pub fn spawn_selection_menu(
    mut commands: Commands, asset_server: Res<AssetServer>
) {
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
                            grid_template_rows: RepeatedGridTrack::flex(8, 1.0),
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

                        item_rect(builder, Color::GRAY);   // Ici il y aura le choix du nom.

                        item_rect_triple(builder, Color::BLACK);     // Choix Kind
                        item_rect(builder, Color::GRAY);     // Description Kind.
                        
                        item_rect_double(builder, Color::BLACK);   // Choix Job
                        item_rect(builder, Color::GRAY);     // Description Job.
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
                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
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
                    builder.spawn(TextBundle::from_section(
                        "Strength : ***** \n Agility : ** \n Logic : * \n",
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            ..default()
                        },
                    ));
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
