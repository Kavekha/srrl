use bevy::prelude::*;

use crate::game::{actions::Action, combat::{components::ActionPoints}, ui::ReloadUiEvent, rules::consume_actionpoints};


pub struct EndTurnAction(pub Entity);
impl Action for EndTurnAction {
   fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        println!("EndTurnAction: Execute.");
        let Some(mut action_points) = world.get_mut::<ActionPoints>(self.0) else { return Err(())};
        consume_actionpoints(&mut action_points, 1000);

        let mut update_interface_event = world.resource_mut::<Events<ReloadUiEvent>>(); //world.get_resource_mut::<EventWriter<ReloadUiEvent>>();
        update_interface_event.send(ReloadUiEvent);
        Ok(Vec::new())   
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
 }