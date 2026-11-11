use glam::Vec3;
use oxijolt::prelude::{
    AllowedDofs, BodyId, BodySettings, PhysicsWorld, Shape, WorldSettings,
    math::{RVec3, Vec3 as PhysicsVec3},
};

const FLOOR_HALF_EXTENT: f32 = 50.0;
const PLAYER_RADIUS: f32 = 0.35;
const PLAYER_CYLINDER_HALF_HEIGHT: f32 = 0.55;
const PLAYER_GROUND_HEIGHT: f32 = PLAYER_RADIUS + PLAYER_CYLINDER_HALF_HEIGHT;

/// Jolt-backed rigid-body movement, gravity, jumping, and floor collision for the player.
pub struct FirstPersonPhysics {
    world: PhysicsWorld,
    player: BodyId,
}

impl FirstPersonPhysics {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut world = PhysicsWorld::new(WorldSettings::default())?;

        let floor_shape =
            Shape::new_box(PhysicsVec3::new(FLOOR_HALF_EXTENT, 0.5, FLOOR_HALF_EXTENT))?;
        world.create_body(
            &floor_shape,
            &BodySettings::new_static().position(RVec3::new(0.0, -0.5, 0.0)),
        )?;

        let player_shape = Shape::new_capsule(PLAYER_CYLINDER_HALF_HEIGHT, PLAYER_RADIUS)?;
        let player = world.create_body(
            &player_shape,
            &BodySettings::new_dynamic()
                .position(RVec3::new(0.0, 1.1, 8.0))
                .allowed_dofs(
                    AllowedDofs::TRANSLATION_X
                        | AllowedDofs::TRANSLATION_Y
                        | AllowedDofs::TRANSLATION_Z,
                )
                .friction(0.0),
        )?;

        Ok(Self { world, player })
    }

    /// Sets horizontal movement speed while preserving Jolt's vertical velocity and gravity.
    pub fn set_horizontal_velocity(
        &mut self,
        velocity: Vec3,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !velocity.is_finite() {
            return Err(std::io::Error::other("Player velocity must be finite").into());
        }

        let vertical_velocity = self.world.body(self.player)?.linear_velocity().y;
        self.world
            .body_mut(self.player)?
            .set_linear_velocity(PhysicsVec3::new(velocity.x, vertical_velocity, velocity.z))?;
        Ok(())
    }

    /// Jumps only while the player is touching the floor.
    pub fn jump(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let body = self.world.body(self.player)?;
        let position = body.position();
        let velocity = body.linear_velocity();
        if position.y as f32 > PLAYER_GROUND_HEIGHT + 0.05 || velocity.y > 0.1 {
            return Ok(false);
        }

        self.world
            .body_mut(self.player)?
            .set_linear_velocity(PhysicsVec3::new(velocity.x, 6.0, velocity.z))?;
        Ok(true)
    }

    pub fn step(&mut self, delta_time: f32) -> Result<(), Box<dyn std::error::Error>> {
        let report = self.world.step(delta_time)?;
        if !report.is_complete() {
            return Err(std::io::Error::other("Jolt reported an incomplete physics step").into());
        }
        Ok(())
    }

    /// Position of the capsule center in world space.
    pub fn player_position(&self) -> Result<Vec3, Box<dyn std::error::Error>> {
        let position = self.world.body(self.player)?.position();
        Ok(Vec3::new(
            position.x as f32,
            position.y as f32,
            position.z as f32,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::FirstPersonPhysics;
    use glam::Vec3;

    #[test]
    fn gravity_falls_and_the_floor_stops_the_player() {
        let mut physics = FirstPersonPhysics::new().expect("create Jolt world");
        let start_y = physics.player_position().expect("read player position").y;

        for _ in 0..180 {
            physics.step(1.0 / 60.0).expect("step Jolt world");
        }

        let landed_position = physics.player_position().expect("read landed position");
        assert!(landed_position.y < start_y - 0.1);
        assert!((landed_position.y - 0.9).abs() < 0.15);

        physics
            .set_horizontal_velocity(Vec3::new(2.0, 50.0, 0.0))
            .expect("set horizontal velocity");
        let vertical_velocity = physics
            .world
            .body(physics.player)
            .expect("read player body")
            .linear_velocity()
            .y;
        assert!(vertical_velocity.abs() < 0.2);
    }

    #[test]
    fn jump_is_grounded_and_gravity_brings_the_player_back_down() {
        let mut physics = FirstPersonPhysics::new().expect("create Jolt world");
        for _ in 0..180 {
            physics.step(1.0 / 60.0).expect("land on floor");
        }

        assert!(physics.jump().expect("jump from floor"));
        let jump_start = physics.player_position().expect("read jump position").y;
        physics.step(1.0 / 60.0).expect("step jump");
        assert!(
            physics.player_position().expect("read rising position").y > jump_start,
            "jump should move the player upward"
        );
        assert!(!physics.jump().expect("reject midair jump"));

        for _ in 0..120 {
            physics.step(1.0 / 60.0).expect("land after jump");
        }
        let landed_position = physics.player_position().expect("read landed position");
        assert!((landed_position.y - 0.9).abs() < 0.15);
        assert!(physics.jump().expect("jump after landing"));
    }
}
