use bevy::{prelude::*, sprite::Anchor};

use crate::{
	components::{Life, MaxLife, Player, PlayerInfo, ToFloat, ScoreText},
	WinSize, DEFAULT_PLAYER_LIFE, PlayerState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system_to_stage(StartupStage::PostStartup, menu_spawn_system)
        .add_system(player_life_system)
        .add_system(player_score_system)
    	;
	}
}

fn menu_spawn_system(mut commands: Commands, win_size: Res<WinSize> ,asset_server: Res<AssetServer>) {
	const LIFE_COLOR: Color = Color::rgb(0.9, 0.1, 0.1);
	const MAX_LIFE_COLOR: Color = Color::rgb(0.3, 0.3, 0.6);
	let top = win_size.h / 2.;
	let right = win_size.w / 2.;

	// Life bar
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: LIFE_COLOR,
				custom_size: Some(Vec2::new(100., 25.)),
				anchor: Anchor::TopLeft,
				..default()
			},
			transform: Transform {
				translation: Vec3::new(right - 120., top - 20., 20.0),
				..default()
			},
			..default()
		})
		.insert(PlayerInfo)
		.insert(Life::new(DEFAULT_PLAYER_LIFE));

	// Max life bar
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: MAX_LIFE_COLOR,
				custom_size: Some(Vec2::new(100., 25.)),
				anchor: Anchor::TopLeft,
				..default()
			},
			transform: Transform {
				translation: Vec3::new(right - 120., top - 20., 10.0),
				..default()
			},
			..default()
		})
		.insert(PlayerInfo)
		.insert(MaxLife::new(DEFAULT_PLAYER_LIFE));

	// Score
	commands
		.spawn_bundle(
			// Create a TextBundle that has a Text with a list of sections.
			TextBundle::from_sections([
				TextSection::new(
					"Score: ",
					TextStyle {
						font: asset_server.load("fonts/FiraSans-Bold.ttf"),
						font_size: 20.0,
						color: Color::WHITE,
                        ..Default::default()
					},
				),
				TextSection::from_style(TextStyle {
					font: asset_server.load("fonts/FiraSans-Medium.ttf"),
					font_size: 20.0,
					color: Color::GOLD,
                    ..Default::default()
				}),
			])
			.with_style(Style {
				align_self: AlignSelf::FlexEnd,
				..default()
			}),
		)
		.insert(ScoreText);
}

fn life_width<T>(t: &T) -> f32
where
	T: ToFloat,
{
	20.0 * t.to_float()
}

fn player_life_system(
	mut player_query: Query<&Life, With<Player>>,
	mut life_query: Query<&mut Sprite, (With<PlayerInfo>, With<Life>)>,
) {
	if let Ok(life) = player_query.get_single() {
		let mut life_sprite =
			life_query.get_single_mut().expect("Can't get Sprite from PlayerInfo and Life");

		let width = life_width(life);
		life_sprite.custom_size = Some(Vec2::new(width, 25.));
	}
}

fn player_score_system(player_state: Res<PlayerState>,
    mut query: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[1].value = format!("{}", player_state.score);
    }
}