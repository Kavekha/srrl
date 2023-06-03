use bevy::{prelude::*, input::{mouse::{MouseWheel, MouseMotion, MouseButtonInput}, ButtonState}, sprite::collide_aabb::collide, window::PrimaryWindow};

use crate::globals::{TILE_SIZE, CHAR_SIZE};

use super::components::{Clickable, MainMenuSelection};


pub fn menu_input_mouse(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    window_query: Query<&Window>,
    button_query: Query<(&Clickable, &Transform)>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut menu_selection: ResMut<MainMenuSelection>,
) {
    for event in mouse_button_input_events.iter() {
        //sr_rl::menus::menus_input: MouseButtonInput { button: Left, state: Pressed }
        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            println!("Left pressed!");

            // Mouse cursor: World coords
            let (camera, camera_transform) = camera_q.single();

            if let Some(world_position) = window_query.single().cursor_position() 
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())      {

                // Any clickable item there?
                let cursor_world_position = Vec3 {x:world_position.x, y:world_position.y, z:0.0};
                for (clickable, transform) in button_query.iter() {
                    println!("Mouse collide between {:?} and {:?} ? : {:?}", world_position, transform.translation, mouse_on_clickable(cursor_world_position, transform.translation, clickable.size));
                }     
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
        // Needed to convert cursor position on window to World coords.
        let (camera, camera_transform) = camera_q.single();
        if let Some(world_position) = window_query.single().cursor_position() 
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())      {
            // Cursor in window
            let cursor_world_position = Vec3 {x:world_position.x, y:world_position.y, z:0.0};
            for (clickable, transform) in button_query.iter() {
                if mouse_on_clickable(cursor_world_position, transform.translation, clickable.size) {
                    menu_selection.selected = clickable.id;
                    println!("Button selected: {:?}", menu_selection.selected)
                }
                //println!("Mouse collide between {:?} and {:?} ? : {:?}", world_position, transform.translation, mouse_on_clickable(cursor_world_position, transform.translation, clickable.size));
            }
        }
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
    //println!("Collision check : Size is {:?} vs {:?}", Vec2::splat(1.0), some_size);

    let collision = collide(
        target_pos,
        Vec2::splat(1.0), 
        some_translation,
        some_size * CHAR_SIZE * 0.9 // TODO : Ugly fix for two buttons too close.
    );
    collision.is_some()
}