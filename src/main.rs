extern crate serde;
use rltk::{GameState, Rltk, Point};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

mod map;
pub use map::*;
mod components;
pub use components::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod gui;
mod gamelog;
mod saveload_system;
mod spawner;


#[derive(PartialEq, Copy, Clone)]
pub enum RunState { 
    PreRun,
    AwaitingInput, 
    Running,
    MainMenu { menu_selection : gui::MainMenuSelection },
    SaveGame
 }

pub struct State {
    pub ecs: World
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        ctx.cls();

        match newrunstate {
            RunState::MainMenu { .. } => {}
            _ => {
                draw_map(&self.ecs, ctx);

                {
                    let positions = self.ecs.read_storage::<Position>();
                    let renderables = self.ecs.read_storage::<Renderable>();
                    for (pos, render) in (&positions, &renderables).join() {
                        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
                    }
            
                    gui::draw_ui(&self.ecs, ctx);
                }
            }
        }

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::Running => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput
            }
            RunState::MainMenu { ..} => {
                let result = gui::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection { selected } => newrunstate = RunState::MainMenu{ menu_selection: selected },
                    gui::MainMenuResult::Selected{ selected } => {
                        match selected {
                            gui::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                            gui::MainMenuSelection::LoadGame => {
                                saveload_system::load_game(&mut self.ecs);
                                newrunstate = RunState::AwaitingInput;
                            }
                            gui::MainMenuSelection::Quit => { ::std::process::exit(0); }
                        }
                    }
                }
            }
            RunState::SaveGame => {
                saveload_system::save_game(&mut self.ecs);
                //let data = serde_json::to_string(&*self.ecs.fetch::<Map>()).unwrap();
                //println!("{}", data);
                newrunstate = RunState::MainMenu{ menu_selection : gui::MainMenuSelection::LoadGame };
            }
        }

        // Update newrunstate
        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
    }
}


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple80x50()
        .with_title("Shadowrun:poc")
        .build()?;
    context.with_post_scanlines(true);

    // Create the state and its current default state
    let mut gs = State {
        ecs: World::new()
    };

    // Create component
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<SimpleMarker<SerializeMe>>(); // save
    gs.ecs.register::<SerializationHelper>(); // save

    // save
    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    //  create map
    //let (rooms, map) = new_map_rooms_and_corridors();
    let map : Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();   // Player will be created at the center of the first room

    // create player char
     let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    // RandomNumber, to seed.
    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);

    // game state
    gs.ecs.insert(RunState::MainMenu { menu_selection: gui::MainMenuSelection::NewGame });

    // game logs
    gs.ecs.insert(gamelog::GameLog{ entries : vec!["Welcome to ShadowRun:Pieces Of Code!".to_string()] });

    // Play main loop
    rltk::main_loop(context, gs)
}