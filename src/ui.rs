use bevy::prelude::*;

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
									"Visualize",
									TextStyle {
										font: asset_server.load("fonts/Inter.ttf"),
										font_size: 25.,
										color: Color::WHITE,
									},
									Default::default(),
								),
								..default()
							});
						})
						.insert(ClickHandler(|| info!("button clicked")));
				});
		});
}

#[derive(Component)]
pub struct ClickHandler(fn() -> ());

#[allow(clippy::type_complexity)]
pub fn button_system(
	mut interaction_query: Query<
		(&Interaction, &mut UiColor, &Children),
		(Changed<Interaction>, With<Button>),
	>,
	click_handler_query: Query<&ClickHandler, With<Button>>,
) {
	for (interaction, mut color, _children) in interaction_query.iter_mut() {
		// let mut text = text_query.get_mut(children[0]).unwrap();
		match *interaction {
			Interaction::Clicked => {
				// text.sections[0].value = "Press".to_string();
				*color = Color::rgb(0.15, 0.15, 0.15).into();
				for handler in click_handler_query.iter() {
					handler.0();
				}
			}
			Interaction::Hovered => {
				// text.sections[0].value = "Hover".to_string();
				*color = Color::rgb(0.25, 0.25, 0.25).into();
			}
			Interaction::None => {
				// text.sections[0].value = "Button".to_string();
				*color = Color::rgb(0.35, 0.75, 0.35).into();
			}
		};
	}
}
