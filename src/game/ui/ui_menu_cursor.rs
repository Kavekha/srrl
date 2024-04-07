// https://github.com/bevyengine/bevy/discussions/8613 (Erwan 88)
// Ce curseur n'est utilisable que dans les Menus car liés à du X/Y UI, qui est different de celui de la camera2d IN GAME.

/* 
#[derive(Component)]
pub struct MenuCursor {}

pub fn setup_ui_cursor(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;
    let cursor_spawn: Vec3 = Vec3::ZERO;

    commands.spawn((
        ImageBundle {
            image: asset_server.load("cursors/cursor_targeting.png").into(),
            style: Style {
                //display: Display::None,
                position_type: PositionType::Absolute,
                //position: UiRect::all(Val::Auto),
                ..default()
            },
            z_index: ZIndex::Global(15),
            transform: Transform::from_translation(cursor_spawn),
            ..default()
        },
        GameCursor {}
    ));
}

pub fn move_ui_cursor(window: Query<&Window>, mut cursor: Query<&mut Style, With<GameCursor>>) {
    let window: &Window = window.single();
    if let Some(position) = window.cursor_position() {
        let mut img_style = cursor.single_mut();
        img_style.left = Val::Px(position.x - 2.0);
        img_style.bottom = Val::Px(position.y + 24.0);      // + to go down
    }
}
*/