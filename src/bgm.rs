use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioTween, AudioControl, AudioInstance};

use crate::derivables::*;

pub struct BgmPlugin;

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_startup_systems((
				play_bgm,
			))
			.add_systems((
				change_volume,
				update_volume,
			))
		;
	}
}

fn play_bgm(
	mut commands: Commands,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
) {
	let handle = audio.play(asset_server.load("bgm/the_tragedy_of_berries.ogg"))
	.fade_in(AudioTween::linear(Duration::new(0, 500)))
	.loop_from(34.259)
	.with_volume(0.5)
	.handle()
	;

	commands.insert_resource(InstanceHandle(handle));
}

fn change_volume(
	keyboard: Res<Input<KeyCode>>,
	mut volume: ResMut<Volume>,
) {
	if keyboard.just_pressed(KeyCode::M) {
		volume.0 = 0.0;
	} else if keyboard.just_pressed(KeyCode::Key0) {
		volume.0 = 0.0;
	} else if keyboard.just_pressed(KeyCode::Key1) {
		volume.0 = 0.1;
	} else if keyboard.just_pressed(KeyCode::Key2) {
		volume.0 = 0.2;
	} else if keyboard.just_pressed(KeyCode::Key3) {
		volume.0 = 0.3;
	} else if keyboard.just_pressed(KeyCode::Key4) {
		volume.0 = 0.4;
	} else if keyboard.just_pressed(KeyCode::Key5) {
		volume.0 = 0.6;
	} else if keyboard.just_pressed(KeyCode::Key6) {
		volume.0 = 0.7;
	} else if keyboard.just_pressed(KeyCode::Key7) {
		volume.0 = 0.8;
	} else if keyboard.just_pressed(KeyCode::Key8) {
		volume.0 = 0.9;
	} else if keyboard.just_pressed(KeyCode::Key9) {
		volume.0 = 1.0;
	} else if keyboard.just_pressed(KeyCode::Plus) {
		volume.0 = (volume.0 + 0.1).clamp(0.0, 1.0);
	} else if keyboard.just_pressed(KeyCode::Minus) {
		volume.0 = (volume.0 - 0.1).clamp(0.0, 1.0);
	}
}

fn update_volume(
	bgm_handle: Res<InstanceHandle>,
	volume: Res<Volume>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
	if let Some(instance) = audio_instances.get_mut(&bgm_handle.0) {
		instance.set_volume(volume.0 as f64, AudioTween::default());
    }
}