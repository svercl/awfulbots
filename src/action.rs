use nalgebra::{Point2, Vector2};

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

pub struct Action {
    first_click: Vector2<f64>,
    first_click_world: Point2<f64>,
    second_click: Vector2<f64>,
    second_click_world: Point2<f64>,
    kind: ActionKind,
    // TODO: Use `Step` enum instead of relying on numbers
    step: usize,
    first_body: Option<usize>,
    second_body: Option<usize>,
}

impl Action {
    pub fn reset(&mut self) {
        log::trace!("Byebye action");
        self.step = 0;
        self.kind = ActionKind::None;
        self.first_click = nalgebra::zero();
        self.first_click_world = Point2::origin();
        self.second_click = nalgebra::zero();
        self.second_click_world = Point2::origin();
    }

    pub fn kind(&self) -> ActionKind {
        self.kind
    }

    pub fn set_kind(&mut self, kind: ActionKind) -> &mut Self {
        self.kind = kind;
        self
    }

    pub fn step(&self) -> usize {
        self.step
    }

    pub fn advance_step(&mut self) {
        self.step += 1;
    }

    pub fn first_click(&self) -> Vector2<f64> {
        self.first_click
    }

    pub fn set_first_click(&mut self, first_click: Vector2<f64>) -> &mut Self {
        self.first_click = first_click;
        self
    }

    pub fn first_click_world(&self) -> Point2<f64> {
        self.first_click_world
    }

    pub fn set_first_click_world(&mut self, first_click_world: Point2<f64>) -> &mut Self {
        self.first_click_world = first_click_world;
        self
    }

    pub fn second_click(&self) -> Vector2<f64> {
        self.second_click
    }

    pub fn set_second_click(&mut self, second_click: Vector2<f64>) -> &mut Self {
        self.second_click = second_click;
        self
    }

    pub fn second_click_world(&self) -> Point2<f64> {
        self.second_click_world
    }

    pub fn set_second_click_world(&mut self, second_click_world: Point2<f64>) -> &mut Self {
        self.second_click_world = second_click_world;
        self
    }
}

impl Default for Action {
    fn default() -> Self {
        Action {
            first_click: nalgebra::zero(),
            first_click_world: Point2::origin(),
            second_click: nalgebra::zero(),
            second_click_world: Point2::origin(),
            kind: ActionKind::None,
            step: 0,
            first_body: None,
            second_body: None,
        }
    }
}
