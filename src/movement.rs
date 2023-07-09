use bevy::prelude::*;

use crate::{derivables::*, setup::{OrthoSize, get_berple_color}};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems((
				apply_velocity,
				lava_fade,
				apply_hunger,
				sky_color,
				update_speech,
				grow_berries,
				bush_collision,
				check_end,
				move_sun,
				speed_up_game,
				move_player,
				ai_bergod,
				role_reverse,
			))
		;
	}
}

pub fn apply_velocity(
	mut velocity_query: Query<(&mut Transform, &mut Velocity, &Limit)>,
	time: Res<Time>,
	speed: Res<SpeedUpFactor>,
) {
	for (mut transform, mut velocity, limit) in velocity_query.iter_mut() {
		let target_x = transform.translation.x + velocity.0.x * time.delta_seconds() * speed.0;
		if target_x < limit.left || target_x > limit.right {
			velocity.0.x = -velocity.0.x;
		}
		transform.translation.x += velocity.0.x * time.delta_seconds() * speed.0;
		transform.translation.y += velocity.0.y * time.delta_seconds() * speed.0;
	}
}

fn move_player(
	mut player_query: Query<(&mut Velocity, With<Player>)>,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
	speed: Res<SpeedUpFactor>,
) {
	for (mut velocity, _) in player_query.iter_mut() {
		if keyboard.pressed(KeyCode::A) {
			velocity.0.x = (velocity.0.x - BERGOD_ACCELERATION * time.delta_seconds() * speed.0).clamp(-MAX_BERGOD_SPEED, MAX_BERGOD_SPEED);
		}
		if keyboard.pressed(KeyCode::D) {
			velocity.0.x = (velocity.0.x + BERGOD_ACCELERATION * time.delta_seconds() * speed.0).clamp(-MAX_BERGOD_SPEED, MAX_BERGOD_SPEED);
		}
	}
}

fn bush_collision(
	mut commands: Commands,
	mut berson_query: Query<(Entity, &Transform, &mut Berson)>,
	mut event: EventWriter<UpdateSpeech>,
	bush_query: Query<(Entity, &Transform, &Bush, Without<Berson>)>,
	asset_server: Res<AssetServer>,
) {
	for (bush_entity, bush_transform, bush, _) in bush_query.iter() {
		for (berson_entity, berson_transform, mut berson) in berson_query.iter_mut() {
			if bush.0.finished() {
				let mut grabbed_berry = false;
				if (berson_transform.translation.x - bush_transform.translation.x).abs() < 64.0 {
					if berson.berries_held == 0 {
						commands.entity(bush_entity).despawn_recursive();
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
					} else if berson.berries_held == 1 {
						commands.entity(bush_entity).despawn_recursive();
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
					}
				}
				if grabbed_berry {
					berson.berries_held += 1;
					event.send(UpdateSpeech{berson: berson_entity, mood: BersonMood::Wavering});
					break;
				}
			}
		}
	}
}

fn ai_bergod (
	mut ai_bergod_query: Query<(&mut Velocity, (With<Bergod>, Without<Player>))>,
) {
	for (mut velocity, _) in ai_bergod_query.iter_mut() {
		velocity.0.x = velocity.0.x.signum() * MAX_BERGOD_SPEED;
	}
}

fn lava_fade(
	mut commands: Commands,
	mut lava_query: Query<(Entity, &Transform, &mut Sprite, With<Lava>)>,
) {
	for (entity, transform, mut sprite, _) in lava_query.iter_mut() {
		sprite.color.set_a((-transform.translation.y + 230.0)/300.0);
		if sprite.color.a() <= 0.0 {
			commands.entity(entity).despawn_recursive();
		}
	}
}

fn apply_hunger(
	end_screen_query: Query<(&Sprite, With<EndScreen>)>,
	mut berson_query: Query<(Entity, &mut Sprite, &mut Hunger, &mut Velocity, (Without<EndScreen>, Without<Player>))>,
	mut player_berson_query: Query<(Entity, &mut Sprite, &mut Hunger, &mut Velocity, (Without<EndScreen>, With<Player>))>,
	mut event: EventWriter<UpdateSpeech>,
	time: Res<Time>,
	speed: Res<SpeedUpFactor>,
	berries_sacrificed: Res<BerriesSacrificed>,
) {
	let mut pause_hunger = false;
	for (end_sprite, _) in end_screen_query.iter() {
		if end_sprite.color.a() > 0.0 {
			pause_hunger = true;
		}
	}
	if !pause_hunger {
		for (entity, mut sprite, mut hunger, mut velocity, _) in berson_query.iter_mut() {
			hunger.hunger = (hunger.hunger - HUNGER_RATE*time.delta_seconds() * speed.0).clamp(0.0, 1.0);
			let mut speech = false;
			if rand::random::<f32>() < SPEECH_RATE {
				speech = true;
			}
			if hunger.hunger > HUNGRY {
				velocity.0.x = velocity.0.x.signum() * (MAX_BERSON_SPEED + berries_sacrificed.0 as f32 * SACRIFICE_SPEED_MULTIPLIER);
				if rand::random::<f32>() < TURN_RATE {
					velocity.0.x = -velocity.0.x;
				}
				if speech {
					event.send(UpdateSpeech{berson: entity, mood: BersonMood::Appeased});
				}
			} else if hunger.hunger > STARVING {
				velocity.0.x = velocity.0.x.signum() * (MAX_BERSON_SPEED + berries_sacrificed.0 as f32 * SACRIFICE_SPEED_MULTIPLIER) * (2.0 - hunger.hunger);
				if speech {
					event.send(UpdateSpeech{berson: entity, mood: BersonMood::Hungry});
				}
			} else if hunger.hunger > 0.0 {
				velocity.0.x = velocity.0.x.signum() * (MAX_BERSON_SPEED + berries_sacrificed.0 as f32 * SACRIFICE_SPEED_MULTIPLIER) * 2.0;
				if speech {
					event.send(UpdateSpeech{berson: entity, mood: BersonMood::Starving});
				}
			} else {
				velocity.0.x = 0.0;
			}
			
			sprite.color = hunger.full_color + (Color::WHITE * 2.0 * (1.0 - hunger.hunger));
			
			if hunger.hunger == 0.0 && !hunger.starved {
				event.send(UpdateSpeech{berson: entity, mood: BersonMood::Starved});
				hunger.starved = true;
			}
		}
		for (entity, mut sprite, mut hunger, mut velocity, _) in player_berson_query.iter_mut() {
			hunger.hunger = (hunger.hunger - HUNGER_RATE*time.delta_seconds() * speed.0).clamp(0.0, 1.0);

			let mut speech = false;
			if rand::random::<f32>() < 0.001 {
				speech = true;
			}
			if hunger.hunger > HUNGRY {
				if speech {
					event.send(UpdateSpeech{berson: entity, mood: BersonMood::Appeased});
				}
			} else if hunger.hunger > STARVING {
				if speech {
					event.send(UpdateSpeech{berson: entity, mood: BersonMood::Hungry});
				}
			} else if hunger.hunger > 0.0 {
				if speech {
					event.send(UpdateSpeech{berson: entity, mood: BersonMood::Starving});
				}
			} else {
				velocity.0.x = 0.0;
			}
			
			sprite.color = hunger.full_color + (Color::WHITE * 2.0 * (1.0 - hunger.hunger));
			
			if hunger.hunger == 0.0 && !hunger.starved {
				event.send(UpdateSpeech{berson: entity, mood: BersonMood::Starved});
				hunger.starved = true;
			}
		}
	}
}

fn sky_color(
	mut sky_query: Query<(&mut Sprite, &mut Sky)>,
	mut stars_query: Query<(&mut Sprite, (With<Stars>, Without<Sky>))>,
	mut glow_query: Query<(&mut Sprite, (With<Glow>, Without<Stars>, Without<Sky>))>,
	mut current_day: ResMut<CurrentDay>,
	time: Res<Time>,
	speed: Res<SpeedUpFactor>,
) {
	let (mut sprite, mut sky) = sky_query.single_mut();
	let (mut star_sprite, _) = stars_query.single_mut();
	let (mut glow_sprite, _) = glow_query.single_mut();
	sprite.color = Color::WHITE * sky.time;
	star_sprite.color.set_a((0.4 - (sky.time / 2.5)).clamp(0.0, 1.0));
	glow_sprite.color.set_a((0.2 - (sky.time / 3.0) + (sky.time*50.0).sin()/40.0).clamp(0.0, 1.0));
	if sky.rising {
		sky.time = (sky.time - DAY_RATE*time.delta_seconds() * speed.0).clamp(0.0, 1.0);
	} else {
		sky.time = (sky.time + DAY_RATE*time.delta_seconds() * speed.0).clamp(0.0, 1.0);
	}
	if sky.time == 0.0 || sky.time == 1.0 {
		sky.rising = !sky.rising;
	}
	if sky.time == 1.0 {
		current_day.0 += 1;
	}
}

fn move_sun(
	sky_query: Query<&Sky>,
	mut sun_query: Query<(&mut Sprite, &mut Transform, (With<Sun>, Without<Sky>))>,
) {
	let sky = sky_query.single();
	let (mut sprite, mut transform, _) = sun_query.single_mut();
	let poles = 1200.0;
	let zenith = 500.0;
	if sky.rising {
		transform.translation.x = poles - poles * sky.time;
		transform.translation.y = zenith * (sky.time * 3.14/2.0).sin() - 320.0;
		sprite.color.set_a(1.0 - (0.8 - (sky.time / 1.5)).clamp(0.0, 1.0));
	} else {
		transform.translation.x =  -poles + poles * sky.time;
		transform.translation.y = zenith * (sky.time * 3.14/2.0).sin() - 320.0;
		sprite.color.set_a(1.0 - (0.8 - (sky.time / 1.5)).clamp(0.0, 1.0));
	}
}

fn grow_berries(
	mut bush_query: Query<(&mut Handle<Image>, &mut Bush)>,
	asset_server: Res<AssetServer>,
	time: Res<Time>,
	speed: Res<SpeedUpFactor>,
) {
	for (mut image, mut bush) in bush_query.iter_mut() {
		bush.0.tick(time.delta() * speed.0 as u32);
		if bush.0.just_finished() {
			*image = asset_server.load("sprites/berry_bush.png");
		}
	}
}

fn check_end(
	remaining_berple: Res<RemainingBerple>,
	current_day: Res<CurrentDay>,
	mut game_over: ResMut<GameOver>,
	mut commands: Commands,
	mut end_screen_query: Query<(Entity, &mut Sprite, With<EndScreen>)>,
	mut skull_query: Query<(&mut Sprite, &Skull, Without<EndScreen>)>,
	mut bounty_query: Query<(&mut Sprite, &Bounty, (Without<EndScreen>, Without<Skull>))>,
	keyboard: Res<Input<KeyCode>>,
	ortho_size: Res<OrthoSize>,
	asset_server: Res<AssetServer>,
) {
	if remaining_berple.0 == 0 {
		game_over.0 = 1;
	};
	for (end_screen_entity, mut sprite, _) in end_screen_query.iter_mut() {
		if sprite.color.a() == 1.0 {
			if keyboard.just_pressed(KeyCode::Space) {
				commands.spawn((SpriteBundle {
					transform: Transform::from_translation(Vec3::new(0.0, 0.0, 920.0)),
					sprite: Sprite {
						custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)),
						color: Color::hex("37946E").unwrap(), 
						..Default::default()},
					..Default::default()
					},
					Name::new("Death Day Screen"),
				));
				commands.spawn((Text2dBundle {
					transform: Transform::from_translation(Vec3::new(0.0, 0.0, 930.0)),
					text: Text::from_section(format!("You survived {} day{}!\nThanks for playing!", current_day.0, if current_day.0 > 1 {"s"} else {""}), get_title_text_style(&asset_server))
							.with_alignment(TextAlignment::Center),
					..Default::default()
					},
					Name::new("Death Day Text"),
				));
				println!("Reset Game");
			}
		}
		if game_over.0 == 1 || current_day.0 > 9 {
			sprite.color = Color::rgba(sprite.color.r(), sprite.color.g(), sprite.color.b(), (sprite.color.a() + 0.005).clamp(0.0, 1.0));
			for (mut sprite, skull, _) in skull_query.iter_mut() {
				if skull.0 > remaining_berple.0 && game_over.0 != 3 {
					sprite.color = Color::rgba(sprite.color.r(), sprite.color.g(), sprite.color.b(), (sprite.color.a() + 0.005).clamp(0.0, 1.0));
				}
			};
			for (mut sprite, bounty, _) in bounty_query.iter_mut() {
				if bounty.0 <= remaining_berple.0 && game_over.0 != 3 {
					sprite.color = Color::rgba(sprite.color.r(), sprite.color.g(), sprite.color.b(), (sprite.color.a() + 0.005).clamp(0.0, 1.0));
				}
			};
		} else if game_over.0 == 2 {
			commands.entity(end_screen_entity).despawn_recursive();
			commands.spawn((SpriteBundle {
				transform: Transform::from_translation(Vec3::new(0.0, 0.0, 900.0)),
				sprite: Sprite {
					custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)),
					color: Color::rgba(1.0, 1.0, 1.0, 0.0), 
					..Default::default()},
				texture: asset_server.load("sprites/end_screen_starved.png"),
				..Default::default()
				},
				EndScreen,
				Name::new("Death Screen"),
			));
			game_over.0 = 3;
		} else if game_over.0 == 3 {
			sprite.color = Color::rgba(sprite.color.r(), sprite.color.g(), sprite.color.b(), (sprite.color.a() + 0.005).clamp(0.0, 1.0));
		}
	}
}

fn role_reverse(
	sky_query: Query<&Sky>,
	mut commands: Commands,
	mut player_query: Query<(Entity, &mut Sprite, With<Player>)>,
	mut berson_query: Query<(Entity, &mut Hunger, &Berson)>,
	mut bergod_query: Query<(Entity, &mut Sprite, &mut Velocity, (With<Bergod>, Without<Berson>, Without<Player>))>,
	mut berries_held: ResMut<BerriesHeld>,
	berries_gifted: Res<BerriesGifted>,
	current_day: Res<CurrentDay>,
) {
	let sky = sky_query.single();
	if sky.time == 0.0 {
		let (player_entity, mut player_sprite, _) = player_query.single_mut();
		if bergod_query.is_empty() {
			let mut new_player = Entity::PLACEHOLDER;
			//let check = rand::thread_rng().gen_range(1..=5);
			for (berson_entity, mut hunger, berson) in berson_query.iter_mut() {
				//if berson.id == check {
					new_player = berson_entity;
					hunger.full_color = Color::DARK_GREEN;
					player_sprite.color = get_berple_color(berson.id);
					/*berries_held.0 = match berson.id {
						1 => berries_gifted.one,
						2 => berries_gifted.two,
						3 => berries_gifted.three,
						4 => berries_gifted.four,
						5 => berries_gifted.five,
						_ => 0,
					};*/
					break;
				//}
			}
			if new_player != Entity::PLACEHOLDER {
				commands.entity(new_player).insert(Player);
				commands.entity(player_entity).remove::<Player>();
			} else {
				println!("You have swapped places with the dead.");
			}
		} else {
			for (_, mut hunger, berson) in berson_query.iter_mut() {
				hunger.full_color = get_berple_color(berson.id);
			}
			let (entity, mut sprite, mut velocity, _) = bergod_query.single_mut();
			sprite.color = Color::DARK_GREEN;
			commands.entity(entity).insert(Player);
			commands.entity(player_entity).remove::<Player>();
			berries_held.0 = 5;
			velocity.0.x = 0.0;
		}
	}		
}

fn speed_up_game(
	mut speed: ResMut<SpeedUpFactor>,
	mouse: Res<Input<MouseButton>>,
	keyboard: Res<Input<KeyCode>>
) {
	if speed.0 != 0.0 {
		if mouse.pressed(MouseButton::Right) || keyboard.pressed(KeyCode::F) {
			speed.0 = GAME_SPEED_MULTIPLIER;
		} else {
			speed.0 = 1.0;
		}
	}
	if keyboard.just_pressed(KeyCode::P) {
		if speed.0 == 0.0 {
			speed.0 = 1.0;
		} else {
			speed.0 = 0.0;
		}
	}
}

fn update_speech(
	mut events: EventReader<UpdateSpeech>,
	mut text_query: Query<(&Berson, &mut BersonText)>,
	current_day: Res<CurrentDay>,
) {
	for event in events.iter() {
		if let Ok((berson, mut text)) = text_query.get_mut(event.berson) {
			text.speech = get_berson_speech(&event.mood, berson.id, current_day.0);
			text.visible = true;
			text.timer.reset();
		}
	}
}

fn get_berson_speech(
	mood: &BersonMood,
	id: usize,
	day: usize,
) -> String {
	match day {
		_ => {
			match id {
				1 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("A gift from above!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Yes! Burn them!"),
						BersonMood::Starving => format!("Am I not worthy?"),
						BersonMood::Confused => format!("They demand sacrifice!"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("I fast for you."),
						BersonMood::Appeased => format!("I want for nothing!"),
					}
				}
				2 => {
					match mood {
						BersonMood::Eating => format!("Nourishment!"),
						BersonMood::Wavering => format!("A berry? From below?"),
						BersonMood::Thankful => format!("Food! Finally."),
						BersonMood::Flattered => format!("I am worth double!"),
						BersonMood::Starved => format!("A life wasted..."),
						BersonMood::Outraged => format!("Berries... Wasted."),
						BersonMood::Starving => format!("I require food!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A bush for me!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("That'll do for now."),
					}
				}
				3 => {
					match mood {
						BersonMood::Eating => format!("Tastes gross."),
						BersonMood::Wavering => format!("Berry in the leaves."),
						BersonMood::Thankful => format!("What is this?"),
						BersonMood::Flattered => format!("Oh great, more."),
						BersonMood::Starved => format!("I guess this is it."),
						BersonMood::Outraged => format!("Sure, destroy them."),
						BersonMood::Starving => format!("Okay give me one!"),
						BersonMood::Confused => format!("Volcano vs berry."),
						BersonMood::Surprised => format!("Berry in the dirt"),
						BersonMood::Hungry => format!("Got anything better?"),
						BersonMood::Appeased => format!("This is the life!"),
					}
				}
				4 => {
					match mood {
						BersonMood::Eating => format!("Yum!"),
						BersonMood::Wavering => format!("Berry bush!"),
						BersonMood::Thankful => format!("A berry!"),
						BersonMood::Flattered => format!("Two berries!"),
						BersonMood::Starved => format!("I had a good life."),
						BersonMood::Outraged => format!("Cooked berry!"),
						BersonMood::Starving => format!("Berry please?"),
						BersonMood::Confused => format!("Cooked berry?"),
						BersonMood::Surprised => format!("A bush!"),
						BersonMood::Hungry => format!("Give berry?"),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				5 => {
					match mood {
						BersonMood::Eating => format!("TASTY!"),
						BersonMood::Wavering => format!("WHAT IS THIS?"),
						BersonMood::Thankful => format!("THANK YOU!"),
						BersonMood::Flattered => format!("ANOTHER ONE?!"),
						BersonMood::Starved => format!("MY TIME IS UP!"),
						BersonMood::Outraged => format!("VOLCANO BURN!"),
						BersonMood::Starving => format!("I STARVE!"),
						BersonMood::Confused => format!("TOASTED BERRY!"),
						BersonMood::Surprised => format!("BERRY GROWS!"),
						BersonMood::Hungry => format!("I GROW HUNGRY!"),
						BersonMood::Appeased => format!("I AM CONTENT!"),
					}
				}
				_ => format!("Wish I had a berry."),
			}
		},
		2 => {
			match id {
				1 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				2 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				3 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				4 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				5 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				_ => format!("Wish I had a berry."),
			}
		},
		3 => {
			match id {
				1 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				2 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				3 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				4 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				5 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				_ => format!("Wish I had a berry."),
			}
		},
		4 => {
			match id {
				1 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				2 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				3 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				4 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				5 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				_ => format!("Wish I had a berry."),
			}
		},
		_ => {
			match id {
				1 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				2 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				3 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				4 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				5 => {
					match mood {
						BersonMood::Eating => format!("Mmm, tasty berry!"),
						BersonMood::Wavering => format!("This bush... a god?"),
						BersonMood::Thankful => format!("Thanks for the berry!"),
						BersonMood::Flattered => format!("You are too generous!"),
						BersonMood::Starved => format!("I become fertiliser..."),
						BersonMood::Outraged => format!("Stop that! Now!"),
						BersonMood::Starving => format!("Please, I beg you!"),
						BersonMood::Confused => format!("Why do you taunt us?"),
						BersonMood::Surprised => format!("Oh? A berry bush!"),
						BersonMood::Hungry => format!("Hey, berries, now."),
						BersonMood::Appeased => format!("Ah, nap time."),
					}
				}
				_ => format!("Wish I had a berry."),
			}
		},
	}
}