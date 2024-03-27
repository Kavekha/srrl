
use bevy::{audio::Volume, prelude::*};

use crate::{
    game::menus::{
        menu_builder::MenuButtonAction::SettingsAudioChange,
        components::OnScreenMenu},
    globals::{NORMAL_BUTTON, TEXT_COLOR},
    engine::{asset_loaders::GraphicsAssets, audios::AudioType},
};

use super::components::MenuButtonAction;


//MenuBuilder v2
#[derive(Clone)]
pub struct Action {pub action: MenuButtonAction, pub text:String}
#[derive(Clone)]
pub struct Header {pub text: String}
#[derive(Clone)]
pub struct Description {pub text: String}
#[derive(Clone)]
pub struct Image {pub name: String}
#[derive(Clone)]
pub struct Footer {pub text: String}
#[derive(Clone)]
pub struct Slider {pub button_1_value:f32, pub button_2_value:f32, pub original_value:Volume, pub text:String, pub audio_type:AudioType}


#[derive(Clone)]
pub enum MenuItem{
    Action(Action),
    Header(Header),
    Description(Description),
    Image(Image),
    Footer(Footer),
    Slider(Slider)
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
    pub fn image(name:&str
    ) -> MenuItem {
        MenuItem::Image(Image{name: name.to_string()})
    }
    pub fn footer(text: &str
    ) -> MenuItem {
        MenuItem::Footer(Footer{text: text.to_string()})
    } 
    pub fn slider(button_1_value:f32, button_2_value:f32, original_value:Volume, text:&str, audio_type:AudioType    // TODO : Supporter autre chose pour le type. Transmettre la resource?
    ) -> MenuItem {
        MenuItem::Slider(Slider{button_1_value: button_1_value, button_2_value:button_2_value, original_value:original_value, text:text.to_string(), audio_type:audio_type})
    }
}

pub struct Menu{
    pub id: String,
    pub entries: Vec<MenuItem>
}
impl Menu{
    pub fn new(id: &str, entries: Vec<MenuItem>) -> Menu {
        let menu = Menu{
            id: id.to_string(),
            entries: entries
        };
        menu
    } 
    pub fn add(&mut self, menu_item: MenuItem){
        self.entries.push(menu_item);
    }
}

// WORK IN PROGRESS
pub fn spawn_recap_menu(
    commands: &mut Commands,
    graph_assets: Res<GraphicsAssets>,
    menu: &Menu,
) {
    let mut images:Vec<Image> = Vec::new();
    let mut headers:Vec<Header>= Vec::new();
    let mut descriptions:Vec<Description> = Vec::new();
    let mut actions:Vec<Action> = Vec::new();
    let mut footers:Vec<Footer> = Vec::new();
    let mut sliders:Vec<Slider> = Vec::new();

    for item in menu.entries.iter() {
        match item {
            MenuItem::Header(header) => headers.push(header.clone()),
            MenuItem::Description(description) => descriptions.push(description.clone()),
            MenuItem::Action(action) => actions.push(action.clone()),
            MenuItem::Image(image) => images.push(image.clone()),
            MenuItem::Footer(footer) => footers.push(footer.clone()),
            MenuItem::Slider(slider) => sliders.push(slider.clone()),
            //_ => println!("This MenuItem is not supported.")
        };
    }

    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(125.0),
        height: Val::Px(32.5),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    /*
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
        */
    let button_text_style = TextStyle {
        font_size: 20.0,    //40
        color: TEXT_COLOR,
        ..default()
    };

    // The full screen as a UI element
    let screen_menu = commands.spawn(
        (
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnScreenMenu,
        )).id();
    
    // Illustration.
    for image in images.iter() {
        println!("Image name is {}", image.name);   //DEBUG
        let image = graph_assets.images[image.name.as_str()].clone().into(); // graph_assets.images[image.name.as_str()].clone().into().expect("something bad happened");
        let logo = commands.spawn(ImageBundle {
                            image: image,   //image: graph_assets.logo.clone().into(),
                            ..default()
        }).id();
        commands.entity(screen_menu).push_children(&[logo]);
        println!("Image devrait être chargée.");
    }
    

    // Header
    for header in headers.iter() {
        let menu_header = commands.spawn(TextBundle::from_section(
            header.text.clone(),   //"YOU DIED.",
            TextStyle {
                font: graph_assets.font.clone(),
                font_size: 30.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
        )).id();
        commands.entity(screen_menu).push_children(&[menu_header]);
    }

    // Description
    for description in descriptions.iter() {
        let menu_description = commands.spawn(TextBundle::from_section(
            description.text.clone(),   //"YOU DIED.",
            TextStyle {
                    font: graph_assets.font.clone(),
                    font_size: 15.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                },
            )).id();
        commands.entity(screen_menu).push_children(&[menu_description]);
    }

    // Si y a des options, on mets un cadre.
    if actions.len() > 0 || sliders.len() > 0 {
        let menu_border = commands.spawn(NodeBundle {
            // Cadre du menu en lui-même.
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            //background_color: Color::CRIMSON.into(),
            ..default()
        }).id();

        commands.entity(screen_menu).push_children(&[menu_border]);

        // Sliders 
        for slider in sliders.iter() {
            // This is the "button"
            let slider_border = commands.spawn(NodeBundle {
                // Cadre pour l'ensemble de bouton du bidule.
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                //background_color: Color::CRIMSON.into(),
                ..default()
            }).id();
            commands.entity(menu_border).push_children(&[slider_border]);

                // This button Reduce the value
                let slider_button_decrease = commands.spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    SettingsAudioChange {modify_volume_by: -0.1, audio_type: slider.audio_type.clone()} //slider,    //Reduce value?   // Une image a la place?
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "<", //action.text.clone(),  //text,
                        button_text_style.clone(),
                    ));
                }).id();
                commands.entity(slider_border).push_children(&[slider_button_decrease]);

                // Display the value.
                let slider_button_value = commands.spawn(
                    TextBundle::from_section(
                        format!("{} ({:.1})", slider.text, slider.original_value.get()), //action.text.clone(),  //text,
                        button_text_style.clone(),
                    )).id();
                commands.entity(slider_border).push_children(&[slider_button_value]);

                // Increase value
                let slider_button_increase = commands.spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    SettingsAudioChange {modify_volume_by: 0.1, audio_type: slider.audio_type.clone()}   //Reduce value?   // Une image a la place?
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        ">", //action.text.clone(),  //text,
                        button_text_style.clone(),
                    ));
                }).id();
                commands.entity(slider_border).push_children(&[slider_button_increase]);
        }

        // Options.
        for action in actions.iter() {
            let action_button = commands.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                action.action.clone(),    //action,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    action.text.clone(),  //text,
                    button_text_style.clone(),
                ));
            }).id();
            commands.entity(menu_border).push_children(&[action_button]);
        };       

        // Footers.
        if footers.len() > 0 {
            let menu_down = commands.spawn(NodeBundle {
                // Cadre du menu en lui-même.
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                //background_color: Color::CRIMSON.into(),
                ..default()
            }).id();
            commands.entity(screen_menu).push_children(&[menu_down]);

            for footer in footers.iter() {
                let menu_footer = commands.spawn(TextBundle::from_section(
                    footer.text.clone(),  //text,
                    button_text_style.clone(),
                )).id();
                commands.entity(menu_down).push_children(&[menu_footer]);
            };
        }
    }      
}
