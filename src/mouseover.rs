//! Systems to detect mouse over blocks

use bevy::prelude::*;
use bevy_mouse_tracking_plugin::{MainCamera, MousePosWorld};

use crate::{GameState, placing::Placing, block::{ObjectData, Orientation}};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Side {
	First,
	Second,
}
#[derive(Component, Debug, PartialEq)]
pub struct Hovering { pub order: f32, pub side: Side }
impl PartialOrd for Hovering {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.order.partial_cmp(&other.order)
	}
}

// Mark objects as currently being hovered over.
pub fn mouseover_system(
	mut commands: Commands,
	mouse: Query<&MousePosWorld, (Changed<MousePosWorld>, With<MainCamera>)>,
	objects: Query<(Entity, &ObjectData, &GlobalTransform), Without<Placing>>,
	mut state: ResMut<GameState>,
) {
	let (mut cur_hover_order, mut cur_entity) = (f32::MAX, None::<Entity>);
	if let Ok(mouse) = mouse.get_single() {
		// info!("Mouse Coords: {}", mouse);
		for (entity, data, transform) in objects.iter() {
			let loc = transform.translation;
			let size = data.size();

			let (hw, hh) = match data.orientation {
				Orientation::Horizontal => (size.x / 2.0, size.y / 2.0),
				Orientation::Vertical => (size.y / 2.0, size.x / 2.0),
			};

			if mouse.x > loc.x - hw
				&& mouse.y > loc.y - hh
				&& mouse.x < loc.x + hw
				&& mouse.y < loc.y + hh
			{
				// info!("{:?}: {} < {} < {}, {} < {} < {}", entity, loc.x - hw, mouse.x, loc.x + hw, loc.y - hh, mouse.y, loc.y + hh);
				
				let side = match data.orientation {
					Orientation::Horizontal if mouse.x < loc.x => Side::First,
					Orientation::Vertical if mouse.y < loc.y => Side::First,
					_ => Side::Second,
				};
				// Order Hovered objects by their size, smallest hovered object should be the one highlighted
				let hovering = Hovering { order: data.size, side };
				if hovering.order <= cur_hover_order {
					cur_hover_order = hovering.order;
					cur_entity = Some(entity);
				}
				commands.entity(entity).insert(hovering);
				
			} else {
				commands.entity(entity).remove::<Hovering>();
			}
		}
		state.top_hovering = cur_entity;
	}
}
