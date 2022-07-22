use bevy::prelude::*;

pub enum Binding {
	None,
	End,
	Branch(Box<Binding>, Box<Binding>)
}

#[derive(Component)]
pub enum Expr {
	Abstraction { bind: Binding, expr: Option<Entity> },
	Application { func: Entity, args: Option<Entity> },
	Variable,
}
