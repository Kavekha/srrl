use bevy::prelude::*;


const INTERFACE_GLOBAL_HEIGHT: f32 = 40.;
const INTERFACE_GLOBAL_WIDTH: f32 = 96.;
const INTERFACE_NAME_HEIGHT: f32 = 32.;
const INTERFACE_NAME_WIDTH: f32 = 20.;

const INTERFACE_HP_BLOCK_HEIGHT: f32 = 32.;
const INTERFACE_HP_BLOCK_WIDTH: f32 = 300.;
const INTERFACE_HP_CHUNK_MARGIN: f32 = 2.;
const INTERFACE_HP_CHUNK_HEIGHT: f32 = 24.;
const INTERFACE_HP_CHUNK_WIDTH: f32 = 16.;

const INTERFACE_HP_CHUNK_MAX: i32 = 20;




#[derive(Component)]
pub struct InterfaceGame;

#[derive(Component)]
pub struct InterfacePlayerName;

#[derive(Component)]
pub struct InterfaceHealthChunk;


pub fn draw_interface(
    mut commands: Commands,
    interface_query: Query<Entity, With<InterfaceGame>>
) {
    clear_interface(&mut commands, interface_query);

    let container = commands.spawn((
        InterfaceGame,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {bottom:Px(0.), left: Px(0.), right: Px(0.), top: Px(0.)},
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.), Val::Px(INTERFACE_GLOBAL_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        }))
        .id();
        
    for i in 0..INTERFACE_HP_CHUNK_MAX {
        let mut margin = UiRect::all(Val::Px(INTERFACE_HP_CHUNK_MARGIN));
        margin.bottom = Val::Px(INTERFACE_HP_CHUNK_MARGIN);
        let chunk = helpers::get_chunk(
            &mut commands,
            Size::new(Val::Px(INTERFACE_HP_CHUNK_WIDTH, Val::Px(INTERFACE_HP_CHUNK_HEIGHT)),
            margin,
            &assets.textures["hp_chunk"],
        ));
    }
}

fn clear_interface(
    commands: &mut Commands,
    interface_query: Query<Entity, With<InterfaceGame>>
) {
    for entity in interface_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}