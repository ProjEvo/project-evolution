//! Manages the simulation of a [Creature]

use std::{collections::HashMap, time::Duration};

use rapier::{na::Vector2, prelude::*};
use uuid::Uuid;

use crate::{creature::Creature, util};

pub const STEPS_PER_SECOND: i32 = 60;
pub const STEPS_FREQUENCY: Duration = Duration::from_nanos(1_000_000_000 / STEPS_PER_SECOND as u64);
pub const WORLD_X_SIZE: f32 = 1000.0;
pub const WORLD_Y_SIZE: f32 = 560.0;
pub const FLOOR_HEIGHT: f32 = WORLD_Y_SIZE * 0.1;
pub const FLOOR_TOP_Y: f32 = WORLD_Y_SIZE - FLOOR_HEIGHT;
pub const SCORE_PER_SCREEN: i32 = 10; // The score range displayed on the screen (x segments)

const GRAVITY: f32 = 200.0;
const SCORE_SCALE_FACTOR: f32 = SCORE_PER_SCREEN as f32 / WORLD_X_SIZE;
// Muscle extension and contraction range, where 0.0 is normal, -1.0 is maximum contraction, and 1.0 is double extension
const MAX_MUSCLE_CONTRACTION: f32 = -0.5;
const MAX_MUSCLE_EXTENSION: f32 = 0.5;
const MUSCLE_LIMIT_FLUX: f32 = 1.15; // The percentage range muscles can go over max extension (1.15 = 15% over)
const MUSCLE_STIFFNESS: f32 = 5.0; // How stiff the muscles are

/// A simulation of a [Creature], using physics
pub struct Simulation {
    physics_pipeline: PhysicsPipeline,
    physics_pipeline_parameters: PhysicsPipelineParameters,
    creature: Creature,
    node_id_to_rigid_body_handles: HashMap<Uuid, RigidBodyHandle>,
    joint_handles_to_muscle_ids: HashMap<ImpulseJointHandle, Uuid>,
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
        let floor = RigidBodyBuilder::fixed()
            .translation(vector![0.0, WORLD_Y_SIZE])
            .build();
        let floor_handle = rigid_body_set.insert(floor);

        let floor_collider = ColliderBuilder::cuboid(f32::MAX, FLOOR_HEIGHT)
            .collision_groups(InteractionGroups {
                memberships: Group::GROUP_1,
                filter: Group::ALL,
            })
            .build();

        collider_set.insert_with_parent(floor_collider, floor_handle, rigid_body_set);

        // Add creature
        let nodes = creature.nodes();
        let muscles = creature.muscles();
        let muscle_id_to_movement_parameters = creature.movement_parameters();

        let mut node_id_to_rigid_body_handles = HashMap::new();
        let mut joint_handles_to_muscle_ids = HashMap::new();

        // Add node rigid bodies
        for node in nodes.values() {
            let body = RigidBodyBuilder::dynamic()
                .translation(vector![node.position.x, node.position.y])
                .build();

            let body_handle = rigid_body_set.insert(body);
            node_id_to_rigid_body_handles.insert(node.id, body_handle);

            let collider = ColliderBuilder::ball(node.size / 2.0)
                .collision_groups(InteractionGroups {
                    memberships: Group::GROUP_2,
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
            let from_node_body_handle = node_id_to_rigid_body_handles.get(&muscle.from_id).unwrap();
            let to_node_body_handle = node_id_to_rigid_body_handles.get(&muscle.to_id).unwrap();
            let movement_parameters = muscle_id_to_movement_parameters.get(id).unwrap();

            let offset = point![
                to_node_position.x - from_node_position.x,
                to_node_position.y - from_node_position.y
            ];

            let rotate_body_from = RigidBodyBuilder::dynamic()
                .translation(vector![from_node_position.x, from_node_position.y])
                .build();

            let rotate_body_from_handle = rigid_body_set.insert(rotate_body_from);

            let collider_from = ColliderBuilder::ball(1.0)
                .collision_groups(InteractionGroups {
                    memberships: Group::NONE,
                    filter: Group::NONE,
                })
                .build();

            collider_set.insert_with_parent(collider_from, rotate_body_from_handle, rigid_body_set);

            let from_joint = RevoluteJointBuilder::new().build();

            impulse_joint_set.insert(
                *from_node_body_handle,
                rotate_body_from_handle,
                from_joint,
                true,
            );

            let rotate_body_to = RigidBodyBuilder::dynamic()
                .translation(vector![to_node_position.x, to_node_position.y])
                .build();

            let rotate_body_to_handle = rigid_body_set.insert(rotate_body_to);

            let collider_to = ColliderBuilder::ball(1.0)
                .collision_groups(InteractionGroups {
                    memberships: Group::NONE,
                    filter: Group::NONE,
                })
                .build();

            collider_set.insert_with_parent(collider_to, rotate_body_to_handle, rigid_body_set);

            let to_joint = RevoluteJointBuilder::new().build();

            impulse_joint_set.insert(*to_node_body_handle, rotate_body_to_handle, to_joint, true);

            let joint_length = movement_parameters.muscle_length();
            let joint =
                PrismaticJointBuilder::new(UnitVector::new_normalize(vector![offset.x, offset.y]))
                    .local_anchor1(offset)
                    .local_anchor2(point![0.0, 0.0])
                    .set_motor(0.0, 0.0, 0.0, 0.0)
                    .limits([
                        joint_length * MUSCLE_LIMIT_FLUX * MAX_MUSCLE_CONTRACTION,
                        joint_length * MUSCLE_LIMIT_FLUX * MAX_MUSCLE_EXTENSION,
                    ])
                    .build();

            let joint_handle =
                impulse_joint_set.insert(*from_node_body_handle, *to_node_body_handle, joint, true);

            joint_handles_to_muscle_ids.insert(joint_handle, *id);
        }

        // Build simulation
        let physics_pipeline = PhysicsPipeline::new();

        Simulation {
            physics_pipeline,
            physics_pipeline_parameters,
            creature,
            node_id_to_rigid_body_handles,
            joint_handles_to_muscle_ids,
            steps: 0,
        }
    }

    /// Gets the [Creature] being simulated
    pub fn creature(&self) -> &Creature {
        &self.creature
    }

    /// Gets the position of the node by it's id
    pub fn get_position_of_node(&self, id: Uuid) -> Vector<f32> {
        *self
            .physics_pipeline_parameters
            .rigid_body_set
            .get(*self.node_id_to_rigid_body_handles.get(&id).unwrap())
            .unwrap()
            .translation()
    }

    /// Gets the extension delta of a node by it's id
    pub fn is_muscle_extending(&self, id: Uuid) -> bool {
        self.creature
            .movement_parameters()
            .get(&id)
            .unwrap()
            .is_extending(self.steps)
    }

    /// Gets the bounds of the [Creature] in the form (top_left, bottom_right)
    pub fn get_bounds(&self) -> (Vector2<f32>, Vector2<f32>) {
        let bodies = self.node_id_to_rigid_body_handles.values().map(|handle| {
            self.physics_pipeline_parameters
                .rigid_body_set
                .get(*handle)
                .unwrap()
        });
        let x_pos_iter = bodies.clone().map(|body| body.translation().x);
        let y_pos_iter = bodies.map(|body| body.translation().y);

        let x_min = x_pos_iter.clone().min_by(util::cmp_f32).unwrap();
        let y_min = y_pos_iter.clone().min_by(util::cmp_f32).unwrap();
        let x_max = x_pos_iter.max_by(util::cmp_f32).unwrap();
        let y_max = y_pos_iter.max_by(util::cmp_f32).unwrap();

        (Vector2::new(x_min, y_min), Vector2::new(x_max, y_max))
    }

    /// Gets the lowest position to safely display text above
    pub fn get_text_position(&self) -> Vector2<f32> {
        let (top_left, bottom_right) = self.get_bounds();

        Vector2::new((top_left.x + bottom_right.x) / 2.0, top_left.y)
    }

    /// Converts a x position to a score
    pub fn x_to_score(x: f32) -> f32 {
        (x - (WORLD_X_SIZE / 2.0)) * SCORE_SCALE_FACTOR
    }

    /// Converts a score to a x position
    pub fn score_to_x(score: f32) -> f32 {
        (score / SCORE_SCALE_FACTOR) + (WORLD_X_SIZE / 2.0)
    }

    /// Gets the score (furthest x distance) of this simulation
    pub fn get_score(&self) -> f32 {
        let (_, bottom_right) = self.get_bounds();

        Self::x_to_score(bottom_right.x)
    }

    /// Steps the muscles one step forward in time
    fn step_muscles(&mut self) {
        let physics_parameters = &mut self.physics_pipeline_parameters;

        for (handle, joint) in physics_parameters.impulse_joint_set.iter_mut() {
            if let Some(muscle_id) = self.joint_handles_to_muscle_ids.get(&handle) {
                let movement_parameters =
                    self.creature.movement_parameters().get(muscle_id).unwrap();
                let muscle_length = movement_parameters.muscle_length();

                let extension_delta = movement_parameters.get_extension_at(self.steps);
                let extension = MAX_MUSCLE_CONTRACTION
                    + (MAX_MUSCLE_EXTENSION - MAX_MUSCLE_CONTRACTION) * extension_delta;

                let motor = joint.data.as_prismatic_mut().unwrap();
                motor.set_motor_position(extension * muscle_length, MUSCLE_STIFFNESS, 0.5);
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
