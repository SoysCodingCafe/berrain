use bevy::prelude::*;
use derivables::*;

mod berries;
mod bgm;
mod derivables;
mod movement;
mod setup;
mod text;
#[cfg(debug_assertions)]
mod debug;

fn main() {
	let mut app = App::new();
	app
		.add_plugins(DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window {
					// Stops the game from stopping keyboard shortcuts e.g. F12
					prevent_default_event_handling: false,
					title: "Berrain".to_string(),
					..default()
				}),
				..default()
			})
			.set(AssetPlugin {
				watch_for_changes: true,
				..Default::default()
			})
			.set(ImagePlugin::default_nearest())
		)
		.add_event::<UpdateSpeech>()
		.insert_resource(DeathDayShown(false))
		.insert_resource(SpeedUpFactor(1.0))
		.insert_resource(Volume(0.5))
		.insert_resource(BerriesHeld(STARTING_BERRIES))
		.insert_resource(BerriesSacrificed(0))
		.insert_resource(BerriesGifted {
			one: 0,
			two: 0,
			three: 0,
			four: 0,
			five: 0,
		})
		.insert_resource(GameOver(0))
		.insert_resource(RemainingBerple(STARTING_BERPLE))
		.insert_resource(CurrentDay(STARTING_DAY))
		.insert_resource(BerryRespawn(Timer::from_seconds(RESPAWN_TIME, TimerMode::Once)))
		.add_plugin(bevy_kira_audio::AudioPlugin)
		.add_plugin(setup::SetupPlugin)
		.add_plugin(bgm::BgmPlugin)
		.add_plugin(berries::BerriesPlugin)
		.add_plugin(movement::MovementPlugin)
		.add_plugin(text::TextPlugin)
		;

	{
		#[cfg(debug_assertions)]
		app
			.add_plugin(debug::DebugPlugin)
		;
	}

	app.run();
}