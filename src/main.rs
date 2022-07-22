use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_mouse_tracking_plugin::{MousePos, MousePosPlugin, MousePosWorld};

use objects::Expr;

use crate::objects::Binding;

mod objects;
mod parse;
mod name;
mod expr;

fn main() {
	println!("Hello, langjam #0003!");
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(PanCamPlugin::default())
		.add_plugin(MousePosPlugin::SingleCamera)
		.add_startup_system(setup)
		.add_system(keyboard_input_system)
		.run();
}


fn setup(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(PanCam::default());
	/* commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			color: Color::rgb(0.25, 0.25, 0.75),
			custom_size: Some(Vec2::new(50.0, 50.0)),
			..default()
		},
		..default()
	}).insert(Expr::Variable); */
}

fn keyboard_input_system(mut commands: Commands, mouse: Res<MousePosWorld>, keyboard_input: Res<Input<KeyCode>>) {
	if keyboard_input.just_pressed(KeyCode::A) {
		info!("'A' just pressed");
		commands.spawn_bundle(SpriteBundle {
			transform: Transform::from_xyz(mouse.x, mouse.y, 0.0),
			sprite: Sprite {
				color: Color::rgb(0.25, 0.25, 0.75),
				custom_size: Some(Vec2::new(50.0, 20.0)),
				..default()
			},
			..default()
		}).insert(Expr::Abstraction { bind: Binding::None, expr: None });
	}
	if keyboard_input.just_pressed(KeyCode::V) {
		info!("'V' just pressed");
		commands.spawn_bundle(SpriteBundle {
			transform: Transform::from_xyz(mouse.x, mouse.y, 0.0),
			sprite: Sprite {
				color: Color::rgb(0.75, 0.25, 0.25),
				custom_size: Some(Vec2::new(50.0, 20.0)),
				..default()
			},
			..default()
		}).insert(Expr::Variable);
	}
}

