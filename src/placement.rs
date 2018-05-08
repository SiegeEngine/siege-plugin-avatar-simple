
use siege_math::{Vec3, Point3};
use std::f32::consts::{FRAC_PI_2, PI};
use std::time::Duration;
use movement::Movement;

const MAX_PITCH: f32 = FRAC_PI_2;
const MIN_PITCH: f32 = -FRAC_PI_2;

const LINEAR_SPEED_MPS: f32 = 15.0;
const ROT_SPEED_RPS: f32 = 1.6;

#[derive(Debug, Clone)]
pub struct Placement {
    /// The cartesian coordinates of the center of the camera
    pub position: Point3<f32>,
    /// Pitch angle in radians from -pi/2 (straight down) to +pi/2 (straight up) that
    /// the camera is facing
    pub pitch: f32,
    /// Yaw angle in radians from 0 (+Z) to 2*pi (+Z again), rotating CCW
    /// that the camera is facing
    pub yaw: f32,
}

impl Placement {
    pub fn new(position: Point3<f32>, pitch: f32, yaw: f32) -> Placement {
        let pitch = if pitch > MAX_PITCH { MAX_PITCH }
        else if pitch < MIN_PITCH { MIN_PITCH }
        else { pitch };

        let yaw = yaw % (2.0 * PI);

        Placement {
            position: position,
            pitch: pitch,
            yaw: yaw
        }
    }

    /*
    pub fn set_position(&mut self, position: Point3<f32>) {
        self.position = position;
    }
    pub fn get_position(&self) -> Point3<f32> {
        self.position
    }
    pub fn set_pitch(&mut self, pitch: f32) {
        let pitch = if pitch > MAX_PITCH { MAX_PITCH }
        else if pitch < MIN_PITCH { MIN_PITCH }
        else { pitch };
        self.pitch = pitch;
    }
    pub fn get_pitch(&self) -> f32 {
        self.pitch
    }
    pub fn set_yaw(&mut self, yaw: f32) {
        let yaw = yaw % (2.0 * PI);
        self.yaw = yaw;
    }
    pub fn get_yaw(&self) -> f32 {
        self.yaw
    }
     */

    pub fn extrapolate(&self, movement: &Movement, time: Duration) -> Placement
    {
        let secs = time.as_secs() as f32
            + (time.subsec_nanos() as f32 * 0.000000001);

        Placement {
            position: {
                let fwd_distance = movement.linear() * secs * LINEAR_SPEED_MPS;
                let sid_distance = movement.strafe() * secs * LINEAR_SPEED_MPS;
                self.position
                    + Vec3::new(-self.yaw.sin() * fwd_distance, 0.0, self.yaw.cos() * fwd_distance)
                    + Vec3::new( self.yaw.cos() * sid_distance, 0.0, self.yaw.sin() * sid_distance)
            },
            pitch: {
                let pitch = self.pitch + movement.pitch() * secs * ROT_SPEED_RPS;
                if pitch > MAX_PITCH { MAX_PITCH }
                else if pitch < MIN_PITCH { MIN_PITCH }
                else { pitch }
            },
            yaw: {
                let yaw = self.yaw + movement.yaw() * secs * ROT_SPEED_RPS;
                yaw % (2.0 * PI)
            },
        }
    }
}
