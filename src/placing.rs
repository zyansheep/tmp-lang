//! Systems and structs to place Blocks

use std::f32::consts::FRAC_1_SQRT_2;

use bevy::prelude::*;
use bevy_mouse_tracking_plugin::{MainCamera, MousePosWorld};

use crate::{AppState, GameState, block::{Binding, Expr, Object, ObjectData, Orientation}, mouseover::{HoverState, Side}};

#[derive(Component, Default, Clone)]
pub struct Placing;

pub fn place_expr(
	mut commands: Commands,
	app_state: &mut State<AppState>,
	state: &mut GameState,
	expr: Expr,
) {
	match app_state.current() {
		AppState::Default => {
			state.just_placed = true; // Prevent a single mouse click
			commands
				.spawn_bundle(Object { expr, ..default() })
				.insert(Placing);
			app_state.set(AppState::PlacingObject).unwrap();
			state.placing_index += 1.0;
		}
		AppState::PlacingObject => {
			state.just_placed = true; // Prevent a single mouse click
			state.update_placing_expr = Some(expr);
		}
		_ => {},
	}
	
}

// System for placing blocks on the canvas and inside other blocks
pub fn placing_system(
	mut commands: Commands,
	mut mouse: ResMut<Input<MouseButton>>,
	mouse_pos: Res<MousePosWorld>,
	mut state: ResMut<GameState>,
	mut app_state: ResMut<State<AppState>>,
	mut placing: Query<(Entity, &mut ObjectData, &mut Expr, Option<&mut Sprite>), With<Placing>>,
	mut other_objects: Query<(Entity, &mut ObjectData, &mut Expr, &HoverState), (Without<Placing>, Changed<HoverState>)>,
	keyboard_input: Res<Input<KeyCode>>,
	camera_proj: Query<&OrthographicProjection, With<MainCamera>>,
	asset_server: Res<AssetServer>,
) {
	if state.just_placed {
		state.just_placed = false;
		mouse.clear_just_pressed(MouseButton::Left);
	}
	// Fetch data on block-to-place
	let (entity, mut data, mut expr, sprite) = placing.single_mut();

	if let Some(new_expr) = state.update_placing_expr.take() {
		*expr = new_expr;
	}

	data.size = camera_proj.iter().next().unwrap().scale * 300.0; // Scale block-to-place with size
	data.location = Vec2::new(mouse_pos.x, mouse_pos.y); // Move block-to-place to mouse cursor
	data.orientation = state.placing_orientation; // Set orientation based on game state

	for (h_entity, mut h_data, mut h_expr, h_hover_state) in other_objects.iter_mut() {
		if let HoverState::Yes { side, top: true, .. } = h_hover_state {
			// Make sure we can place block
			if let Some((side, expr_slot)) = match (&mut *h_expr, side) {
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
