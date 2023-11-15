#[derive(Clone, Debug, PartialEq)]
pub struct Camp {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
}

pub enum CampKind {
    Small,
    Medium,
    Large,
}

impl Camp {
    pub fn new(x: i32, y: i32, kind: CampKind) -> Self {
        Self { x, y, radius: 0 }
    }
}
