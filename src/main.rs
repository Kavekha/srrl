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

pub struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Shadowrun:poc")
        .build()?;
    let mut gs = State{
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map_rooms_and_corridors());

    gs.ecs
    .create_entity()
    .with(Position { x: 40, y: 25 })
    .with(Renderable {
        glyph: rltk::to_cp437('@'),
        fg: RGB::named(rltk::YELLOW),
        bg: RGB::named(rltk::BLACK),
    })
    .with(Player{})
    .build();

    rltk::main_loop(context, gs)
}