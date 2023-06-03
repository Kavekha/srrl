use bevy::{prelude::*, input::{mouse::{MouseWheel, MouseMotion, MouseButtonInput}, ButtonState}, sprite::collide_aabb::collide, window::PrimaryWindow};

use crate::globals::{TILE_SIZE, CHAR_SIZE};

use super::components::Clickable;


pub fn menu_input_mouse(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    window_query: Query<&Window>,
    button_query: Query<(&Clickable, &Transform)>
) {
    for event in mouse_button_input_events.iter() {
        //sr_rl::menus::menus_input: MouseButtonInput { button: Left, state: Pressed }
        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            println!("Left pressed!");

            let cursor = window_query.single().cursor_position().unwrap();
            println!("window cursor is {:?}", cursor);
            let cursor_position = Vec3{x:cursor.x, y:cursor.y, z:0.0};

            for (clickable, transform) in button_query.iter() {
                println!("Mouse collide between {:?} and {:?} ? : {:?}", cursor_position, transform.translation, mouse_on_clickable(cursor_position, transform.translation, clickable.size));
            }
        }
        if event.button == MouseButton::Left && event.state == ButtonState::Released {
            println!("Left released!");
        }
        info!("{:?}", event);
    }

    for event in mouse_motion_events.iter() {
        //info!("{:?}", event);
    }

    for event in cursor_moved_events.iter() {
        //info!("{:?}", event);
    }

    for event in mouse_wheel_events.iter() {
        //info!("{:?}", event);
    }
}

pub fn mouse_on_clickable(
    target_pos: Vec3,
    some_translation: Vec3,
    some_size: Vec2
) -> bool {
    let collision = collide(
        target_pos,
        Vec2::splat(CHAR_SIZE * 0.9), 
        some_translation,
        some_size
    );
    collision.is_some()
}