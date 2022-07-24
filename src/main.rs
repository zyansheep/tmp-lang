use bevy::prelude::*;
// use bevy_mod_picking::{DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use bevy_mouse_tracking_plugin::{MainCamera, MousePosPlugin};
use bevy_pancam::{PanCam, PanCamPlugin};
use block::{Orientation};
use placing::place_expr;

mod expr;
mod mouseover;
mod name;
mod block;
mod placing;
mod parse;
mod ui;

use crate::block::{Binding, Expr};

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
		.add_system_set(SystemSet::on_update(AppState::PlacingObject).with_system(placing::placing_system))
		.add_system(block::data_update).add_system(block::expr_update).add_system(block::hover_update)
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
		.insert(PanCam { track_mouse: true, ..default() });
}

#[derive(Default)]
pub struct GameState {
	placing_orientation: Orientation,
	placing_index: f32,
	top_hovering: Option<Entity>,
	just_placed: bool,
	update_placing_expr: Option<Expr>,
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
