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

const IMAGE_SIZE: f32 = 300.0;

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
		.add_system(data_update).add_system(expr_update).add_system(hover_update)
		.add_system(mouseover::mouseover_system)
		.add_system(ui::button_system)
    	.add_system(bevy::window::exit_on_window_close_system)
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
			commands.entity(entity).despawn()
		}
	}
}

fn data_update(mut objects: Query<(&ObjectData, &mut Transform), Changed<ObjectData>>) {
	for (data, mut transform) in objects.iter_mut() {
		let index = transform.translation.z;
		*transform = data.gen_transform(index);
	}
}
fn expr_update(mut objects: Query<(&Expr, &mut Handle<Image>), Changed<Expr>>, asset_server: Res<AssetServer>) {
	for (expr, mut image) in objects.iter_mut() {
		*image = ObjectData::gen_texture(&expr, &asset_server);
	}
}
// System for updating blocks based on external state
fn hover_update(
	mut objects: Query<
		(Entity, &mut Sprite),
		Without<Placing>,
	>,
	state: ResMut<GameState>,
) {
	for (entity, mut sprite) in objects.iter_mut() {
		sprite.color = ObjectData::gen_color(state.top_hovering == Some(entity));
	}
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
		),
		With<Placing>,
	>,
	mut other_objects: Query<
		(Entity, &mut ObjectData, &mut Expr, &Hovering),
		Without<Placing>,
	>,
	keyboard_input: Res<Input<KeyCode>>,
	camera_proj: Query<&OrthographicProjection, With<MainCamera>>,
	asset_server: Res<AssetServer>,
) {
	// Fetch data on block-to-place
	let (entity, mut data, mut expr, sprite) = placing.single_mut();

	data.size = camera_proj.iter().next().unwrap().scale * 300.0; // Scale block-to-place with size
	data.location = Vec2::new(mouse_pos.x, mouse_pos.y); // Move block-to-place to mouse cursor
	data.orientation = state.placing_orientation; // Set orientation based on game state

	for (h_entity, mut h_data, mut h_expr, h_hovering) in other_objects.iter_mut() {
		if state.top_hovering == Some(h_entity) {
			// Make sure we can place block
			if let Some((side, expr_slot)) = match (&mut *h_expr, h_hovering.side) {
				(Expr::Function { bind: _, expr }, Side::First) if expr.is_none() => {
					h_data.flip = true; // Make sure the dot is on the right side of the Function block texture
					Some((Side::First, expr))
				},
				(Expr::Function { bind: _, expr }, Side::Second) if expr.is_none() => Some((Side::Second, expr)),
				(Expr::Application { func, args: _ }, Side::First) if func.is_none() => {
					Some((Side::First, func))
				}
				(Expr::Application { func: _, args }, Side::Second) if args.is_none() => {
					Some((Side::Second, args))
				}
				(_, _) => None,
			} {
				let size = (h_data.size * FRAC_1_SQRT_2) * 0.90;
				data.orientation = h_data.orientation.swap();
				data.size = size;

				let half_h_size_oriented = match h_data.orientation {
					Orientation::Horizontal => Vec2::new(h_data.size / 4.0, 0.0),
					Orientation::Vertical => Vec2::new(0.0, h_data.size / 4.0),
				};
				let relative_loc = match side {
					Side::First => -half_h_size_oriented,
					Side::Second => half_h_size_oriented,
				};
				data.location = h_data.location + relative_loc;

				// Place block inside another block
				if mouse.just_pressed(MouseButton::Left) {
					*expr_slot = Some(entity);
					data.parent = Some(h_entity);
					// commands.entity(h_entity).add_child(entity); // DONT DO THIS, YOUR LIFE WILL BE PAINNNN
					commands.entity(entity).remove::<Placing>();
					app_state.set(AppState::Default).unwrap();
					return;
				}
			}
			break
		}
	}
	// Place block on blank canvas (if there are no objects in scene)
	if mouse.just_pressed(MouseButton::Left) && other_objects.is_empty() {
		commands.entity(entity).remove::<Placing>();
		app_state.set(AppState::Default).unwrap();
	}
	
	// Generate / Update visuals from Object data
	if sprite.is_none() {
		commands.entity(entity).insert_bundle(SpriteBundle {
			sprite: data.gen_sprite(),
			transform: data.gen_transform(state.placing_index),
			texture: ObjectData::gen_texture(&expr, &*asset_server),
			..default()
		});
	}

	// Press R to rotate while placing
	if keyboard_input.just_pressed(KeyCode::R) {
		state.placing_orientation = state.placing_orientation.swap();
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
