//! Systems and structs to deal with blocks.

use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::{GameState, placing::Placing};

pub enum Binding {
	None,
	End,
	Branch(Box<Binding>, Box<Binding>)
}

#[derive(Component, Default)]
pub enum Expr {
	Function { bind: Binding, expr: Option<Entity> },
	Application { func: Option<Entity>, args: Option<Entity> },
	#[default]
	Variable,
}


#[derive(Default, Clone, Copy)]
pub enum Orientation {
	Vertical,
	#[default]
	Horizontal,
}

impl Orientation {
	pub fn swap(self) -> Self {
		match self {
			Self::Horizontal => Self::Vertical,
			Self::Vertical => Self::Horizontal,
		}
	}
}

#[derive(Component, Default, Clone)]
pub struct ObjectData {
	pub orientation: Orientation,
	pub location: Vec2,
	pub size: f32, // Size of longer side
	pub parent: Option<Entity>,
	pub flip: bool,
}

impl ObjectData {
	pub fn gen_color(hovering: bool) -> Color {
		if !hovering { Color::GRAY } else { Color::WHITE }
	}
	pub fn gen_sprite(&self) -> Sprite {
		Sprite {
			custom_size: None,
			color: Self::gen_color(false),
			..default()
		}
	}
	pub fn gen_texture(expr: &Expr, asset_server: &AssetServer) -> Handle<Image> {
		match expr {
			Expr::Variable => asset_server.load("VariableDot.png"),
			Expr::Function { bind: Binding::None, expr: None } => asset_server.load("Lambda.png"),
			Expr::Function { .. } => asset_server.load("LambdaDot.png"),
			Expr::Application { .. } => asset_server.load("Application.png"),
		}
	}
	pub fn gen_transform(&self, z_loc: f32) -> Transform {
		let scale = self.size / crate::IMAGE_SIZE;
		Transform {
			translation: Vec3::new(self.location.x, self.location.y, z_loc),
			rotation: Quat::from_rotation_z(if self.flip { PI } else { 0.0 } + if let Orientation::Vertical = self.orientation { FRAC_PI_2 } else { 0.0 }),
			scale: Vec3::new(scale, scale, 1.0),
		}
	}
	// Gen rectangles of A4-paper size
	pub fn size(&self) -> Vec2 {
		Vec2::new(self.size, self.size * FRAC_1_SQRT_2)
	}
}

#[derive(Bundle, Default)]
pub struct Object {
	pub data: ObjectData,
	pub expr: Expr,
}

pub fn data_update(mut objects: Query<(&ObjectData, &mut Transform), Changed<ObjectData>>) {
	for (data, mut transform) in objects.iter_mut() {
		let index = transform.translation.z;
		*transform = data.gen_transform(index);
	}
}
pub fn expr_update(mut objects: Query<(&Expr, &mut Handle<Image>), Changed<Expr>>, asset_server: Res<AssetServer>) {
	for (expr, mut image) in objects.iter_mut() {
		*image = ObjectData::gen_texture(&expr, &asset_server);
	}
}
// System for updating blocks based on external state
pub fn hover_update(
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