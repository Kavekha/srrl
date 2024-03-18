
use bevy::{prelude::*, app::AppExit};

use super::components::MenuButtonAction;

pub struct MenuView{
    pub action: MenuButtonAction,
    pub text: String,
}
impl MenuView {
    pub fn new(action: MenuButtonAction, text:String
    ) -> MenuView {
        let menu = MenuView {action: action, text:text};
        menu
    }
}


pub struct Menu{
    pub pages: Vec<MenuView>
}
impl Menu {
    pub fn new() -> Menu {
        let mut menu = Menu{pages:Vec::new()};
        menu
    }
}