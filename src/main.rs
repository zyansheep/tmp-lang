use bevy::prelude::*;
use bevy_mod_picking::{DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use bevy_mouse_tracking_plugin::{MousePosPlugin, MousePosWorld};
use bevy_pancam::{PanCam, PanCamPlugin};

use objects::Expr;

use crate::objects::Binding;

mod cli;
mod expr;
mod name;
mod objects;
mod parse;

#[derive(Component)]
struct Hoverable;

fn main() {
	println!("Hello, langjam #0003!");
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(PanCamPlugin::default())
		.add_plugin(MousePosPlugin::SingleCamera)
		.add_plugins(DefaultPickingPlugins) // <- Adds Picking, Interaction, and Highlighting plugins.
        .add_plugin(DebugEventsPickingPlugin)
		.add_startup_system(setup)
		.add_system(keyboard_input_system)
    	.add_system(mouseover_system)
		.run();
}

fn setup(mut commands: Commands) {
	commands
		.spawn_bundle(OrthographicCameraBundle::new_2d())
		.insert(PanCam::default())
    	.insert_bundle(PickingCameraBundle::default());
}

fn mouseover_system(
	mouse: Res<MousePosWorld>,
	objects: Query<(&Sprite, &Transform), With<Hoverable>>,
) {
	for object in objects.iter() {

	}
	for (sprite, transform) in objects.iter() {
		if sprite
        /* match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => info!("Gee Willikers, it's a click! {:?}", e),
        } */
    }
}

fn keyboard_input_system(
	mut commands: Commands,
	mouse: Res<MousePosWorld>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	if keyboard_input.just_pressed(KeyCode::A) {
		info!("'A' just pressed");
		commands
			.spawn_bundle(SpriteBundle {
				transform: Transform::from_xyz(mouse.x, mouse.y, 0.0),
				sprite: Sprite {
					color: Color::rgb(0.25, 0.25, 0.75),
					custom_size: Some(Vec2::new(50.0, 20.0)),
					..default()
				},
				..default()
			})
			.insert(Expr::Abstraction {
				bind: Binding::None,
				expr: None,
			}).insert(Hoverable);
	}
	if keyboard_input.just_pressed(KeyCode::V) {
		info!("'V' just pressed");
		commands
			.spawn_bundle(SpriteBundle {
				transform: Transform::from_xyz(mouse.x, mouse.y, 0.0),
				sprite: Sprite {
					color: Color::rgb(0.75, 0.25, 0.25),
					custom_size: Some(Vec2::new(50.0, 20.0)),
					..default()
				},
				..default()
			})
			.insert(Expr::Variable)
			.insert(Hoverable);
	}
}
