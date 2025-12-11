use bevy::prelude::*;

// How frequently should physics be updated (in ms)
const PHYSICS_TICK_TIME: u128 = 33;

#[derive(Default)]
pub struct PhysicsTimer(u128);

#[derive(Event)]
pub struct PhysicsTick;

pub fn physics_clock(
    mut clock: Local<PhysicsTimer>,
    time: Res<Time>,
    mut on_tick: EventWriter<PhysicsTick>,
) {
    let ms_since_last_call = time.delta().as_millis();
    clock.0 += ms_since_last_call;
    if clock.0 >= PHYSICS_TICK_TIME {
        clock.0 = 0;
        on_tick.write(PhysicsTick);
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(Vec3::ZERO)
    }
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Velocity(Vec3::new(x, y, z))
    }
}

#[derive(Event)]
pub struct Impulse {
    pub target: Entity,
    pub amount: Vec3,
    pub absolute: bool,
    pub source: i32,
}

pub fn sum_impulses(mut impulses: EventReader<Impulse>, mut velocities: Query<&mut Velocity>) {
    let mut dedupe_by_source = std::collections::HashMap::new();
    for impulse in impulses.read() {
        dedupe_by_source.insert(impulse.source, impulse);
    }

    let mut absolute = std::collections::HashSet::new();
    for (_, impulse) in dedupe_by_source {
        if let Ok(mut velocity) = velocities.get_mut(impulse.target) {
            if absolute.contains(&impulse.target) {
                continue;
            }

            if impulse.absolute {
                velocity.0 = impulse.amount;
                absolute.insert(impulse.target);
            } else {
                velocity.0 += impulse.amount;
            }
        }
    }
}

pub fn apply_velocity(
    mut tick: EventReader<PhysicsTick>,
    mut movement: Query<(&Velocity, &mut Transform)>,
) {
    for _tick in tick.read() {
        movement.iter_mut().for_each(|(velocity, mut transform)| {
            transform.translation += velocity.0;
        });
    }
}

#[derive(Component)]
pub struct ApplyGravity;

pub fn apply_gravity(
    mut tick: EventReader<PhysicsTick>,
    mut gravity: Query<&mut Velocity, With<ApplyGravity>>,
) {
    for _tick in tick.read() {
        gravity.iter_mut().for_each(|mut velocity| {
            velocity.0.y -= 0.75;
        });
    }
}
