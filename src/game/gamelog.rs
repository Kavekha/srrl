use bevy::prelude::*;

use super::ui::game_interface::display_log_ui;

pub struct GameLogsPlugin;

impl Plugin for GameLogsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Gamelog { entries: Vec::new()})

            .add_event::<LogEvent>()
            
            .add_systems(Update, display_log_ui.run_if(on_event::<LogEvent>()))
            ;
    }
}

#[derive(Resource)]
pub struct Gamelog {
    pub entries : Vec<String>
}

#[derive(Event)]
pub struct LogEvent{pub entry: String}

/* 
fn display_log(
    mut ev_log: EventReader<LogEvent>,
    mut gamelog: ResMut<Gamelog>
){
    for event in ev_log.read() {
        gamelog.entries.push(event.entry.to_string());
        println!("{}", event.entry);
    }    
}*/