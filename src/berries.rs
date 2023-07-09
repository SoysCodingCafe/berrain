use bevy::{prelude::*, math::Vec3Swizzles};
use rand::Rng;

use crate::{derivables::*, movement::apply_velocity};

pub struct BerriesPlugin;

impl Plugin for BerriesPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_startup_systems((

			))
			.add_systems((
				berry_track_bergod.after(apply_velocity),
				drop_berry,
				berry_respawn_countdown,
				berry_collision,
				plant_berry,
				fix_berry_z,
				pray_for_berries,
				eat_berry,
			))
		;
	}
}

fn berry_track_bergod(
	bergod_query: Query<(&Transform, With<Bergod>)>,
	mut berry_query: Query<(&mut Transform, (With<Berry>, Without<Bergod>, Without<Velocity>))>,
) {
	let (bergod_transform, _) = bergod_query.single();
	for (mut berry_transform, _) in berry_query.iter_mut() {
		berry_transform.translation.x = bergod_transform.translation.x + 64.0;
		berry_transform.translation.y = bergod_transform.translation.y + 55.0;
	}
}

fn pray_for_berries(
	mut berries_held: ResMut<BerriesHeld>,
	mut berson_query: Query<(&Transform, &Velocity, &Limit, With<Berson>)>,
	time: Res<Time>,
	speed: Res<SpeedUpFactor>,
) {
	for (transform, velocity, limit, _) in berson_query.iter_mut() {
		let target_x = transform.translation.x + velocity.0.x * time.delta_seconds() * speed.0;
		if target_x < limit.left {
			berries_held.0 = (berries_held.0 + 1).clamp(0, 9);
		}
	}
}

fn drop_berry(
	keyboard: Res<Input<KeyCode>>,
	mouse: Res<Input<MouseButton>>,
	berry_query: Query<(Entity, (With<Berry>, Without<Velocity>))>,
	bergod_player_query: Query<(With<Bergod>, With<Player>)>,
	mut commands: Commands,
	mut berry_respawn: ResMut<BerryRespawn>,
	mut berries_held: ResMut<BerriesHeld>,
	time: Res<Time>,
	speed: Res<SpeedUpFactor>,
) {
	if keyboard.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left){
		if !bergod_player_query.is_empty() {
			for (entity, _) in berry_query.iter() {
				if berry_respawn.0.finished() {
					commands
						.entity(entity)
						.insert(Velocity(Vec2::new(0.0, BERRY_SPEED)));
					berries_held.0 = (berries_held.0 - 1).clamp(0, 9);
					berry_respawn.0.reset();
				}
			}
		}
	}
	if bergod_player_query.is_empty() {
		for (entity, _) in berry_query.iter() {
			if berry_respawn.0.finished() {
				commands
					.entity(entity)
					.insert(Velocity(Vec2::new(0.0, BERRY_SPEED)));
				berries_held.0 = (berries_held.0 - 1).clamp(0, 9);
				berry_respawn.0.reset();
			}
		}
	}
	if berries_held.0 > 0 {
		berry_respawn.0.tick(time.delta() * speed.0 as u32);
	}
}

fn berry_respawn_countdown(
	mut commands: Commands,
	berry_respawn: ResMut<BerryRespawn>,
	asset_server: Res<AssetServer>,
	berry_query: Query<(With<Berry>, Without<Velocity>)>,
	bergod_query: Query<(&Transform, With<Bergod>)>,
) {
	if berry_query.is_empty() {
		if berry_respawn.0.just_finished() {
			let (transform, _) = bergod_query.single();
			commands.spawn((SpriteBundle {
				transform: Transform::from_translation(Vec3::new(transform.translation.x + 64.0, transform.translation.y + 55.0, 400.0)),
				sprite: Sprite {custom_size: Some(Vec2::new(52.0, 64.0)), ..Default::default()},
				texture: asset_server.load("sprites/berry.png"),
				..Default::default()
				},
				Berry,
				Limit {
					left: -774.0,
					right: 774.0,
				},
				Name::new("Berry"),
			));
		}
	}
}

fn berry_collision(
	mut commands: Commands,
	mut berson_query: Query<(Entity, &mut Berson, &Transform, Without<Berry>)>,
	mut event: EventWriter<UpdateSpeech>,
	mut berries_gifted: ResMut<BerriesGifted>,
	berry_query: Query<(Entity, &Transform, (With<Berry>, With<Velocity>))>,
	asset_server: Res<AssetServer>,
) {
	for (berry_entity, berry_transform, _) in berry_query.iter() {
		for (berson_entity, mut berson, berson_transform, _) in berson_query.iter_mut() {
			let mut grabbed_berry = false;
			if (berry_transform.translation.xy() - berson_transform.translation.xy()).length() < 64.0 {
				match berson.id {
					1 => berries_gifted.one += 1,
					2 => berries_gifted.two += 1,
					3 => berries_gifted.three += 1,
					4 => berries_gifted.four += 1,
					5 => berries_gifted.five += 1,
					_ => println!("Unknown berson detected!"),
				}
				if berson.berries_held == 0 {
					commands.entity(berry_entity).despawn_recursive();
					commands.entity(berson_entity).with_children(|parent| {
						parent.spawn((SpriteBundle {
							transform: Transform::from_translation(Vec3::new(-40.0, 40.0, -0.1)),
							sprite: Sprite {custom_size: Some(Vec2::new(52.0, 64.0)), ..Default::default()},
							texture: asset_server.load("sprites/berry.png"),
							..Default::default()
							},
							HeldBerry(1),
							Name::new("Berson Berry"),
						));
					});
					grabbed_berry = true;
					event.send(UpdateSpeech{berson: berson_entity, mood: BersonMood::Thankful});
				} else if berson.berries_held == 1 {
					commands.entity(berry_entity).despawn_recursive();
					commands.entity(berson_entity).with_children(|parent| {
						parent.spawn((SpriteBundle {
							transform: Transform::from_translation(Vec3::new(35.0, 23.0, -0.1)),
							sprite: Sprite {custom_size: Some(Vec2::new(52.0, 64.0)), ..Default::default()},
							texture: asset_server.load("sprites/berry.png"),
							..Default::default()
							},
							HeldBerry(2),
							Name::new("Berson Berry"),
						));
					});
					grabbed_berry = true;
					event.send(UpdateSpeech{berson: berson_entity, mood: BersonMood::Flattered});
				}
			}
			if grabbed_berry {
				berson.berries_held += 1;
				break;
			}
		}
	}
}

fn plant_berry(
	mut berries_sacrificed: ResMut<BerriesSacrificed>,
	mut commands: Commands,
	berry_query: Query<(Entity, &Transform, (With<Berry>, With<Velocity>))>,
	asset_server: Res<AssetServer>,
	berson_query: Query<(Entity, &Berson)>,
	mut event: EventWriter<UpdateSpeech>,
) {
	let mut event_entity = Entity::PLACEHOLDER;
	let check = rand::thread_rng().gen_range(1..=5);
	for (berson_entity, berson) in berson_query.iter() {
		if berson.id == check {
			event_entity = berson_entity;
			break;
		}
	}
	for (entity, transform, _) in berry_query.iter() {
		if transform.translation.x < 624.0 {
			if transform.translation.y <= -370.0 {
				commands.entity(entity).despawn_recursive();
				commands.spawn((SpriteBundle {
					transform: Transform::from_translation(Vec3::new(transform.translation.x, transform.translation.y, 600.0))
					.with_scale(Vec3::new(if rand::random::<f32>() > 0.5 {-1.0} else {1.0}, 1.0, 1.0)),
					sprite: Sprite {custom_size: Some(Vec2::new(155.0, 130.0)), ..Default::default()},
					texture: asset_server.load("sprites/bush.png"),
					..Default::default()
					},
					Bush(Timer::from_seconds(10.0, TimerMode::Once)),
					Name::new("Bush"),
				));
				event.send(UpdateSpeech{berson: event_entity, mood: BersonMood::Surprised});
			}
		} else {
			if transform.translation.y <= -200.0 {
				commands.entity(entity).despawn_recursive();
				commands.spawn((SpriteBundle {
					transform: Transform::from_translation(Vec3::new(706.0, -70.0, 50.0)),
					sprite: Sprite {custom_size: Some(Vec2::new(144.0, 151.0)), ..Default::default()},
					texture: asset_server.load("sprites/lava.png"),
					..Default::default()
					},
					Velocity(Vec2::new(0.0, 100.0)),
					Limit{
						left: -800.0,
						right: 800.0,
					},
					Lava,
					Name::new("Lava"),
				));
				berries_sacrificed.0 += 1;
				if berries_sacrificed.0 < 3 {
					event.send(UpdateSpeech{berson: event_entity, mood: BersonMood::Confused});
				} else {
					event.send(UpdateSpeech{berson: event_entity, mood: BersonMood::Outraged});
				}
			}
		}
	}
}

fn fix_berry_z(
	mut berry_query: Query<(&mut Transform, (With<Berry>, With<Velocity>))>,
) {
	for (mut transform, _) in berry_query.iter_mut() {
		if transform.translation.x > 624.0 {
			transform.translation.z = 45.0;
		} else {
			transform.translation.z = 400.0;
		}
	}
}

fn eat_berry(
	mut commands: Commands,
	mut berson_query: Query<(&mut Berson, &mut Hunger, &Children)>,
	mut held_query: Query<(Entity, &HeldBerry)>,
	mut event: EventWriter<UpdateSpeech>,
) {
	for (mut berson, mut hunger, children) in berson_query.iter_mut() {
		if hunger.hunger < HUNGER_EAT_THRESHOLD {
			if berson.berries_held > 0 {
				for child in children.iter() {
					if let Ok((entity, held_berry)) = held_query.get_mut(*child) {
						if berson.berries_held == 2 {
							if held_berry.0 == 2 {
								commands.entity(entity).despawn_recursive();
								berson.berries_held -= 1;
								hunger.hunger = (hunger.hunger + HUNGER_RESTORED_BERRY).clamp(0.0, 1.0);
								hunger.starved = false;
								event.send(UpdateSpeech{berson: entity, mood: BersonMood::Eating});
								break;
							}
						} else if berson.berries_held == 1 {
							if held_berry.0 == 1 {
								commands.entity(entity).despawn_recursive();
								berson.berries_held -= 1;
								hunger.hunger = (hunger.hunger + HUNGER_RESTORED_BERRY).clamp(0.0, 1.0);
								hunger.starved = false;
								event.send(UpdateSpeech{berson: entity, mood: BersonMood::Eating});
								break;
							} 
						}
					}
				}
			}
		}
	}
}