use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2};

use bevy::prelude::*;
// use bevy_mod_picking::{DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use bevy_mouse_tracking_plugin::{MainCamera, MousePosPlugin, MousePosWorld};
use bevy_pancam::{PanCam, PanCamPlugin};

mod expr;
mod name;
mod parse;
mod objects;
mod ui;

use crate::{objects::{Binding, Expr}};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum AppState {
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
		.add_system(mouseover_system.before(object_system))
		.add_system(ui::button_system)
		.init_resource::<GameState>()
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands
		.spawn_bundle(OrthographicCameraBundle::new_2d())
		.insert(MainCamera)
		.insert(PanCam::default());

	asset_server.load_folder("assets");
}

#[derive(Default)]
struct GameState {
	placing_orientation: Orientation,
	placing_index: f32,
	hovering: Option<Entity>,
}

#[derive(Debug, PartialEq, Eq)]
enum Side {
	First,
	Second,
}
#[derive(Component, Debug, PartialEq, Eq)]
struct Hovering(u32, Side);
impl PartialOrd for Hovering {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(&other.0)
	}
}
impl Ord for Hovering {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.cmp(&other.0)
	}
}

fn mouseover_system(
	mut commands: Commands,
	mouse: Query<&MousePosWorld, (Changed<MousePosWorld>, With<MainCamera>)>,
	objects: Query<(Entity, &ObjectData), Without<Placing>>,
) {
	if let Ok(mouse) = mouse.get_single() {
		// info!("Mouse Coords: {}", mouse);
		let mut hover_index = 0;
		for (entity, data) in objects.iter() {
			let loc = data.location;
			let size = data.size();
			let hw = size.x / 2.0;
			let hh = size.y / 2.0;

			if mouse.x > loc.x - hw
				&& mouse.y > loc.y - hh
				&& mouse.x < loc.x + hw
				&& mouse.y < loc.y + hh
			{
				let side = match data.orientation {
					Orientation::Horizontal if mouse.x < loc.x => Side::First,
					Orientation::Vertical if mouse.y < loc.y => Side::First,
					_ => Side::Second,
				};
				commands.entity(entity).insert(Hovering(hover_index, side));
				hover_index += 1;
			} else {
				commands.entity(entity).remove::<Hovering>();
			}
		}
	}
}

#[derive(Default, Clone, Copy)]
enum Orientation {
	Vertical,
	#[default]
	Horizontal,
}

impl Orientation {
	fn swap(&mut self) {
		*self = match *self {
			Self::Horizontal => Self::Vertical,
			Self::Vertical => Self::Horizontal,
		}
	}
}

#[derive(Component, Default, Clone)]
struct ObjectData {
	orientation: Orientation,
	location: Vec2,
	size: f32, // Size of longer side
}

impl ObjectData {
	fn gen_color(&self, expr: &Expr, hovering: bool) -> Color {
		let color = match expr {
			Expr::Function { .. } => Color::BLUE,
			Expr::Application { .. } => Color::GRAY,
			Expr::Variable => Color::RED,
		};
		if !hovering {
			color
		} else {
			color + Color::rgb_u8(100, 100, 100)
		}
	}
	fn gen_sprite(&self, expr: &Expr) -> Sprite {
		Sprite {
			custom_size: Some(self.size()),
			color: self.gen_color(expr, false),
			..default()
		}
	}
	fn gen_texture(&self, expr: &Expr, asset_server: &AssetServer) -> Handle<Image> {
		match expr {
			Expr::Variable => asset_server.load("VariableDot.png"),
			Expr::Function { bind: Binding::None, expr: None } => asset_server.load("Lambda.png"),
			Expr::Function { .. } => asset_server.load("LambdaDot.png"),
			Expr::Application { .. } => asset_server.load("Application.png"),
		}
	}
	fn gen_transform(&self, z_loc: f32) -> Transform {
		Transform {
			translation: Vec3::new(self.location.x, self.location.y, z_loc),
			rotation: match self.orientation {
				Orientation::Horizontal => Quat::IDENTITY,
				Orientation::Vertical => Quat::from_rotation_z(FRAC_PI_2),
			},
			scale: Vec3::ONE,
		}
	}

	// Gen rectangles of A4-paper size
	fn size(&self) -> Vec2 {
		Vec2::new(self.size, self.size * FRAC_1_SQRT_2)
	}
}

#[derive(Bundle, Default)]
struct Object {
	data: ObjectData,
	expr: Expr,
	placing: Placing,
}

#[derive(Component, Default, Clone)]
struct Placing;

fn keyboard_input_system(
	mut commands: Commands,
	mut state: ResMut<GameState>,
	mut app_state: ResMut<State<AppState>>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	if keyboard_input.just_pressed(KeyCode::F) {
		info!("Placing Function Block");
		commands.spawn_bundle(Object {
			expr: Expr::Function {
				bind: Binding::None,
				expr: None,
			},
			..default()
		});
		app_state.set(AppState::PlacingObject).unwrap();
		state.placing_index += 1.0;
	} else if keyboard_input.just_pressed(KeyCode::V) {
		info!("Placing Variable Block");
		commands.spawn_bundle(Object::default());
		app_state.set(AppState::PlacingObject).unwrap();
		state.placing_index += 1.0;
	} else if keyboard_input.just_pressed(KeyCode::A) {
		info!("Placing Application Block");
		commands.spawn_bundle(Object {
			expr: Expr::Application {
				func: None,
				args: None,
			},
			..default()
		});
		app_state.set(AppState::PlacingObject).unwrap();
		state.placing_index += 1.0;
	} else if keyboard_input.just_pressed(KeyCode::D) {
		info!("Deleting Block");
		if let Some(entity) = state.hovering {
			commands.entity(entity).despawn();
		}
	}
}

fn object_system(
	mut objects: Query<
		(Entity, &ObjectData, &Expr, &mut Sprite, Option<&Hovering>),
		Without<Placing>,
	>,
	mut state: ResMut<GameState>,
) {
	let mut obj_iter = objects.iter_mut();
	if let Some((mut entity, mut data, mut expr, mut sprite, mut hovering)) = obj_iter.next() {
		// Find top hovered object
		for (o_entity, o_data, o_expr, o_sprite, o_hovering) in obj_iter {
			if hovering <= o_hovering {
				entity = o_entity;
				data = o_data;
				sprite = o_sprite;
				expr = o_expr;
				hovering = o_hovering;
			}
			sprite.color = data.gen_color(expr, false)
		}
		sprite.color = data.gen_color(expr, hovering.is_some());
		state.hovering = Some(entity);
	}
	/* for (_entity, data, expr, mut sprite, hovering) in objects.iter_mut() {
		sprite.color = data.gen_color(expr, hovering.is_some());
	} */
}

fn placing_system(
	mut commands: Commands,
	mouse: Res<Input<MouseButton>>,
	mouse_pos: Res<MousePosWorld>,
	mut state: ResMut<GameState>,
	mut app_state: ResMut<State<AppState>>,
	mut placing: Query<(Entity, &mut ObjectData, &Expr, Option<&mut Sprite>, Option<&mut Transform>), With<Placing>>,
	mut other_objects: Query<(&mut ObjectData, Option<&Hovering>), Without<Placing>>,
	keyboard_input: Res<Input<KeyCode>>,
	camera_proj: Query<&OrthographicProjection, With<Camera>>,
	asset_server: Res<AssetServer>,
) {
	let (entity, mut data, expr, sprite, transform) = placing.single_mut();
	data.size = camera_proj.iter().next().unwrap().scale * 512.0;
	data.location = Vec2::new(mouse_pos.x, mouse_pos.y);
	data.orientation = state.placing_orientation;

	let mut obj_iter = other_objects.iter_mut();
	if let Some((mut h_data, mut h_hovering)) = obj_iter.next() {
		// Find top hovered object
		for (o_data, o_hovering) in obj_iter {
			if h_hovering < o_hovering {
				h_data = o_data;
				h_hovering = o_hovering;
			}
		}
		if let Some(hovering) = h_hovering {
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
			match hovering.1 {
				Side::First => {
					data.location = h_data.location - half_h_size_oriented;
				}
				Side::Second => {
					data.location = h_data.location + half_h_size_oriented;
				}
			}
		}
	}
	// If sprite exists, update it, otherwise create new sprite
	if let (Some(mut sprite), Some(mut transform)) = (sprite, transform) {
		*sprite = data.gen_sprite(expr);
		*transform = data.gen_transform(state.placing_index);
	} else {
		commands.entity(entity).insert_bundle(SpriteBundle {
			sprite: data.gen_sprite(expr),
			transform: data.gen_transform(state.placing_index),
			texture: data.gen_texture(expr, &*asset_server),
			..default()
		});
	}

	// Press R to rotate while placing
	if keyboard_input.just_pressed(KeyCode::R) {
		state.placing_orientation.swap();
	} else if keyboard_input.just_pressed(KeyCode::Escape) {
		commands.entity(entity).despawn();
		app_state.set(AppState::Default).unwrap();
	}

	// Place block on Left Click
	if mouse.just_pressed(MouseButton::Left) {
		commands.entity(entity).remove::<Placing>();
		app_state.set(AppState::Default).unwrap();
	}
}
