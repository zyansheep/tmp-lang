use std::f32::consts::FRAC_1_SQRT_2;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
// use bevy_mod_picking::{DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use bevy_mouse_tracking_plugin::{MousePosPlugin, MousePosWorld};
use bevy_pancam::{PanCam, PanCamPlugin};

use objects::Expr;

use crate::objects::Binding;

mod expr;
mod name;
mod objects;
mod parse;

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
    	.add_state(AppState::Default)
		.add_system_set(
			ConditionSet::new()
				.run_in_state(AppState::Default)
				.with_system(keyboard_input_system)
				.with_system(mouseover_system)
				.into()
		)
		.add_system_set(
			ConditionSet::new()
				.run_in_state(AppState::PlacingObject)
				.with_system(placing_system)
				.into()
		)
    	.add_system(object_system)
		.init_resource::<GameState>()
		.run();
}

fn setup(mut commands: Commands) {
	commands
		.spawn_bundle(OrthographicCameraBundle::new_2d())
		.insert(PanCam::default());
}

#[derive(Default)]
struct GameState {
	hovering: Option<Entity>,
	placing_orientation: Orientation,
	placing_size: f32,
}

fn mouseover_system(
	mouse: Res<MousePosWorld>,
	mut state: ResMut<GameState>,
	objects: Query<(Entity, &Sprite, &Transform), With<ObjectData>>,
) {
	let old_state = state.hovering;
	let mut found_hover = false;
	for (entity, sprite, transform) in objects.iter() {
		if let Some(size) = sprite.custom_size {
			let hw = size.x / 2.0;
			let hh = size.y / 2.0;
			if mouse.x > transform.translation.x - hw &&
				mouse.y > transform.translation.y - hh &&
				mouse.x < transform.translation.x + hw &&
				mouse.y < transform.translation.y + hh
			{
				state.hovering = Some(entity);
				found_hover = true;
			}
		}
    }
	if !found_hover { state.hovering = None }

	if state.hovering != old_state {
		info!("Hovering: {:?}", state.hovering);
	}
}

#[derive(Default, Clone, Copy)]
enum Orientation { Vertical, #[default] Horizontal }
impl Orientation { fn swap(&mut self) { *self = match *self { Self::Horizontal => Self::Vertical, Self::Vertical => Self::Horizontal } } }

#[derive(Component, Default)]
struct ObjectData {
	orientation: Orientation,
	location: Vec2,
	size: f32, // Size of longer side
}
impl ObjectData {
	fn gen_sprite(&self, expr: &Expr) -> Sprite {
		Sprite {
			custom_size: Some(match self.orientation {
				Orientation::Horizontal => Vec2::new(self.size, self.size * FRAC_1_SQRT_2),
				Orientation::Vertical => Vec2::new(self.size * FRAC_1_SQRT_2, self.size),
			}),
			color: match expr {
				Expr::Function { .. } => Color::BLUE,
				Expr::Application { .. } => Color::GRAY,
				Expr::Variable => Color::RED
			},
    		..default()
		}
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
	match app_state.current() {
		AppState::Default => {
			if keyboard_input.just_pressed(KeyCode::F) {
				info!("Placing Function Block");
				commands.spawn_bundle(Object {
					expr: Expr::Function {
						bind: Binding::None,
						expr: None,
					}, ..default()
				});
				app_state.set(AppState::PlacingObject);
			} else if keyboard_input.just_pressed(KeyCode::V) {
				info!("Placing Variable Block");
				commands.spawn_bundle(Object::default());
				app_state.set(AppState::PlacingObject);
			}
		}
		AppState::PlacingObject => {
			if keyboard_input.just_pressed(KeyCode::R) {
				state.placing_orientation.swap()
			}
		}
	}
}

fn object_system(
	mut commands: Commands,
	mut state: ResMut<GameState>,
	objects: Query<(Entity, &ObjectData, &Expr, Option<&Sprite>), Without<Placing>>,
) {
	for (entity, data, expr, sprite) in objects.iter() {
		let sprite = sprite.get_or_insert()
		if sprite.is_none() {
			commands.entity(entity).insert_bundle(SpriteBundle {
				sprite: data.gen_sprite(expr),
				transform: Transform::from_xyz(data.location.x, data.location.y, 0.0),
				..default()
			})
		}
	}
}

fn placing_system(
	mut commands: Commands,
	mut app_state: ResMut<State<AppState>>,
	mouse: Res<Input<MouseButton>>,
	mouse_pos: Res<MousePosWorld>,
	placing: Query<(Entity, &mut ObjectData), With<Placing>>
) {
	let (entity, mut data) = placing.iter_mut().next().unwrap();
	data.location = Vec2::new(mouse_pos.x, mouse_pos.y);

	if mouse.just_pressed(MouseButton::Left) {
		commands.entity(entity).remove::<Placing>();
	}
}