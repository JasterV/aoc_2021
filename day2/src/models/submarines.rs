pub trait Submarine {
    fn forward(self, unit: i32) -> Self;
    fn down(self, unit: i32) -> Self;
    fn up(self, unit: i32) -> Self;
    fn position(&self) -> i32;
    fn depth(&self) -> i32;
    fn position_x_depth(&self) -> i32 {
        self.position() * self.depth()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SubmarineV1 {
    pub xcord: i32,
    pub depth: i32,
}

impl SubmarineV1 {
    pub fn new() -> Self {
        SubmarineV1 { xcord: 0, depth: 0 }
    }
}

impl Submarine for SubmarineV1 {
    fn forward(self, unit: i32) -> Self {
        SubmarineV1 {
            xcord: self.xcord + unit,
            depth: self.depth,
        }
    }

    fn down(self, unit: i32) -> Self {
        SubmarineV1 {
            xcord: self.xcord,
            depth: self.depth + unit,
        }
    }

    fn up(self, unit: i32) -> Self {
        SubmarineV1 {
            xcord: self.xcord,
            depth: self.depth - unit,
        }
    }

    fn position(&self) -> i32 {
        self.xcord
    }

    fn depth(&self) -> i32 {
        self.depth
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SubmarineV2 {
    xcord: i32,
    depth: i32,
    aim: i32,
}

impl SubmarineV2 {
    pub fn new() -> Self {
        Self {
            xcord: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl Submarine for SubmarineV2 {
    fn forward(self, unit: i32) -> Self {
        SubmarineV2 {
            xcord: self.xcord + unit,
            depth: self.depth + self.aim * unit,
            aim: self.aim,
        }
    }

    fn down(self, unit: i32) -> Self {
        SubmarineV2 {
            xcord: self.xcord,
            depth: self.depth,
            aim: self.aim + unit,
        }
    }

    fn up(self, unit: i32) -> Self {
        SubmarineV2 {
            xcord: self.xcord,
            depth: self.depth,
            aim: self.aim - unit,
        }
    }

    fn position(&self) -> i32 {
        self.xcord
    }

    fn depth(&self) -> i32 {
        self.depth
    }
}
