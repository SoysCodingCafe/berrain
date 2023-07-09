use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;

// Constants
pub const BERRY_SPEED: f32 = -400.0;
pub const MAX_BERGOD_SPEED: f32 = 800.0;
pub const MAX_BERSON_SPEED: f32 = 80.0;

pub const BERGOD_ACCELERATION: f32 = 350.0;

pub const HUNGER_RATE: f32 = 0.015;
pub const DAY_RATE: f32 = 0.08;
pub const SPEECH_RATE: f32 = 0.001;
pub const TURN_RATE: f32 = 0.005;

pub const HUNGRY: f32 = 0.7;
pub const STARVING: f32 = 0.4;

pub const HUNGER_EAT_THRESHOLD: f32 = 0.9;
pub const HUNGER_RESTORED_BERRY: f32 = 0.2;

pub const RESPAWN_TIME: f32 = 1.0;
pub const DIALOGUE_DISPLAY_TIME: f32 = 4.0;

pub const STARTING_BERPLE: usize = 5;
pub const STARTING_BERRIES: isize = 9;
pub const STARTING_DAY: usize = 0;

pub const GAME_SPEED_MULTIPLIER: f32 = 5.0;
pub const SACRIFICE_SPEED_MULTIPLIER: f32 = 5.0;

// Enum
pub enum BersonMood{
	Eating,
	Wavering,
	Thankful,
	Flattered,
	Starved,
	Outraged,
	Starving,
	Confused,
	Surprised,
	Hungry,
	Appeased,
}

// Components
#[derive(Component)]
pub struct Bergod;

#[derive(Component)]
pub struct Berson {
	pub id: usize,
	pub berries_held: usize,
}

#[derive(Component)]
pub struct Sky {
	pub rising: bool,
	pub time: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stars;

#[derive(Component)]
pub struct Sun;

#[derive(Component)]
pub struct Glow;

#[derive(Component)]
pub struct EndScreen;

#[derive(Component)]
pub struct Berry;

#[derive(Component)]
pub struct Skull(pub usize);

#[derive(Component)]
pub struct Bounty(pub usize);

#[derive(Component)]
pub struct HeldBerry(pub usize);

#[derive(Component)]
pub struct Bush(pub Timer);

#[derive(Component)]
pub struct Hunger {
	pub hunger: f32,
	pub full_color: Color,
	pub starved: bool,
}

#[derive(Component)]
pub struct BerryBush;

#[derive(Component)]
pub struct BerryText;

#[derive(Component)]
pub struct DayText;

#[derive(Component)]
pub struct BersonText{
	pub speech: String,
	pub timer: Timer,
	pub visible: bool,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Limit {
	pub left: f32,
	pub right: f32,
}

#[derive(Component)]
pub struct Lava;

#[derive(Component)]
pub struct Speech;

// Resources
#[derive(Resource)]
pub struct BerriesHeld(pub isize);

#[derive(Resource)]
pub struct BerriesSacrificed(pub usize);

#[derive(Resource)]
pub struct BerriesGifted {
	pub one: isize,
	pub two: isize,
	pub three: isize,
	pub four: isize,
	pub five: isize,
}

#[derive(Resource)]
pub struct CurrentDay(pub usize);

#[derive(Resource)]
pub struct RemainingBerple(pub usize);

#[derive(Resource)]
pub struct BerryRespawn(pub Timer);

#[derive(Resource)]
pub struct InstanceHandle(pub Handle<AudioInstance>);

#[derive(Resource)]
pub struct Volume(pub f32);

#[derive(Resource)]
pub struct SpeedUpFactor(pub f32);

// 0 - Game
// 1 - All Dead
// 2 - Starved
// 3 - End
#[derive(Resource)]
pub struct GameOver(pub usize);

#[derive(Resource)]
pub struct DeathDayShown(pub bool);

// Events
pub struct UpdateSpeech {
	pub berson: Entity,
	pub mood: BersonMood,
}

// Text Styles
pub fn get_number_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/FiraSans-Bold.ttf"),
		font_size: 150.0,
		color: Color::rgba(0.1, 0.1, 0.1, 1.0),
	}
}

pub fn get_title_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/FiraSans-Bold.ttf"),
		font_size: 160.0,
		color: Color::rgba(0.7, 0.2, 0.2, 1.0),
	}
}

pub fn get_day_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/FiraSans-Bold.ttf"),
		font_size: 80.0,
		color: Color::rgba(0.1, 0.1, 0.1, 1.0),
	}
}

pub fn get_dialogue_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/FiraSans-Bold.ttf"),
		font_size: 30.0,
		color: Color::rgba(0.1, 0.1, 0.1, 1.0),
	}
}