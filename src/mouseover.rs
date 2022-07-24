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
pub enum HoverState {
	Yes { order: f32, side: Side, top: bool },
	No,
}
impl HoverState {
	pub fn is_top(&self) -> bool { if let Self::Yes { top: true, .. } = self { true } else { false } }
}

// Mark objects as currently being hovered over.
pub fn mouseover_system(
	mouse: Query<&MousePosWorld, (Changed<MousePosWorld>, With<MainCamera>)>,
	mut objects: Query<(Entity, &ObjectData, &GlobalTransform, &mut HoverState), Without<Placing>>,
) {
	let (mut top_order, mut top_entity) = (f32::MAX, None::<Entity>);
	if let Ok(mouse) = mouse.get_single() {
		// info!("Mouse Coords: {}", mouse);
		for (entity, data, transform, mut hover_state) in objects.iter_mut() {
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
				let order = data.size;
				if order <= top_order {
					top_order = order;
					top_entity = Some(entity);
				}
				*hover_state = HoverState::Yes { order, side, top: false };
				
			} else {
				*hover_state = HoverState::No;
			}
		}
		info!("round");
		for (entity, _, _, mut state) in objects.iter_mut() {
			info!("{entity:?}: {:?}", state);
			if Some(entity) == top_entity {
				if let HoverState::Yes { top, .. } = &mut *state {
					*top = true;
				}
				break
			}
		}
		
	}
}
