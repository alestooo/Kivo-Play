use super::{DiePoseResponse, PhysicsDieInput, PhysicsFrameResponse, PhysicsSimulation};

use rapier2d::prelude::*;

use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_3, PI, TAU},
};

const SIMULATION_STEPS: usize = 120;
const FRAME_INTERVAL: usize = 4;

const ARENA_RADIUS: f32 = 5.4;
const DIE_HALF_SIZE: f32 = 0.42;

pub fn simulate_roll(
    dice: &[PhysicsDieInput],
    owner_id: uuid::Uuid,
    die_index: usize,
) -> PhysicsSimulation {
    let mut pipeline = PhysicsPipeline::new();

    let gravity = Vector::new(0.0, 0.0);

    let mut integration = IntegrationParameters::default();
    integration.dt = 1.0 / 60.0;

    let mut islands = IslandManager::new();
    let mut broad_phase = BroadPhaseBvh::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut impulse_joints = ImpulseJointSet::new();
    let mut multibody_joints = MultibodyJointSet::new();
    let mut soft_bodies = SoftBodySet::new();
    let mut ccd_solver = CCDSolver::new();

    create_arena(&mut colliders);

    let mut handles: HashMap<RigidBodyHandle, (uuid::Uuid, usize)> = HashMap::new();

    for die in dice {
        if !die.state.rolled {
            continue;
        }

        let body = RigidBodyBuilder::dynamic()
            .translation(Vector::new(die.state.x, die.state.y))
            .rotation(die.state.rotation)
            .linear_damping(1.5)
            .angular_damping(1.25)
            .ccd_enabled(true)
            .build();

        let handle = bodies.insert(body);

        let collider = ColliderBuilder::cuboid(DIE_HALF_SIZE, DIE_HALF_SIZE)
            .restitution(0.72)
            .friction(0.65)
            .density(1.0)
            .build();

        colliders.insert_with_parent(collider, handle, &mut bodies);

        handles.insert(handle, (die.owner_id, die.die_index));
    }

    let spawn = spawn_position(owner_id, die_index);

    let angle = rand::random_range(-PI..PI);
    let velocity_x = rand::random_range(-4.8..4.8);
    let velocity_y = rand::random_range(4.0..8.5);
    let angular_velocity = rand::random_range(-13.0..13.0);

    let new_body = RigidBodyBuilder::dynamic()
        .translation(Vector::new(spawn.0, spawn.1))
        .rotation(angle)
        .linvel(Vector::new(velocity_x, velocity_y))
        .angvel(angular_velocity)
        .linear_damping(1.25)
        .angular_damping(1.1)
        .ccd_enabled(true)
        .build();

    let new_handle = bodies.insert(new_body);

    let new_collider = ColliderBuilder::cuboid(DIE_HALF_SIZE, DIE_HALF_SIZE)
        .restitution(0.78)
        .friction(0.6)
        .density(1.0)
        .build();

    colliders.insert_with_parent(new_collider, new_handle, &mut bodies);

    handles.insert(new_handle, (owner_id, die_index));

    let mut frames = Vec::new();

    for step in 0..SIMULATION_STEPS {
        pipeline.step(
            gravity,
            &integration,
            &mut islands,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut impulse_joints,
            &mut multibody_joints,
            &mut soft_bodies,
            &mut ccd_solver,
            &(),
            &(),
        );

        if step % FRAME_INTERVAL == 0 {
            frames.push(capture_frame(&bodies, &handles));
        }
    }

    frames.push(capture_frame(&bodies, &handles));

    let mut updated = dice.to_vec();

    for (handle, (die_owner, simulated_die_index)) in &handles {
        let Some(body) = bodies.get(*handle) else {
            continue;
        };

        let position = body.translation();
        let rotation = body.rotation().angle();
        let value = rotation_to_value(rotation);

        if let Some(existing) = updated
            .iter_mut()
            .find(|die| die.owner_id == *die_owner && die.die_index == *simulated_die_index)
        {
            existing.state.rolled = true;
            existing.state.x = position.x;
            existing.state.y = position.y;
            existing.state.rotation = rotation;
            existing.state.value = Some(value);
        } else {
            updated.push(PhysicsDieInput {
                owner_id: *die_owner,
                die_index: *simulated_die_index,
                state: super::DieState {
                    rolled: true,
                    x: position.x,
                    y: position.y,
                    rotation,
                    value: Some(value),
                },
            });
        }
    }

    PhysicsSimulation {
        frames,
        dice: updated,
    }
}

fn capture_frame(
    bodies: &RigidBodySet,
    handles: &HashMap<RigidBodyHandle, (uuid::Uuid, usize)>,
) -> PhysicsFrameResponse {
    let mut dice = Vec::with_capacity(handles.len());

    for (handle, (owner_id, die_index)) in handles {
        let Some(body) = bodies.get(*handle) else {
            continue;
        };

        let position = body.translation();

        dice.push(DiePoseResponse {
            owner_id: *owner_id,
            die_index: *die_index,
            x: position.x,
            y: position.y,
            rotation: body.rotation().angle(),
        });
    }

    PhysicsFrameResponse { dice }
}

fn create_arena(colliders: &mut ColliderSet) {
    let vertices = octagon_vertices();

    for index in 0..8 {
        let start = vertices[index];
        let end = vertices[(index + 1) % 8];

        let wall =
            ColliderBuilder::segment(Vector::new(start.0, start.1), Vector::new(end.0, end.1))
                .restitution(0.78)
                .friction(0.55)
                .build();

        colliders.insert(wall);
    }
}

fn octagon_vertices() -> Vec<(f32, f32)> {
    let mut vertices = Vec::with_capacity(8);

    for index in 0..8 {
        let angle = (index as f32 * TAU / 8.0) + PI / 8.0;

        vertices.push((ARENA_RADIUS * angle.cos(), ARENA_RADIUS * angle.sin()));
    }

    vertices
}

fn spawn_position(owner_id: uuid::Uuid, die_index: usize) -> (f32, f32) {
    let player_seed = owner_id.as_u128() % 4;

    let horizontal = match player_seed {
        0 => -2.2,
        1 => -0.75,
        2 => 0.75,
        _ => 2.2,
    };

    let die_offset = match die_index {
        0 => -0.35,
        1 => 0.0,
        _ => 0.35,
    };

    (horizontal + die_offset, -3.5)
}

fn rotation_to_value(rotation: f32) -> u8 {
    let normalized = rotation.rem_euclid(TAU);
    let face = (normalized / FRAC_PI_3).floor() as u8;

    (face % 6) + 1
}
