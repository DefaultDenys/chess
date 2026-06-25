use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Selection {
    pub selected: Option<(i32, i32)>,
    pub press_origin: Option<Vec2>,
    pub dragging: bool,
}

impl Selection {
    pub fn clear(&mut self) {
        self.selected = None;
        self.press_origin = None;
        self.dragging = false;
    }
}
