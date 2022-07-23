use std::f32::consts::FRAC_1_SQRT_2;

use bevy::prelude::*;
// use bevy_mod_picking::{DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use bevy_mouse_tracking_plugin::{MainCamera, MousePosPlugin, MousePosWorld};
use bevy_pancam::{PanCam, PanCamPlugin};
use mouseover::{Hovering, Side};
use objects::{Object, ObjectData, Orientation};

mod expr;
mod mouseover;
mod name;
mod objects;
mod parse;
mod ui;

use crate::objects::{Binding, Expr};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum AppState {
	Default,
	PlacingObject,
}

fn main() {
	println!("Hello, langjam #0003!");
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(PanCamPlugin::default())
		.add_plugin(MousePosPlugin::SingleCamera)
		.add_startup_system(setup)
		.add_startup_system(ui::ui_setup)
		.add_state(AppState::Default)
		.add_system_set(SystemSet::on_update(AppState::Default).with_system(keyboard_input_system))
		.add_system_set(SystemSet::on_update(AppState::PlacingObject).with_system(placing_system))
		// .add_system(keyboard_input_system)
		.add_system(object_system)
		.add_system(mouseover::mouseover_system)
		.add_system(ui::button_system)
		.init_resource::<GameState>()
		.run();
}

fn setup(mut commands: Commands) {
	commands
		.spawn_bundle(OrthographicCameraBundle::new_2d())
		.insert(MainCamera)
		.insert(PanCam::default());
}

#[derive(Default)]
pub struct GameState {
	placing_orientation: Orientation,
	placing_index: f32,
	top_hovering: Option<Entity>,
}

#[derive(Component, Default, Clone)]
pub struct Placing;

fn place_expr(
	mut commands: Commands,
	app_state: &mut State<AppState>,
	state: &mut GameState,
	expr: Expr,
) {
	commands
		.spawn_bundle(Object { expr, ..default() })
		.insert(Placing);
	app_state.set(AppState::PlacingObject).unwrap();
	state.placing_index += 1.0;
}

// System for triggering things based on keyboard input
fn keyboard_input_system(
	mut commands: Commands,
	mut state: ResMut<GameState>,
	mut app_state: ResMut<State<AppState>>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	if keyboard_input.just_pressed(KeyCode::F) {
		place_expr(
			commands,
			&mut app_state,
			&mut state,
			Expr::Function {
				bind: Binding::None,
				expr: None,
			},
		);
	} else if keyboard_input.just_pressed(KeyCode::V) {
		place_expr(commands, &mut app_state, &mut state, Expr::Variable);
	} else if keyboard_input.just_pressed(KeyCode::A) {
		place_expr(
			commands,
			&mut app_state,
			&mut state,
			Expr::Application {
				func: None,
				args: None,
			},
		);
	} else if keyboard_input.just_pressed(KeyCode::D) {
		if let Some(entity) = state.top_hovering {
			commands.entity(entity).despawn();
		}
	}
}

// System for updating blocks based on external state
fn object_system(
	mut objects: Query<
		(Entity, &ObjectData, &mut Sprite, Option<&Hovering>),
		Without<Placing>,
	>,
	mut state: ResMut<GameState>,
) {
	let mut obj_iter = objects.iter_mut();
	if let Some((mut entity, mut data, mut sprite, mut hovering)) = obj_iter.next() {
		// Find top hovered object
		for (o_entity, o_data, mut o_sprite, o_hovering) in obj_iter {
			if state.top_hovering == Some(o_entity) {
				o_sprite.color = o_data.gen_color(false)
			}
			if hovering < o_hovering {
				entity = o_entity;
				data = o_data;
				sprite = o_sprite;
				hovering = o_hovering;
			}
		}
		state.top_hovering = if let Some(_) = hovering {
			sprite.color = data.gen_color(true);
			Some(entity)
		} else { None }
		
	}
	/* for (_entity, data, expr, mut sprite, hovering) in objects.iter_mut() {
		sprite.color = data.gen_color(expr, hovering.is_some());
	} */
}

// System for placing blocks on the canvas and inside other blocks
fn placing_system(
	mut commands: Commands,
	mouse: Res<Input<MouseButton>>,
	mouse_pos: Res<MousePosWorld>,
	mut state: ResMut<GameState>,
	mut app_state: ResMut<State<AppState>>,
	mut placing: Query<
		(
			Entity,
			&mut ObjectData,
			&mut Expr,
			Option<&mut Sprite>,
			Option<&mut Transform>,
		),
		With<Placing>,
	>,
	mut other_objects: Query<
		(Entity, &mut ObjectData, &mut Expr, Option<&Hovering>),
		Without<Placing>,
	>,
	keyboard_input: Res<Input<KeyCode>>,
	camera_proj: Query<&OrthographicProjection, With<Camera>>,
	asset_server: Res<AssetServer>,
) {
	// Fetch data on block-to-place
	let (entity, mut data, mut expr, sprite, transform) = placing.single_mut();

	data.size = camera_proj.iter().next().unwrap().scale * 512.0; // Scale block-to-place with size
	data.location = Vec2::new(mouse_pos.x, mouse_pos.y); // Move block-to-place to mouse cursor
	data.orientation = state.placing_orientation; // Set orientation based on game state

	let mut obj_iter = other_objects.iter_mut();
	if let Some((mut h_entity, mut h_data, mut h_expr, mut h_hovering)) = obj_iter.next() {
		// Find top hovered object
		for (o_entity, o_data, o_expr, o_hovering) in obj_iter {
			if h_hovering < o_hovering {
				h_entity = o_entity;
				h_data = o_data;
				h_expr = o_expr;
				h_hovering = o_hovering;
			}
		}
		if let Some(hovering) = h_hovering {
			if let Some((side, expr_slot)) = match (&mut *h_expr, hovering.side) {
				(Expr::Function { bind: _, expr }, side) if expr.is_none() => Some((side, expr)),
				(Expr::Application { func, args: _ }, Side::First) if func.is_none() => {
					Some((Side::First, func))
				}
				(Expr::Application { func: _, args }, Side::Second) if args.is_none() => {
					Some((Side::Second, args))
				}
				(_, _) => None,
			} {
				// Check which side of top hovered block we need to place the block we are currently placing.
				let size = (h_data.size * FRAC_1_SQRT_2) * 0.95;
				let mut orientation = h_data.orientation;
				orientation.swap();
				data.orientation = orientation;
				data.size = size;

				let half_h_size_oriented = match h_data.orientation {
					Orientation::Horizontal => Vec2::new(h_data.size / 4.0, 0.0),
					Orientation::Vertical => Vec2::new(0.0, h_data.size / 4.0),
				};
				match side {
					Side::First => data.location = h_data.location - half_h_size_oriented,
					Side::Second => data.location = h_data.location + half_h_size_oriented,
				}

				// Place block inside another block
				if mouse.just_pressed(MouseButton::Left) {
					*expr_slot = Some(entity);
					commands.entity(h_entity).add_child(entity);
					commands.entity(entity).remove::<Placing>();
					app_state.set(AppState::Default).unwrap();
					return;
				}
			}
		}
	}
	
	// Place block on blank canvas
	if mouse.just_pressed(MouseButton::Left) {
		commands.entity(entity).remove::<Placing>();
		app_state.set(AppState::Default).unwrap();
	}

	// Generate / Update visuals from Object data
	if let (Some(mut sprite), Some(mut transform)) = (sprite, transform) {
		*sprite = data.gen_sprite(&expr);
		*transform = data.gen_transform(state.placing_index);
	} else {
		commands.entity(entity).insert_bundle(SpriteBundle {
			sprite: data.gen_sprite(&expr),
			transform: data.gen_transform(state.placing_index),
			texture: data.gen_texture(&expr, &*asset_server),
			..default()
		});
	}

	// Press R to rotate while placing
	if keyboard_input.just_pressed(KeyCode::R) {
		state.placing_orientation.swap();
	}
	// Press Escape to stop placing block
	if keyboard_input.just_pressed(KeyCode::Escape) {
		commands.entity(entity).despawn();
		app_state.set(AppState::Default).unwrap();
	}
	// Change placing Expr variant
	if keyboard_input.just_pressed(KeyCode::A) {
		*expr = Expr::Application { func: None, args: None };
	}
	if keyboard_input.just_pressed(KeyCode::F) {
		*expr = Expr::Function { bind: Binding::None, expr: None };
	}
	if keyboard_input.just_pressed(KeyCode::V) {
		*expr = Expr::Variable;
	}
}
