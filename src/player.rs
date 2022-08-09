use std::f32::consts::FRAC_1_SQRT_2;

use crate::components::{
	FromPlayer, Laser, Life, MaxLife, Movable, Player, PlayerInfo, SpriteSize, ToFloat,
	Velocity,
};
use crate::{
	GameTextures, PlayerState, WinSize, DEFAULT_PLAYER_LIFE, PLAYER_LASER_SIZE,
	PLAYER_RESPAWN_DELAY, PLAYER_SIZE, SPRITE_SCALE,
};
use bevy::sprite::Anchor;
use bevy::time::FixedTimestep;
use bevy::{prelude::*, transform};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(PlayerState::default())
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(0.5))
					.with_system(player_spawn_system),
			)
			.add_system(player_keyboard_event_system)
			.add_system(player_fire_system)
			// .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(0.2)).with_system(player_debug_system))
			;
	}
}


fn player_spawn_system(
	mut commands: Commands,
	mut player_state: ResMut<PlayerState>,
	time: Res<Time>,
	game_textures: Res<GameTextures>,
	win_size: Res<WinSize>,
) {
	let now = time.seconds_since_startup();
	let last_shot = player_state.last_shot;

	// The player should spawn if it's not 'alive' and after a `PLAYER_RESPAWN_DELAY` delay
	let should_respawn = !player_state.alive && match last_shot {
		None => true,
		Some(time) => now > time + PLAYER_RESPAWN_DELAY
	};

	if should_respawn {
		// add player
		let bottom = -win_size.h / 2.;
		commands
			.spawn_bundle(SpriteBundle {
				texture: game_textures.player.clone(),
				transform: Transform {
					translation: Vec3::new(
						0.,
						bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.,
						10.,
					),
					scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Player)
			.insert(SpriteSize::from(PLAYER_SIZE))
			.insert(Movable { auto_despawn: false })
			.insert(Velocity { x: 0., y: 0. })
			.insert(Life::new(DEFAULT_PLAYER_LIFE))
			.insert(MaxLife::new(DEFAULT_PLAYER_LIFE));

		player_state.spawned();
	}
}

fn player_fire_system(
	mut commands: Commands,
	kb: Res<Input<KeyCode>>,
	game_textures: Res<GameTextures>,
	query: Query<&Transform, With<Player>>,
) {
	if let Ok(player_tf) = query.get_single() {
		if kb.just_pressed(KeyCode::Space) {
			let (x, y) = (player_tf.translation.x, player_tf.translation.y);
			let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;

			let mut spawn_laser = |x_offset: f32| {
				commands
					.spawn_bundle(SpriteBundle {
						texture: game_textures.player_laser.clone(),
						transform: Transform {
							translation: Vec3::new(x + x_offset, y + 15., 0.),
							scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
							..Default::default()
						},
						..Default::default()
					})
					.insert(Laser)
					.insert(FromPlayer)
					.insert(SpriteSize::from(PLAYER_LASER_SIZE))
					.insert(Movable { auto_despawn: true })
					.insert(Velocity { x: 0., y: 1. });
			};

			spawn_laser(x_offset);
			spawn_laser(-x_offset);
		}
	}
}

fn player_keyboard_event_system(
	kb: Res<Input<KeyCode>>,
	mut query: Query<&mut Velocity, With<Player>>,
) {
	if let Ok(mut velocity) = query.get_single_mut() {
		let left = kb.pressed(KeyCode::Left);
		let right = kb.pressed(KeyCode::Right);
		let up = kb.pressed(KeyCode::Up);
		let down = kb.pressed(KeyCode::Down);

		let (vx, vy): (f32, f32) = match (left, right, up, down) {
			(true, false, false, false) => (-1., 0.), // LEFT
			(false, true, false, false) => (1., 0.),  // RIGHT
			(false, false, true, false) => (0., 1.),  // UP
			(false, false, false, true) => (0., -1.), // DOWN

			(true, false, true, false) => (-FRAC_1_SQRT_2, FRAC_1_SQRT_2), // LEFT UP
			(true, false, false, true) => (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2), // LEFT DOWN
			(false, true, true, false) => (FRAC_1_SQRT_2, FRAC_1_SQRT_2),  // RIGHT UP
			(false, true, false, true) => (FRAC_1_SQRT_2, -FRAC_1_SQRT_2), // RIGHT DOWN

			_ => (0., 0.),
		};

		velocity.x = vx;
		velocity.y = vy;
	}
}

fn player_debug_system(mut player_query: Query<&Life, With<Player>>) {
	if let Ok(life) = player_query.get_single() {
		println!("Player : life={}", life.0);
	}
}
