use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2};

use bevy::prelude::*;

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
	pub fn swap(&mut self) {
		*self = match *self {
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
}

impl ObjectData {
	pub fn gen_color(&self, expr: &Expr, hovering: bool) -> Color {
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
	pub fn gen_sprite(&self, expr: &Expr) -> Sprite {
		Sprite {
			custom_size: Some(self.size()),
			color: self.gen_color(expr, false),
			..default()
		}
	}
	pub fn gen_texture(&self, expr: &Expr, asset_server: &AssetServer) -> Handle<Image> {
		match expr {
			Expr::Variable => asset_server.load("VariableDot.png"),
			Expr::Function { bind: Binding::None, expr: None } => asset_server.load("Lambda.png"),
			Expr::Function { .. } => asset_server.load("LambdaDot.png"),
			Expr::Application { .. } => asset_server.load("Application.png"),
		}
	}
	pub fn gen_transform(&self, z_loc: f32) -> Transform {
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
	pub fn size(&self) -> Vec2 {
		Vec2::new(self.size, self.size * FRAC_1_SQRT_2)
	}
}

#[derive(Bundle, Default)]
pub struct Object {
	pub data: ObjectData,
	pub expr: Expr,
}