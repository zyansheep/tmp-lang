use bevy::prelude::*;
// use bevy_mod_picking::{DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};
use bevy_mouse_tracking_plugin::{MainCamera, MousePosPlugin};
use bevy_pancam::{PanCam, PanCamPlugin};
use block::{ObjectData, Orientation, WrappedExpr};
use block_to_expr::block_to_expr;
use expr::Binding;
use mouseover::{BottomHover, HoverState, TopHover};
use placing::place_expr;

mod expr;
mod mouseover;
mod name;
mod block;
mod placing;
mod parse;
mod ui;
mod block_to_expr;

const IMAGE_SIZE: f32 = 300.0;

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub enum AppState {
	#[default]
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
		.add_system_set(SystemSet::on_update(AppState::Default).with_system(input_system).with_system(block_input))

		.add_system_set(SystemSet::on_update(AppState::PlacingObject).with_system(placing::placing_system))

    	.add_system_set(SystemSet::on_update(AppState::WiringObject).with_system(wiring_system))
		.add_event::<WireFromUpdate>()

		.add_system(block::data_update).add_system(block::expr_update).add_system(block::hover_update)
		.add_system(mouseover::mouseover_system)
		.add_system(state_change_detect)
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

fn state_change_detect(app_state: Res<State<AppState>>, mut previous: Local<AppState>) {
	let current = app_state.current();
	if *previous != *current {
		info!("State changed: {:?}", app_state.current());
		*previous = current.clone();
	}
}

#[derive(Default)]
pub struct GameState {
	placing_orientation: Orientation,
	placing_index: f32,
	update_placing_expr: Option<WrappedExpr>,
	just_pressed: bool,
}

// System for triggering things based on keyboard input
fn input_system(
	mut commands: Commands,
	mut state: ResMut<GameState>,
	mut app_state: ResMut<State<AppState>>,
	keyboard_input: Res<Input<KeyCode>>,
	mut mouse_input: ResMut<Input<MouseButton>>,
	mut other_objects: Query<(Entity, &mut ObjectData, &mut block::WrappedExpr, &HoverState)>,
	mut expr_text: Query<&mut Text, With<ui::TextThatShouldBeChangedToExpression>>
) {
	if keyboard_input.just_pressed(KeyCode::F) {
		place_expr(commands, &mut app_state, &mut state, WrappedExpr::LAMBDA);
	} else if keyboard_input.just_pressed(KeyCode::V) {
		place_expr(commands, &mut app_state, &mut state, WrappedExpr::VARIABLE);
	} else if keyboard_input.just_pressed(KeyCode::A) {
		place_expr(commands, &mut app_state, &mut state, WrappedExpr::APPLICATION);
	} else if keyboard_input.just_pressed(KeyCode::C) {
		for (h_entity, mut h_data, mut h_expr, h_hover_state) in other_objects.iter_mut() {
			// if let HoverState::Yes { side, .. } = h_hover_state {
			// 	let text = expr_text.iter().next().unwrap();
			//   text.sections[1].value = match block_to_expr(&h_expr) {
			// 		Ok(expr) => format!("{}", &expr),
			// 		Err(_) => "malformed expression".into(),
			// 	};
			//   break
			// }
		  }
	} /* else if mouse_input.clear_just_pressed(MouseButton::Left) {
		app_state.push(AppState::WiringObject).unwrap();
	} */
}

fn block_input(
	mut commands: Commands,
	mut keyboard_input: ResMut<Input<KeyCode>>,
	objects: Query<(Entity, &HoverState, Option<&TopHover>, Option<&BottomHover>)>,
) {
	for (entity, state, top, bottom) in objects.iter() {
		match (state, top, bottom) {
			(HoverState::Yes { .. }, Some(_), None) => {
				if keyboard_input.clear_just_pressed(KeyCode::C) {
					commands.entity(entity).insert(WireFrom);
				}
			}
			(HoverState::Yes { .. }, None, Some(_)) => {
				
			}
			(HoverState::Yes { .. }, Some(_), Some(_)) => {}
			(HoverState::Yes { .. }, None, None) => {}
			(HoverState::No, None, None) => {}
			_ => { panic!("Invalid Hover component configuration: {entity:?}, {state:?}, {top:?}, {bottom:?}") }
		}
	}
}

#[derive(Component, Debug, Clone, Copy)]
struct WireFrom;
struct WireFromUpdate(Entity);

// Component that travels from Variable to Lambda and once it gets there, it changes the state.
#[derive(Component)]
struct WireFinder {
	bind: Binding<'static>,
}

// System for wiring things up
fn wiring_system(
	mut commands: Commands,
	mut app_state: ResMut<State<AppState>>,
	mut state: ResMut<GameState>,
	mut top_hover: Query<(Entity, &ObjectData, &HoverState), With<TopHover>>,
	mut wiring_from: Query<Entity, With<WireFrom>>,
	mut mouse: ResMut<Input<MouseButton>>,
	mut keyboard: ResMut<Input<KeyCode>>,
) {
	if let Ok(wiring_from) = wiring_from.get_single_mut() {
		if let Ok((entity, data, state)) = top_hover.get_single_mut() {
			if mouse.clear_just_pressed(MouseButton::Left) {
				commands.entity(entity).insert(WireFinder { bind: Binding::End });
			}
		}
		if keyboard.clear_just_pressed(KeyCode::Escape) {
			commands.entity(wiring_from).remove::<WireFrom>();
			app_state.pop().unwrap();
		}
	}
}