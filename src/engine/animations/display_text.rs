use bevy::{prelude::*, text::{BreakLineOn, Text2dBounds}};

#[derive(Event)]
pub struct TextEvent {
    pub entry: String
}


pub fn display_text_box(
    mut ev_text_box: EventReader<TextEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>

){
    for event in ev_text_box.read() {
        let font = asset_server.load("fonts/PressStart2P-vaV7");

        let slightly_smaller_text_style = TextStyle {
            font,
            font_size: 42.0,
            color: Color::WHITE,
        };
    
        let box_size = Vec2::new(300.0, 200.0);
            let box_position = Vec2::new(0.0, -250.0);
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                        ..default()
                    },
                    transform: Transform::from_translation(box_position.extend(0.0)),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(Text2dBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                event.entry.clone(),
                                slightly_smaller_text_style.clone(),
                            )],
                            justify: JustifyText::Left,
                            linebreak_behavior: BreakLineOn::WordBoundary,
                        },
                        text_2d_bounds: Text2dBounds {
                            // Wrap text in the rectangle
                            size: box_size,
                        },
                        // ensure the text is drawn on top of the box
                        transform: Transform::from_translation(Vec3::Z),
                        ..default()
                    });
                });
    }
    
    }
