use bevy::prelude::*;

use crate::{
	block::{Binding, Expr},
	place_expr, AppState, GameState,
};

pub fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(UiCameraBundle::default());

	commands
		.spawn_bundle(NodeBundle {
			transform: Transform {
				translation: Vec3::new(0., 0., 0.),
				..default()
			},
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::SpaceBetween,
				..default()
			},
			color: Color::NONE.into(),
			..default()
		})
		.with_children(|parent| {
			// right vertical fill
			parent
				.spawn_bundle(NodeBundle {
					style: Style {
						padding: Rect {
							top: Val::Px(15.0),
							bottom: Val::Px(15.0),
							..default()
						},
						flex_direction: FlexDirection::ColumnReverse,
						justify_content: JustifyContent::FlexStart,
						size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
						..default()
					},
					color: Color::rgb(0.15, 0.15, 0.15).into(),
					..default()
				})
				.with_children(|parent| {
					// Title
					parent.spawn_bundle(TextBundle {
						style: Style {
							size: Size::new(Val::Undefined, Val::Px(25.)),
							margin: Rect {
								left: Val::Auto,
								right: Val::Auto,
								..default()
							},
							..default()
						},
						text: Text::with_section(
							"TMP App",
							TextStyle {
								font: asset_server.load("fonts/Inter.ttf"),
								font_size: 25.,
								color: Color::WHITE,
							},
							Default::default(),
						),
						..default()
					});

					build_button(
						parent,
						asset_server.load("fonts/Inter.ttf"),
						"Add variable",
						|commands, app_state, state| {
							place_expr(commands, app_state, state, Expr::Variable);
							info!("Added variable");
						},
					);
					build_button(
						parent,
						asset_server.load("fonts/Inter.ttf"),
						"Add function",
						|commands, app_state, state| {
							place_expr(
								commands,
								app_state,
								state,
								Expr::Function {
									bind: Binding::None,
									expr: None,
								},
							);
							info!("Added function");
						},
					);
					build_button(
						parent,
						asset_server.load("fonts/Inter.ttf"),
						"Add application",
						|commands, app_state, state| {
							place_expr(
								commands,
								app_state,
								state,
								Expr::Application {
									func: None,
									args: None,
								},
							);
							info!("Added application");
						},
					);
				});
		});
}

fn build_button(
	parent: &mut ChildBuilder,
	font: Handle<Font>,
	text: &str,
	handler: ClickHandlerFunction,
) {
	parent
		.spawn_bundle(ButtonBundle {
			style: Style {
				padding: Rect::all(Val::Px(10.)),
				margin: Rect {
					left: Val::Auto,
					right: Val::Auto,
					top: Val::Px(20.),
					..default()
				},
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..default()
			},
			color: Color::rgb(0.35, 0.75, 0.35).into(),
			..default()
		})
		.with_children(|parent| {
			parent.spawn_bundle(TextBundle {
				style: Style {
					size: Size::new(Val::Undefined, Val::Undefined),
					margin: Rect {
						left: Val::Auto,
						right: Val::Auto,
						..default()
					},
					..default()
				},
				text: Text::with_section(
					text,
					TextStyle {
						font,
						font_size: 25.,
						color: Color::WHITE,
					},
					Default::default(),
				),
				..default()
			});
		})
		.insert(ClickHandler(handler));
}

#[derive(Component)]
pub struct ClickHandler(ClickHandlerFunction);
type ClickHandlerFunction =
	fn(commands: Commands, app_state: &mut State<AppState>, state: &mut GameState) -> ();

#[allow(clippy::type_complexity)]
pub fn button_system(
	commands: Commands,
	mut state: ResMut<GameState>,
	mut app_state: ResMut<State<AppState>>,
	mut interaction_query: Query<
		(&Interaction, &mut UiColor, &Children),
		(Changed<Interaction>, With<Button>),
	>,
	click_handler_query: Query<&ClickHandler, With<Button>>,
) {
	// for (interaction, mut color, _children) in interaction_query.iter_mut() {
	if let Some((interaction, mut color, _children)) = interaction_query.iter_mut().next() {
		match *interaction {
			Interaction::Clicked => {
				*color = Color::rgb(0.15, 0.15, 0.15).into();
				click_handler_query.iter().next().unwrap().0(commands, &mut app_state, &mut state);
			}
			Interaction::Hovered => {
				*color = Color::rgb(0.25, 0.25, 0.25).into();
			}
			Interaction::None => {
				*color = Color::rgb(0.35, 0.75, 0.35).into();
			}
		};
	}
	// }
}
