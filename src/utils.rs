use bevy::prelude::*;
use bevy::render::mesh::shape::Cube;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{bullet::Bullet, lifespan::Lifespan},
    plugins::damage::HitBox,
    RAPIER_SCALE,
};

pub fn _spawn_bullet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Transform,
) -> Entity {
    let forward = get_forward(&position);
    let linvel = forward * 50.0;
    commands
        .spawn()
        .insert(Bullet {})
        .insert(Lifespan { duration: 3.0 })
        // Rapier
        .insert(RigidBody::Dynamic)
        .insert(position)
        .insert(Velocity {
            linvel,
            ..Default::default()
        })
        .insert(Collider::cuboid(0.1 * RAPIER_SCALE, 0.1 * RAPIER_SCALE))
        .insert(ColliderMassProperties::Density(0.1))
        .insert(ActiveEvents::COLLISION_EVENTS)
        //
        .with_children(|builder| {
            builder.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.0, 0.0, 0.9).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            });
        })
        .id()
}

pub fn get_forward(position: &Transform) -> Vec2 {
    let (axis, angle) = position.rotation.to_axis_angle();
    // println!(
    //     "axis: {axis}, angle: {angle}, x/y: {:.2}/{:.2}",
    //     angle.cos(),
    //     angle.sin()
    // );
    let angle = angle * axis.z;
    Vec2::new(angle.cos(), angle.sin())
    // Vec2::new(
    //     position.rotation.sin_angle(),
    //     -position.rotation.cos_angle(),
    // )
    // Vec2::new(
    //     position.rotation.cos_angle(),
    //     position.rotation.sin_angle(),
    // )
    // Vec2::new(position.position.rotation.re, position.position.rotation.im)
}

pub fn get_forward_global(position: &GlobalTransform) -> Vec2 {
    let (axis, angle) = position.to_scale_rotation_translation().1.to_axis_angle();
    let angle = angle * axis.z;
    Vec2::new(angle.cos(), angle.sin())
}

pub struct Melee {
    pub parent_position: Transform,
    pub offset: Vec2,
    pub lifespan: f32,

    pub source: Entity,
    pub shape: Shape,

    pub damage: i32,
    pub hit_stun: f32,
    pub knockback: Knockback,
}

#[derive(Debug, Clone, Copy, Inspectable)]
pub enum Shape {
    Ball(f32),
    Cuboid(f32, f32),
}

#[derive(Debug, Clone, Copy, Inspectable)]
pub enum Knockback {
    Center(f32),
    Direction(f32, f32),
}

impl From<Shape> for Collider {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Ball(radius) => Collider::ball(radius * RAPIER_SCALE),
            Shape::Cuboid(hx, hy) => Collider::cuboid(hx * RAPIER_SCALE, hy * RAPIER_SCALE),
        }
    }
}

pub fn spawn_melee(melee: Melee, commands: &mut Commands) -> Entity {
    let mut next_pos = melee.parent_position;

    let forward = get_forward(&melee.parent_position);
    let offset = forward * melee.offset.x * RAPIER_SCALE;
    next_pos.translation.x += offset.x;
    next_pos.translation.y += offset.y;

    let id = commands
        .spawn()
        .insert_bundle(SpatialBundle {
            transform: next_pos,
            ..Default::default()
        })
        .insert(Name::new("Melee"))
        .insert(HitBox {
            source: melee.source,
            damage: melee.damage,
            hit_stun: melee.hit_stun,
            knockback: melee.knockback,
        })
        .insert(Lifespan {
            duration: melee.lifespan,
        })
        // Rapier
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::from(melee.shape))
        .insert(ColliderMassProperties::Density(0.1))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::all())
        .insert(Sensor)
        //
        .insert(MeleeParent {
            entity: melee.source,
            offset: melee.offset,
        })
        .id();
    id
}
#[derive(Debug, Component, Inspectable)]
pub struct MeleeParent {
    pub entity: Entity,
    pub offset: Vec2,
}

pub fn sync_melee(
    mut query: Query<(&mut Transform, &MeleeParent), With<MeleeParent>>,
    parent: Query<&Transform, Without<MeleeParent>>,
) {
    for (mut pos, melee) in query.iter_mut() {
        if let Ok(parent) = parent.get(melee.entity) {
            let mut next_pos = *parent;

            let forward = get_forward(parent);
            let offset = forward * melee.offset.x * RAPIER_SCALE;
            next_pos.translation.x += offset.x;
            next_pos.translation.y += offset.y;

            *pos = next_pos;
        }
    }
}

pub struct Projectile {
    // pub offset: Vec2,
    // pos + rot / dir to move
    pub position: Transform,
    pub lifespan: f32,

    pub linvel: Vec2,

    pub source: Entity,
    pub shape: Shape,

    pub damage: i32,
    pub hit_stun: f32,
    pub knockback: Knockback,
}

pub fn spawn_projectile(projectile: Projectile, commands: &mut Commands) -> Entity {
    commands
        .spawn()
        .insert_bundle(SpatialBundle {
            transform: projectile.position,
            ..Default::default()
        })
        .insert(Name::new("Projectile"))
        .insert(HitBox {
            source: projectile.source,
            damage: projectile.damage,
            hit_stun: projectile.hit_stun,
            knockback: projectile.knockback,
        })
        .insert(Lifespan {
            duration: projectile.lifespan,
        })
        // Rapier
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            linvel: projectile.linvel,
            ..Default::default()
        })
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Collider::from(projectile.shape))
        //
        .id()
}
