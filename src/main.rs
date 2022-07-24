use bevy::{input::keyboard::KeyboardInput, prelude::*};
// use bevy_mod_picking::{DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use bevy_mouse_tracking_plugin::{MainCamera, MousePosPlugin};
use bevy_pancam::{PanCam, PanCamPlugin};
use block::{ObjectData, Orientation};
use mouseover::{BottomHover, HoverState, TopHover};
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
	WiringObject,
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
		.add_system_set(SystemSet::on_update(AppState::Default).with_system(input_system))

		.add_system_set(SystemSet::on_update(AppState::PlacingObject).with_system(placing::placing_system))

    	.add_system_set(SystemSet::on_update(AppState::WiringObject).with_system(wiring_system))
		.add_event::<WireFromUpdate>()

		.add_system(block::data_update).add_system(block::expr_update).add_system(block::hover_update)
		.add_system(mouseover::mouseover_system)
		// .add_system(state_change)
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

/* fn state_change(app_state: Res<State<AppState>>) {
	if app_state.is_changed() {
		info!("State changed: {:?}", app_state.current());
	}
} */

#[derive(Default)]
pub struct GameState {
	placing_orientation: Orientation,
	placing_index: f32,
	update_placing_expr: Option<Expr>,
	just_pressed: bool,
}

// System for triggering things based on keyboard input
fn input_system(
	mut commands: Commands,
	mut state: ResMut<GameState>,
	mut app_state: ResMut<State<AppState>>,
	keyboard_input: Res<Input<KeyCode>>,
	mut mouse_input: ResMut<Input<MouseButton>>,
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
	} else if mouse_input.clear_just_pressed(MouseButton::Left) {
		app_state.push(AppState::WiringObject).unwrap();
	}
}

fn block_input(
	mut commands: Commands,
	mut keyboard_input: ResMut<Input<KeyCode>>,
	objects: Query<(&HoverState, Option<&TopHover>, Option<&BottomHover>)>
) {
	for (state, top, bottom) in objects.iter() {
		match (state, top, bottom) {
			(HoverState::Yes { .. }, Some(_), None) => {
				if keyboard_input.clear_just_pressed(KeyCode::C) {
					
				}
			}
			(HoverState::Yes { .. }, None, Some(_)) => {
				
			}
			(HoverState::No, None, None) => {}
			_ => { panic!("Invalid Hover component configuration") }
		}
	}
}

#[derive(Component, Debug, Clone, Copy)]
struct WireFrom;
struct WireFromUpdate(Entity);

// System for wiring things up
fn wiring_system(
	mut commands: Commands,
	mut app_state: ResMut<State<AppState>>,
	mut state: ResMut<GameState>,
	mut top_hover: Query<(Entity, &ObjectData, &HoverState), With<TopHover>>,
	mut wiring_from: Query<Entity, With<WireFrom>>,
	mut update_wire_from: EventReader<WireFromUpdate>
) {
	if !update_wire_from.is_empty() {
		if let Ok(entity) = wiring_from.get_single() {
			commands.entity(entity).remove::<WireFrom>();
		}
		if let Some(ev) = update_wire_from.iter().last() {
			commands.entity(ev.0).insert(WireFrom);
			return;
		}
	}
	/* if h_hover_state.is_top() {
					
	} */
	/* if let Some(wiring_from) = wiring_from {
		/* if let Expr::Function { bind, expr: Some(expr_entity) } = expr {
			for (h_entity, h_data, h_hover_state) in hover_set.iter_mut() {
				
			}
		} else {
			commands.entity(entity).remove::<WiringFrom>();
			app_state.pop().unwrap()
		} */
	} else {
		// detect hovering
	} */
}