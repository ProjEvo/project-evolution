use rapier::prelude::*;

use crate::creature::Creature;

pub const MAX_WORLD_X: f32 = 100.0;
pub const MAX_WORLD_Y: f32 = 56.0;
pub const FLOOR_HEIGHT: f32 = MAX_WORLD_Y * 0.1;
pub const FLOOR_TOP_Y: f32 = MAX_WORLD_Y - FLOOR_HEIGHT;
const GRAVITY: f32 = 10.0;

pub struct Simulation {
    physics_pipeline: PhysicsPipeline,
    physics_pipeline_parameters: PhysicsPipelineParameters,
}

impl Simulation {
    pub fn new() -> Simulation {
        // Initialize pipeline params
        let mut physics_pipeline_parameters = PhysicsPipelineParameters {
            gravity: vector![0.0, GRAVITY],
            integration_parameters: IntegrationParameters::default(),
            islands: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        };

        // Add floor
        let floor_collider = ColliderBuilder::cuboid(f32::MAX, FLOOR_HEIGHT)
            .translation(vector![0.0, FLOOR_TOP_Y + (FLOOR_HEIGHT / 2.0)])
            .build();

        physics_pipeline_parameters
            .collider_set
            .insert(floor_collider);

        // Build simulation
        let physics_pipeline = PhysicsPipeline::new();

        Simulation {
            physics_pipeline,
            physics_pipeline_parameters,
        }
    }

    pub fn rigid_body_set(&self) -> &RigidBodySet {
        &self.physics_pipeline_parameters.rigid_body_set
    }

    pub fn collider_set(&self) -> &ColliderSet {
        &self.physics_pipeline_parameters.collider_set
    }

    pub fn add_creature(&mut self, creature: &Creature) {
        let physics_pipeline_parameters = &mut self.physics_pipeline_parameters;
        let rigid_body_set = &mut physics_pipeline_parameters.rigid_body_set;
        let collider_set = &mut physics_pipeline_parameters.collider_set;

        for node in creature.nodes().values() {
            let body = RigidBodyBuilder::dynamic()
                .translation(vector![node.position.x, node.position.y])
                .build();

            let body_handle = rigid_body_set.insert(body);

            let collider = ColliderBuilder::ball(node.size / 2.0)
                .restitution(0.7)
                .build();

            collider_set.insert_with_parent(collider, body_handle, rigid_body_set);
        }
    }

    pub fn step(&mut self) {
        let params = &mut self.physics_pipeline_parameters;

        let physics_hooks = ();
        let events_handler = ();

        self.physics_pipeline.step(
            &params.gravity,
            &params.integration_parameters,
            &mut params.islands,
            &mut params.broad_phase,
            &mut params.narrow_phase,
            &mut params.rigid_body_set,
            &mut params.collider_set,
            &mut params.impulse_joints,
            &mut params.multibody_joints,
            &mut params.ccd_solver,
            &physics_hooks,
            &events_handler,
        );
    }
}

struct PhysicsPipelineParameters {
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    islands: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
}
