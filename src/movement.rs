
/// These represent the directions the avatar can be moving.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveDirection {
    Forward = 0,
    Backward = 1,
    YawLeft = 2,
    YawRight = 3,
    StrafeLeft = 4,
    StrafeRight = 5,
    PitchUp = 6,
    PitchDown = 7,
}

/// This stores the current movement of the avatar in a bitfield. The Avatar can be
/// moving in multiple directions at once (e.g. YawLeft and Backward) as long
/// as they are not opposite (which stops that degree of movement)
#[derive(Debug, Clone)]
pub struct Movement(pub u8);

impl Movement {
    pub fn new() -> Movement {
        Movement(0)
    }

    /*
    pub fn is_nil(&self) -> bool {
        ((self.0 & (0b1 << 0)) >> 0 ) == ((self.0 & (0b1 << 1)) >> 1) &&
            ((self.0 & (0b1 << 2)) >> 2 ) == ((self.0 & (0b1 << 3)) >> 3) &&
            ((self.0 & (0b1 << 4)) >> 4 ) == ((self.0 & (0b1 << 5)) >> 5) &&
            ((self.0 & (0b1 << 6)) >> 6 ) == ((self.0 & (0b1 << 7)) >> 7)
    }
     */

    /// Turn on a movement (e.g. press movement key might do this)
    pub fn set(&mut self, direction: MoveDirection) {
        self.0 = self.0 | (1 << (direction as usize))
    }

    /// Turn off a movement (e.g. release movement key might do this)
    pub fn unset(&mut self, direction: MoveDirection) {
        self.0 = self.0 & !(1 << (direction as usize))
    }

    /// Get the forward/backward movement as 1.0 (forward) 0.0 (stopped) or -1.0 (backward)
    pub fn linear(&self) -> f32 {
        match self.0 & 0b11 {
            0b00 => 0.0,
            0b01 => 1.0,
            0b10 => -1.0,
            0b11 => 0.0,
            _ => unreachable!(),
        }
    }

    /// Get the rotational movement as 1.0 (anticlockwise) 0.0 (stopped) or -1.0 (clockwise)
    pub fn yaw(&self) -> f32 {
        match (self.0 >> 2) & 0b11 {
            0b00 => 0.0,
            0b01 => 1.0,
            0b10 => -1.0,
            0b11 => 0.0,
            _ => unreachable!(),
        }
    }

    /// Get the side-to-side movement as 1.0 (rightwards) 0.0 (stopped) or -1.0 (leftwards)
    pub fn strafe(&self) -> f32 {
        match (self.0 >> 4) & 0b11 {
            0b00 => 0.0,
            0b01 => -1.0,
            0b10 => 1.0,
            0b11 => 0.0,
            _ => unreachable!(),
        }
    }

    /// Get the rotational pitch movement as 1.0 (upwards) 0.0 (stopped) or -1.0 (downwards)
    pub fn pitch(&self) -> f32 {
        match (self.0 >> 6) & 0b11 {
            0b00 => 0.0,
            0b01 => 1.0,
            0b10 => -1.0,
            0b11 => 0.0,
            _ => unreachable!(),
        }
    }
}
