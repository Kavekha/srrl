use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

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


#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

pub struct State {
    pub ecs: World,
    pub runstate : RunState
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused
        } else {
            player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

        gui::draw_ui(&self.ecs, ctx);
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
        ecs: World::new(),
        runstate : RunState::Running
    };

    // Create component
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    //  create map
    //let (rooms, map) = new_map_rooms_and_corridors();
    let map : Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();   // Player will be created at the center of the first room
    gs.ecs.insert(map);

    // create player char
    gs.ecs
    .create_entity()
    .with(Position { x: player_x, y: player_y })
    .with(Renderable {
        glyph: rltk::to_cp437('@'),
        fg: RGB::named(rltk::YELLOW),
        bg: RGB::named(rltk::BLACK),
    })
    .with(Player{})
    .build();

    // game logs
    gs.ecs.insert(gamelog::GameLog{ entries : vec!["Welcome to ShadowRun:Pieces Of Code!".to_string()] });

    // Play main loop
    rltk::main_loop(context, gs)
}