use bevy::prelude::*;

pub enum Binding {
	None,
	End,
	Branch(Box<Binding>, Box<Binding>)
}

#[derive(Component, Default)]
pub enum Expr {
	Function { bind: Binding, expr: Option<Entity> },
	Application { func: Entity, args: Option<Entity> },
	#[default]
	Variable,
}
