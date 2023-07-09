use bevy::prelude::*;

use crate::derivables::*;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems((
				update_text,
				berson_dialogue,
			))
		;
	}
}

fn update_text(
	berries_held: Res<BerriesHeld>,
	current_day: Res<CurrentDay>,
	mut berry_text_query: Query<(&mut Text, &mut Transform, With<BerryText>)>,
	mut day_text_query: Query<&mut Text, (With<DayText>, Without<BerryText>)>,
) {
	let (mut berry_text, mut transform, _) = berry_text_query.single_mut();
	let mut day_text = day_text_query.single_mut();

	//if berries_held.0 > 8 {
	//	transform.rotation = Quat::from_rotation_z(-3.14/2.0);
	//	berry_text.sections[0].value = format!("8");
	//} else {
	//	transform.rotation.z = 0.0;
		berry_text.sections[0].value = format!("{}", berries_held.0);
	//}
	if current_day.0 <= 9 {
		day_text.sections[0].value = format!("{} days left", 10 - current_day.0);
	} else {
		day_text.sections[0].value = format!("You Win!");
	}
}

fn berson_dialogue(
	mut commands: Commands,
	mut berson_query: Query<(Entity, &Transform, &mut BersonText, &Children, &Hunger)>,
	mut visibility_query: Query<&mut Visibility, With<Speech>>,
	mut text_query: Query<&mut Text>,
	mut remaining_berple: ResMut<RemainingBerple>,
	mut game_over: ResMut<GameOver>,
	end_screen_query: Query<(&Sprite, With<EndScreen>)>,
	asset_server: Res<AssetServer>,
	time: Res<Time>,
	speed: Res<SpeedUpFactor>,
) {
	if speed.0 != 0.0 {
		for (entity, transform, mut berson_text, children, hunger) in berson_query.iter_mut() {
			for child in children.iter() {
				if berson_text.visible {
					if let Ok(mut visibility) = visibility_query.get_mut(*child) {
						*visibility = Visibility::Visible;
					}
					if let Ok(mut text) = text_query.get_mut(*child) {
						text.sections[0].value = format!("{}", berson_text.speech);
					}
				} else {
					if let Ok(mut visibility) = visibility_query.get_mut(*child) {
						*visibility = Visibility::Hidden;
					}
				}
			}
			berson_text.timer.tick(time.delta() * speed.0 as u32);
			if berson_text.timer.just_finished() {
				let mut despawn = true;
				for (sprite, _) in end_screen_query.iter() {
					if sprite.color.a() > 0.0 {
						despawn = false;
					}
				}
				if despawn {
					berson_text.visible = false;
					if hunger.starved {
						if hunger.full_color == Color::DARK_GREEN {
							game_over.0 = 2;
						} else {
							commands.entity(entity).despawn_recursive();
							remaining_berple.0 -= 1;
							let loc = transform.translation;
							commands.spawn((SpriteBundle {
								transform: Transform::from_xyz(loc.x, loc.y - 20.0, loc.z),
								sprite: Sprite {custom_size: Some(Vec2::new(168.0, 84.0)), ..Default::default()},
								texture: asset_server.load("sprites/fallen_berson.png"),
								..Default::default()
								},
								Name::new("Fallen Berson"),
							));
						}
					}
				}
			}
		}
	}
}