use nalgebra::Point2;
use ncollide2d::world::CollisionGroups;
use nphysics2d::object::{BodyPartHandle, ColliderAnchor};
use nphysics2d::world::World;

// this will return a body (if any) at the current point
pub fn get_body_at_mouse(world: &World<f64>, point: &Point2<f64>) -> Option<BodyPartHandle> {
    let groups = CollisionGroups::default();
    for b in world
        .collider_world()
        .interferences_with_point(point, &groups)
    {
        if !b.query_type().is_proximity_query() && !b.body().is_ground() {
            if let ColliderAnchor::OnBodyPart { body_part, .. } = b.anchor() {
                return Some(*body_part);
            } else {
                continue;
            }
        }
    }
    None
}

pub fn clamp<T>(val: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
