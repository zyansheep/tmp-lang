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
				
				});
			
		});
}