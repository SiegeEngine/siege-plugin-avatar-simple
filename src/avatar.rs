
use std::time::Instant;
use movement::{Movement, MoveDirection};
use placement::Placement;

/// This represents the avatar, including its position and movement in the world
#[derive(Debug)]
pub struct Avatar {
    /// The place (and orientation) of the avatar
    pub placement: Placement,
    /// The movement that the avatar was undergoing, starting from the placement at timestamp
    pub movement: Movement,
    /// The time when the avatar was precisely as specified in the placement
    pub timestamp: Instant,
}

impl Avatar {
    pub fn new(placement: Placement,
               movement: Movement,
               timestamp: Instant)
               -> Avatar
    {
        Avatar {
            placement: placement,
            movement: movement,
            timestamp: timestamp,
        }
    }

    // Update placement to the current (or optional other instant) placement.
    pub fn update(&mut self, at: Option<Instant>) {
        let at = match at {
            Some(at) => at,
            None => Instant::now()
        };
        let deltatime = at.duration_since(self.timestamp);
        self.placement = self.placement.extrapolate(&self.movement, deltatime);
        self.timestamp = at;
    }

    pub fn movement_cmd(&mut self,
                        direction: MoveDirection,
                        positive: bool,
                        timestamp: Instant)
    {
        // Update to the given timestamp
        self.update(Some(timestamp));

        // Update the movement
        if positive { self.movement.set(direction); }
        else { self.movement.unset(direction); }
    }

    pub fn get_current_placement(&self) -> Placement
    {
        // extrapolate avatar to now
        let deltatime = Instant::now().duration_since(self.timestamp);
        self.placement.extrapolate(&self.movement, deltatime)
    }
}
