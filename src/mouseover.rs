use bevy::prelude::*;
use bevy_mouse_tracking_plugin::{MainCamera, MousePosWorld};

use crate::{Placing, objects::{ObjectData, Orientation}};

#[derive(Debug, PartialEq, Eq)]
pub enum Side {
	First,
	Second,
}
#[derive(Component, Debug, PartialEq, Eq)]
pub struct Hovering { pub index: u32, pub side: Side }
impl PartialOrd for Hovering {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.index.partial_cmp(&other.index)
	}
}
impl Ord for Hovering {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.index.cmp(&other.index)
	}
}

pub fn mouseover_system(
	mut commands: Commands,
	mouse: Query<&MousePosWorld, (Changed<MousePosWorld>, With<MainCamera>)>,
	objects: Query<(Entity, &ObjectData), Without<Placing>>,
) {
	if let Ok(mouse) = mouse.get_single() {
		// info!("Mouse Coords: {}", mouse);
		let mut hover_index = 0;
		for (entity, data) in objects.iter() {
			let loc = data.location;
			let size = data.size();
			let hw = size.x / 2.0;
			let hh = size.y / 2.0;

			if mouse.x > loc.x - hw
				&& mouse.y > loc.y - hh
				&& mouse.x < loc.x + hw
				&& mouse.y < loc.y + hh
			{
				let side = match data.orientation {
					Orientation::Horizontal if mouse.x < loc.x => Side::First,
					Orientation::Vertical if mouse.y < loc.y => Side::First,
					_ => Side::Second,
				};
				commands.entity(entity).insert(Hovering { index: hover_index, side });
				hover_index += 1;
			} else {
				commands.entity(entity).remove::<Hovering>();
			}
		}
	}
}
