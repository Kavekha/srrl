use bevy::{audio::Volume,  prelude::*};

use crate::engine::audios::AudioType;

use super::menus::components::MenuButtonAction;


//MenuBuilder v2

// 0.20a : utilisé par game\player
#[derive(Component, Default)]
pub struct ScrollingList {
    pub position: f32,
}

//0.20a Techniquement possible de les avoir en private, mais elles sont utilisés par MenuItem qui doit être public. Les attributs peuvent être privés.
#[derive(Clone)]
pub struct Action {pub action: MenuButtonAction, pub text:String}
#[derive(Clone)]
pub struct Header {pub text: String}
#[derive(Clone)]
pub struct Description {pub text: String}
#[derive(Clone)]
pub struct Illustration { pub name: String}
#[derive(Clone)]
pub struct Footer { pub text: String}
#[derive(Clone)]
pub struct Slider { pub original_value:Volume, pub text:String, pub audio_type:AudioType}
#[derive(Clone)]
pub struct ScrollingText {pub text: String}

#[derive(Clone)]
pub enum MenuItem{
    Action(Action),
    Header(Header),
    Description(Description),
    Illustration(Illustration),
    Footer(Footer),
    Slider(Slider),
    ScrollingText(ScrollingText),
}

impl MenuItem{
    pub fn action(action:MenuButtonAction, text:&str
    ) -> MenuItem {
        MenuItem::Action(Action{action: action, text:text.to_string()})
    }
    pub fn header(text:&str
    ) -> MenuItem {
        MenuItem::Header(Header{text:text.to_string()})
    }
    pub fn description(text:&str
    ) -> MenuItem {
        MenuItem::Description(Description{text:text.to_string()})
    }
    pub fn illustration(name:&str
    ) -> MenuItem {
        MenuItem::Illustration(Illustration{name: name.to_string()})
    }
    pub fn footer(text: &str
    ) -> MenuItem {
        MenuItem::Footer(Footer{text: text.to_string()})
    } 
    pub fn slider( original_value:Volume, text:&str, audio_type:AudioType    // AMELIORATION : Supporter autre chose pour le type. Transmettre la resource?
    ) -> MenuItem {
        MenuItem::Slider(Slider{ original_value:original_value, text:text.to_string(), audio_type:audio_type})
    }
    pub fn scrolling_text(text: &str
    ) -> MenuItem {
        MenuItem::ScrollingText(ScrollingText{text: text.to_string()})
    }
}