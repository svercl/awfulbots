use nalgebra::Vector2;

// used internally to track state
enum ActionKind {
    CreatingCircle,
    CreatingRectangle,
}

pub struct Action {
    first_click: Vector2<f64>,
    kind: ActionKind,
    // this is used to keep track of the current step
    step: u8,
}

impl Action {
    pub fn creating_circle(first_click: Vector2<f64>) -> Self {
        Action {
            first_click,
            kind: ActionKind::CreatingCircle,
            step: 0,
        }
    }

    pub fn creating_rectangle(first_click: Vector2<f64>) -> Self {
        Action {
            first_click,
            kind: ActionKind::CreatingRectangle,
            step: 0,
        }
    }

    pub fn advance(&mut self) {
        self.step += 1;
    }
}
