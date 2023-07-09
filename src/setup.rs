use bevy::{prelude::*, render::camera::ScalingMode};

use crate::derivables::*;

// Game State
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
	#[default]
	Boot,
	Menu,
	Level,
	Outro,
}

#[derive(Resource)]
pub struct OrthoSize {
	pub width: f32,
	pub height: f32,
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_state::<GameState>()
			.insert_resource(OrthoSize{width: 1600.0, height: 900.0})
			.add_startup_systems((
				spawn_camera,
			))
		;
	}
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(
	ortho_size: Res<OrthoSize>,
	asset_server: Res<AssetServer>,
	mut commands: Commands,
) {
	commands
		.spawn((
			Camera2dBundle{
				transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
				projection: OrthographicProjection {
					scale: 1.0,
					scaling_mode: ScalingMode::Fixed {width: ortho_size.width, height: ortho_size.height},
					..Default::default()
				},
				..default()
			},
			MainCamera,
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
		sprite: Sprite {custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)), ..Default::default()},
		texture: asset_server.load("sprites/background.png"),
		..Default::default()
		},
		Name::new("Background"),
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 180.0, 10.0)),
		sprite: Sprite {custom_size: Some(Vec2::new(128.0, 128.0)), ..Default::default()},
		texture: asset_server.load("sprites/sun.png"),
		..Default::default()
		},
		Sun,
		Name::new("Sun"),
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 0.0, 900.0)),
		sprite: Sprite {
			custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)),
			color: Color::rgba(1.0, 1.0, 1.0, 0.0), 
			..Default::default()},
		texture: asset_server.load("sprites/end_screen_descend.png"),
		..Default::default()
		},
		EndScreen,
		Name::new("End Screen"),
	));
	
	for i in 1..=5 {
		commands.spawn((SpriteBundle {
			transform: Transform::from_translation(Vec3::new(-810.0 + 270.0 * i as f32, -260.0, 910.0)),
			sprite: Sprite {
				custom_size: Some(Vec2::new(192.0, 192.0)),
				color: Color::rgba(1.0, 1.0, 1.0, 0.0), 
				..Default::default()},
			texture: asset_server.load("sprites/skull.png"),
			..Default::default()
			},
			Skull(i),
			Name::new("Skull"),
		));

		commands.spawn((SpriteBundle {
			transform: Transform::from_translation(Vec3::new(-810.0 + 270.0 * i as f32, -260.0, 910.0)),
			sprite: Sprite {
				custom_size: Some(Vec2::new(192.0, 192.0)),
				color: Color::rgba(1.0, 1.0, 1.0, 0.0), 
				..Default::default()},
			texture: asset_server.load("sprites/bounty.png"),
			..Default::default()
			},
			Bounty(i),
			Name::new("Bounty"),
		));
	}

	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
		sprite: Sprite {custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)), ..Default::default()},
		texture: asset_server.load("sprites/sky.png"),
		..Default::default()
		},
		Sky {
			rising: false,
			time: 1.0,
		},
		Name::new("Sky"),
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 0.0, 30.0)),
		sprite: Sprite {custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)), ..Default::default()},
		texture: asset_server.load("sprites/stars.png"),
		..Default::default()
		},
		Stars,
		Name::new("Stars"),
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(-668.0, 352.0, 100.0)),
		sprite: Sprite {custom_size: Some(Vec2::new(168.0, 148.0)), ..Default::default()},
		texture: asset_server.load("sprites/berries.png"),
		..Default::default()
		},
		Name::new("Berries"),
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(-126.0, -96.0, 40.0)),
		sprite: Sprite {custom_size: Some(Vec2::new(1348.0, 324.0)), ..Default::default()},
		texture: asset_server.load("sprites/mountains.png"),
		..Default::default()
		},
		Name::new("Mountains"),
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(-726.0, -208.0, 50.0)),
		sprite: Sprite {custom_size: Some(Vec2::new(148.0, 352.0)), ..Default::default()},
		texture: asset_server.load("sprites/tree.png"),
		..Default::default()
		},
		Name::new("Tree"),
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(600.0, -128.0, 50.0)),
		sprite: Sprite {custom_size: Some(Vec2::new(400.0, 336.0)), ..Default::default()},
		texture: asset_server.load("sprites/volcano.png"),
		..Default::default()
		},
		Name::new("Volcano"),
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(640.0, 142.0, 41.0)),
		sprite: Sprite {custom_size: Some(Vec2::new(320.0, 292.0)), ..Default::default()},
		texture: asset_server.load("sprites/glow.png"),
		..Default::default()
		},
		Glow,
		Name::new("Glow"),
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(600.0, -128.0, 40.0)),
		sprite: Sprite {custom_size: Some(Vec2::new(400.0, 336.0)), ..Default::default()},
		texture: asset_server.load("sprites/volcano_back.png"),
		..Default::default()
		},
		Name::new("Volcano Back"),
	));

	commands.spawn((Text2dBundle {
		transform: Transform::from_translation(Vec3::new(-532.0, 336.0, 100.0)),
		text: Text::from_section(format!("0"), get_number_text_style(&asset_server))
				.with_alignment(TextAlignment::Center),
		..Default::default()
		},
		BerryText,
		Name::new("Berry Text"),
	));

	commands.spawn((Text2dBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 344.0, 100.0)),
		text: Text::from_section(format!("Berrain"), get_title_text_style(&asset_server))
				.with_alignment(TextAlignment::Center),
		..Default::default()
		},
		Name::new("Title Text"),
	));

	commands.spawn((Text2dBundle {
		transform: Transform::from_translation(Vec3::new(576.0, 344.0, 100.0)),
		text: Text::from_section(format!("Day 1"), get_day_text_style(&asset_server))
				.with_alignment(TextAlignment::Center),
		..Default::default()
		},
		DayText,
		Name::new("Day Text"),
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 120.0, 500.0)),
		sprite: Sprite {
			custom_size: Some(Vec2::new(256.0, 256.0)),
			color: Color::DARK_GREEN,
			..Default::default()},
		texture: asset_server.load("sprites/bergod.png"),
		..Default::default()
		},
		Bergod,
		Limit {
			left: -672.0,
			right: 672.0,
		},
		Player,
		Velocity(Vec2::new(0.0, 0.0)),
		Name::new("Bergod"),
	)).with_children(|parent| {
		parent.spawn(SpriteBundle {
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
			sprite: Sprite {custom_size: Some(Vec2::new(256.0, 256.0)), ..Default::default()},
			texture: asset_server.load("sprites/bergod_crown.png"),
			..Default::default()
			}
		);
		parent.spawn(SpriteBundle {
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
			sprite: Sprite {custom_size: Some(Vec2::new(256.0, 256.0)), ..Default::default()},
			texture: asset_server.load("sprites/bergod_cloud.png"),
			..Default::default()
			}
		);
	});

	for id in 1..=STARTING_BERPLE {
		let child_bubble = commands.spawn((SpriteBundle {
			transform: Transform::from_translation(Vec3::new(0.0, 125.0, 100.0)),
			sprite: Sprite {custom_size: Some(Vec2::new(254.0, 102.0)), ..Default::default()},
			texture: asset_server.load("sprites/bubble.png"),
			visibility: Visibility::Hidden,
			..Default::default()
			},
			Speech,
			Name::new("Bubble"),
		)).id();
			
		let child_text = commands.spawn((Text2dBundle {
			transform: Transform::from_translation(Vec3::new(0.0, 135.0, 100.1)),
			text: Text::from_section(format!("Berries please?"), get_dialogue_text_style(&asset_server))
					.with_alignment(TextAlignment::Center),
			visibility: Visibility::Hidden,
			..Default::default()
			},
			Speech,
			Name::new("Bubble Text"),
		)).id();

		let berson = commands.spawn((SpriteBundle {
			transform: Transform::from_translation(Vec3::new( -600.0 + 200.0 * id as f32, -365.0 + id as f32 * 5.0, 505.0 - id as f32)),
			sprite: Sprite {
				custom_size: Some(Vec2::new(128.0, 128.0)), 
				color: get_berple_color(id),			
				..Default::default()},
			texture: asset_server.load("sprites/berson.png"),
			..Default::default()
			},
			Berson {
				id: id,
 				berries_held: 0,
			},
			BersonText{
				speech: format!("Wish I had a berry."),
				timer: Timer::from_seconds(DIALOGUE_DISPLAY_TIME, TimerMode::Once),
				visible: false,
			},
			Limit {
				left: -736.0,
				right: 736.0,
			},
			Hunger {
				hunger: 1.0 - (rand::random::<f32>() * 0.2),
				full_color: get_berple_color(id),
				starved: false,
			},
			Velocity(Vec2::new(MAX_BERSON_SPEED, 0.0)),
			Name::new("Berson"),
		)).id();

		commands.entity(berson).push_children(&[child_bubble, child_text]);
	}
}

pub fn get_berple_color(
	id: usize,
) -> Color {
	match id {
		1 => Color::RED,
		2 => Color::ORANGE_RED,
		3 => Color::BLUE,
		4 => Color::PURPLE,
		5 => Color::YELLOW,
		_ => Color::WHITE,
	}
}

pub fn _despawn_entities_with<T: Component>(
	to_despawn: Query<Entity, With<T>>, 
	mut commands: Commands
) {
	for entity in to_despawn.iter() {
		commands.entity(entity).despawn_recursive();
	}
}