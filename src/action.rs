// used internally to track state
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionKind {
    None,
    CreatingCircle,
    CreatingRectangle,
    CreatingTriangle,
    CreatingFixedJoint,
    CreatingSlidingJoint,
    CreatingRotatingJoint,
}
