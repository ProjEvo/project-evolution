//! Manages the simulations of [Creature]s

use std::collections::HashMap;

use rapier::prelude::*;

use crate::creature::{Creature, MovementParameters};

pub const STEPS_PER_SECOND: i32 = 60;
pub const MAX_WORLD_X: f32 = 100.0;
pub const MAX_WORLD_Y: f32 = 56.0;
pub const FLOOR_HEIGHT: f32 = MAX_WORLD_Y * 0.1;
pub const FLOOR_TOP_Y: f32 = MAX_WORLD_Y - FLOOR_HEIGHT;
const GRAVITY: f32 = 10.0;
// Muscle extension and contraction range, where 0.0 is normal, -1.0 is maximum contraction, and 1.0 is double extension
const MAX_MUSCLE_CONTRACTION: f32 = -0.5;
const MAX_MUSCLE_EXTENSION: f32 = 0.5;
const MUSCLE_STIFFNESS: f32 = 500.0;

/// A simulation of a [Creature], using physics
pub struct Simulation {
    physics_pipeline: PhysicsPipeline,
    physics_pipeline_parameters: PhysicsPipelineParameters,
    joint_handles_to_movement_parameters: HashMap<ImpulseJointHandle, MovementParameters>,
    steps: i32,
}

impl Simulation {
    /// Creates a simulation of a [Creature]
    pub fn new(creature: Creature) -> Simulation {
        // Initialize pipeline params
        let mut physics_pipeline_parameters = PhysicsPipelineParameters {
            gravity: vector![0.0, GRAVITY],
            integration_parameters: IntegrationParameters::default(),
            islands: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joints_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        };
        let rigid_body_set = &mut physics_pipeline_parameters.rigid_body_set;
        let collider_set = &mut physics_pipeline_parameters.collider_set;
        let impulse_joint_set = &mut physics_pipeline_parameters.impulse_joint_set;

        // Add floor
        let floor_collider = ColliderBuilder::cuboid(f32::MAX, FLOOR_HEIGHT)
            .translation(vector![0.0, FLOOR_TOP_Y + (FLOOR_HEIGHT / 2.0)])
            .collision_groups(InteractionGroups {
                memberships: Group::GROUP_1,
                filter: Group::ALL,
            })
            .build();

        collider_set.insert(floor_collider);

        // Add creature
        let nodes = creature.nodes();
        let muscles = creature.muscles();
        let muscle_id_to_movement_parameters = creature.movement_parameters();

        let mut node_id_to_body_handles = HashMap::new();
        let mut joint_handles_to_movement_parameters = HashMap::new();

        // Add node rigid bodies
        for node in nodes.values() {
            let body = RigidBodyBuilder::dynamic()
                .translation(vector![node.position.x, node.position.y])
                .build();

            let body_handle = rigid_body_set.insert(body);
            node_id_to_body_handles.insert(node.id, body_handle);

            let collider = ColliderBuilder::ball(node.size / 2.0)
                .collision_groups(InteractionGroups {
                    memberships: Group::GROUP_2.union(Group::GROUP_1),
                    filter: Group::GROUP_1,
                })
                .restitution(0.7)
                .build();

            collider_set.insert_with_parent(collider, body_handle, rigid_body_set);
        }

        // Add muscle joints
        for (id, muscle) in muscles {
            let from_node_position = &nodes.get(&muscle.from_id).unwrap().position;
            let to_node_position = &nodes.get(&muscle.to_id).unwrap().position;
            let from_node_body_handle = node_id_to_body_handles.get(&muscle.from_id).unwrap();
            let to_node_body_handle = node_id_to_body_handles.get(&muscle.to_id).unwrap();

            let offset = point![
                to_node_position.x - from_node_position.x,
                to_node_position.y - from_node_position.y
            ];

            let joint =
                PrismaticJointBuilder::new(UnitVector::new_normalize(vector![offset.x, offset.y]))
                    .local_anchor1(offset)
                    .local_anchor2(point![0.0, 0.0])
                    .set_motor(0.0, 0.0, 0.0, 0.0)
                    .build();

            let joint_handle =
                impulse_joint_set.insert(*from_node_body_handle, *to_node_body_handle, joint, true);

            joint_handles_to_movement_parameters.insert(
                joint_handle,
                muscle_id_to_movement_parameters.get(id).unwrap().clone(),
            );
        }

        // Build simulation
        let physics_pipeline = PhysicsPipeline::new();

        Simulation {
            physics_pipeline,
            physics_pipeline_parameters,
            joint_handles_to_movement_parameters,
            steps: 0,
        }
    }

    /// Gets the [RigidBodySet]
    pub fn rigid_body_set(&self) -> &RigidBodySet {
        &self.physics_pipeline_parameters.rigid_body_set
    }

    /// Gets the [ColliderSet]
    pub fn collider_set(&self) -> &ColliderSet {
        &self.physics_pipeline_parameters.collider_set
    }

    /// Gets the [ImpulseJointSet]
    pub fn impulse_joint_set(&self) -> &ImpulseJointSet {
        &self.physics_pipeline_parameters.impulse_joint_set
    }

    /// Steps the muscles one step forward in time
    fn step_muscles(&mut self) {
        let physics_parameters = &mut self.physics_pipeline_parameters;

        for (handle, joint) in physics_parameters.impulse_joint_set.iter_mut() {
            if let Some(movement_parameters) =
                self.joint_handles_to_movement_parameters.get(&handle)
            {
                let muscle_length = movement_parameters.muscle_length();

                let extension_delta = movement_parameters.get_extension_at(self.steps);
                let extension = MAX_MUSCLE_CONTRACTION
                    + (MAX_MUSCLE_EXTENSION - MAX_MUSCLE_CONTRACTION) * extension_delta;

                let motor = joint.data.as_prismatic_mut().unwrap();
                motor.set_motor(extension * muscle_length, 0.1, MUSCLE_STIFFNESS, 0.5);
            }
        }
    }

    /// Steps the simulation one step forward in time
    pub fn step(&mut self) {
        self.step_muscles();

        let params = &mut self.physics_pipeline_parameters;

        let physics_hooks = &();
        let events_handler = &();

        self.physics_pipeline.step(
            &params.gravity,
            &params.integration_parameters,
            &mut params.islands,
            &mut params.broad_phase,
            &mut params.narrow_phase,
            &mut params.rigid_body_set,
            &mut params.collider_set,
            &mut params.impulse_joint_set,
            &mut params.multibody_joints_set,
            &mut params.ccd_solver,
            physics_hooks,
            events_handler,
        );
        self.steps += 1;
    }
}

/// A struct to store all the parameters for the [PhysicsPipeline]
struct PhysicsPipelineParameters {
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    islands: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joints_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
}
