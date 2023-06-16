use rapier2d::{prelude::*, na::Vector2};

#[derive(Default)]
pub struct PhysicsData {
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    islands: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    hooks: (),
    pub events: CollisionHandler,
}

#[derive(Default)]
pub struct CollisionHandler {
    pub collisions: Option<Vec<CollisionEvent>>,
}

impl EventHandler for CollisionHandler {
    fn handle_collision_event(
        &self,
        _bodies: &RigidBodySet,
        _colliders: &ColliderSet,
        event: CollisionEvent,
        _contact_pair: Option<&ContactPair>,
    ) {
        let ptr = self as *const CollisionHandler as *mut CollisionHandler;
        unsafe {
            match &mut (*ptr).collisions {
                Some(vec) => vec.push(event),
                None => {
                    (*ptr).collisions = Some(vec![event]);
                }
            }
        }
    }

    fn handle_contact_force_event(
        &self,
        _dt: Real,
        _bodies: &RigidBodySet,
        _colliders: &ColliderSet,
        _contact_pair: &ContactPair,
        _total_force_magnitude: Real,
    ) {
        todo!()
    }
}

impl PhysicsData {
    pub fn get_rb(&mut self, handle: RigidBodyHandle) -> &RigidBody {
        self.bodies.get(handle).unwrap()
    }

    pub fn get_rb_mut(&mut self, handle: RigidBodyHandle) -> &mut RigidBody {
        self.bodies.get_mut(handle).unwrap()
    }

    pub fn get_coll(&mut self, handle: ColliderHandle) -> &Collider {
        self.colliders.get(handle).unwrap()
    }

    pub fn get_coll_mut(&mut self, handle: ColliderHandle) -> &mut Collider {
        self.colliders.get_mut(handle).unwrap()
    }
}

pub fn create_pipeline(rigidbody_set: RigidBodySet, collider_set: ColliderSet) -> PhysicsData {
    PhysicsData {
        gravity: Vector2::new(0.0, 0.0),
        integration_parameters: IntegrationParameters::default(),
        islands: IslandManager::new(),
        broad_phase: BroadPhase::new(),
        narrow_phase: NarrowPhase::new(),
        bodies: rigidbody_set,
        colliders: collider_set,
        impulse_joints: ImpulseJointSet::new(),
        multibody_joints: MultibodyJointSet::new(),
        ccd_solver: CCDSolver::new(),
        hooks: (),
        events: CollisionHandler { collisions: Some(Vec::new()) },
    }
}

pub fn physics_step(physics_pipeline: &mut PhysicsPipeline, data: &mut PhysicsData) {
    physics_pipeline.step(
        &data.gravity,
        &data.integration_parameters,
        &mut data.islands,
        &mut data.broad_phase,
        &mut data.narrow_phase,
        &mut data.bodies,
        &mut data.colliders,
        &mut data.impulse_joints,
        &mut data.multibody_joints,
        &mut data.ccd_solver,
        None,
        &data.hooks,
        &data.events
    );
}

pub fn create_edges(width: f32, height: f32, edge_width: f32, edge_height: f32,
                    rigidbody_set: &mut RigidBodySet, collider_set: &mut ColliderSet) {
    let edge_rb = RigidBodyBuilder::kinematic_position_based()
                .position(Isometry::new(vector![0.0, 0.0], 0.0))
                .ccd_enabled(true)
                .build();
    let edge_rb_handle = rigidbody_set.insert(edge_rb);

    edge(width, edge_height, 0.0, height + edge_height, collider_set, edge_rb_handle, rigidbody_set); // bottom
    edge(width, edge_height, 0.0, -edge_height, collider_set, edge_rb_handle, rigidbody_set); // top
    edge(edge_width, height, -edge_width, 0.0, collider_set, edge_rb_handle, rigidbody_set); // left
    edge(edge_width, height, width + edge_width, 0.0, collider_set, edge_rb_handle, rigidbody_set); // right

    fn edge(sx: f32, sy: f32, x: f32, y: f32,
            collider_set: &mut ColliderSet,
            rb_handle: RigidBodyHandle,
            set: &mut RigidBodySet) {
        let pos = Isometry::new(vector![x, y], 0.0);
        collider_set.insert_with_parent(ColliderBuilder::cuboid(sx, sy).position(pos).build(), rb_handle, set);
    }
}