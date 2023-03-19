use rltk::{ RGB, Rltk, VirtualKeyCode };
use specs::prelude::*;
use super::{gamelog::GameLog, RunState, State};


#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection { NewGame, LoadGame, Quit }

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult { NoSelection{ selected : MainMenuSelection }, Selected{ selected: MainMenuSelection } }


pub fn draw_ui(ecs: &World, ctx : &mut Rltk) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    //logs
    let log = ecs.fetch::<GameLog>();
    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 { ctx.print(2, y, s); }
        y += 1;
    }

    //mouse cursor
    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));
}


pub fn main_menu(gs : &mut State, ctx : &mut Rltk) -> MainMenuResult {
    let save_exists = super::saveload_system::does_save_exist();
    let runstate = gs.ecs.fetch::<RunState>();

    ctx.print_color_centered(15, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "SHADOW RUN : Pieces Of Code");

    if let RunState::MainMenu{ menu_selection : selection } = *runstate {
        if selection == MainMenuSelection::NewGame {
            ctx.print_color_centered(24, RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK), "Begin New Game");
        } else {
            ctx.print_color_centered(24, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Begin New Game");
        }

        if save_exists {
            if selection == MainMenuSelection::LoadGame {
                ctx.print_color_centered(25, RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK), "Load Game");
            } else {
                ctx.print_color_centered(25, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Load Game");
            }
        }

        if selection == MainMenuSelection::Quit {
            ctx.print_color_centered(26, RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK), "Quit");
        } else {
            ctx.print_color_centered(26, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Quit");
        }

        match ctx.key {
            None => return MainMenuResult::NoSelection{ selected: selection },
            Some(key) => {
                match key {
                    VirtualKeyCode::Escape => { return MainMenuResult::NoSelection{ selected: MainMenuSelection::Quit } }
                    VirtualKeyCode::Up => {
                        let mut newselection;
                        match selection {
                            MainMenuSelection::NewGame => newselection = MainMenuSelection::Quit,
                            MainMenuSelection::LoadGame => newselection = MainMenuSelection::NewGame,
                            MainMenuSelection::Quit => newselection = MainMenuSelection::LoadGame
                        }
                        if newselection == MainMenuSelection::LoadGame && !save_exists {
                            newselection = MainMenuSelection::NewGame;
                        }
                        return MainMenuResult::NoSelection{ selected: newselection }
                    }
                    VirtualKeyCode::Down => {
                        let mut newselection;
                        match selection {
                            MainMenuSelection::NewGame => newselection = MainMenuSelection::LoadGame,
                            MainMenuSelection::LoadGame => newselection = MainMenuSelection::Quit,
                            MainMenuSelection::Quit => newselection = MainMenuSelection::NewGame
                        }
                        if newselection == MainMenuSelection::LoadGame && !save_exists {
                            newselection = MainMenuSelection::Quit;
                        }
                        return MainMenuResult::NoSelection{ selected: newselection }
                    }
                    VirtualKeyCode::Return => return MainMenuResult::Selected{ selected : selection },
                    _ => return MainMenuResult::NoSelection{ selected: selection }
                }
            }
        }
    }

    MainMenuResult::NoSelection { selected: MainMenuSelection::NewGame }
}