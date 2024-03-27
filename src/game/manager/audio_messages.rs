use bevy::{audio::Volume, ecs:: world::World};

use crate::{
    engine::audios::AudioConfig,
    globals::{MAX_VOLUME, MIN_VOLUME}
};

use super::Message;



pub struct ChangeSoundVolumeMessage {pub modify_value:f32}
impl Message for ChangeSoundVolumeMessage {
    fn execute(&self, world: &mut World) {
        let mut audio_resource = world.resource_mut::<AudioConfig>();
        println!("Change Sound Volume");
        let mut new_volume = (audio_resource.sound_volume.get() + self.modify_value).max(MIN_VOLUME);
        new_volume = new_volume.min(MAX_VOLUME);
        
        audio_resource.sound_volume = Volume::new(new_volume);
    }
}

pub struct ChangeMusicVolumeMessage {pub modify_value:f32}
impl Message for ChangeMusicVolumeMessage {
    fn execute(&self, world: &mut World) {
        let mut audio_resource = world.resource_mut::<AudioConfig>();
        println!("Change Music Volume");
        let mut new_volume = (audio_resource.music_volume.get() + self.modify_value).max(MIN_VOLUME);
        new_volume = new_volume.min(MAX_VOLUME);
        
        audio_resource.music_volume = Volume::new(new_volume);
    }
}