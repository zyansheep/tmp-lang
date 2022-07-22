
#[derive(Component)]
enum Expr {
	Abstraction {
		expr: Entity,
	}
	Application {
		func: Entity,
		args: Entity,
	}
	Variable,
}