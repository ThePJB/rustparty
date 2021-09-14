use crate::rect::*;
use crate::grid::*;
use crate::components::physics_component::PhysicsComponent;
use crate::vec2::*;

use std::collections::HashMap;

// take rect subjects and rect static



#[derive(Debug)]
pub struct CollisionEvent {
    pub subject: u32,
    pub object: u32,
    pub dir: Vec2,
    pub subject_rect: Rect,
    pub object_rect: Rect,
    pub subject_vel: Vec2,
    pub object_vel: Vec2,
}

pub fn simulate_collisions(subjects: &HashMap<u32, PhysicsComponent>, collisions: &mut Vec<CollisionEvent>, t: f32) {
    for (subject_key, subject_comp) in subjects {
        let subject_rect_old = subject_comp.aabb;
        let subject_rect_desired = subject_rect_old.translate(subject_comp.vel.mul_scalar(t));

        for (object_key, object_comp) in subjects {
            if *subject_key == *object_key {continue};

            let object_rect_old = object_comp.aabb;
            let object_rect_desired = object_rect_old.translate(object_comp.vel.mul_scalar(t));

            if rect_intersection(subject_rect_desired, object_rect_desired) {
                //let collision_dir = rect_collision_direction(subject_rect_old, subject_rect_desired, object_rect_desired);
                let collision_dir = least_penetration(subject_rect_desired, object_rect_desired).normalize();
                collisions.push(CollisionEvent {
                    dir: collision_dir,
                    subject: *subject_key,
                    object: *object_key,
                    subject_rect: subject_rect_old,
                    object_rect: object_rect_old,
                    subject_vel: subject_comp.vel,
                    object_vel: object_comp.vel,
                });
            }
        }
    }
}

fn movement_bounds(subject_key: u32, collisions: &Vec<CollisionEvent>) -> (f32, f32, f32, f32) {
    let max_dx = collisions.iter().filter(|col| col.subject == subject_key)
        .filter(|col| col.dir == Vec2::left())
        .map(|col| col.object_rect.left() - col.subject_rect.right())
        .fold(f32::INFINITY, |a, b| a.min(b));

    let max_dy = collisions.iter().filter(|col| col.subject == subject_key)
        .filter(|col| col.dir == Vec2::up())
        .map(|col| col.object_rect.top() - col.subject_rect.bot())
        .fold(f32::INFINITY, |a, b| a.min(b));
        
    let min_dx = collisions.iter().filter(|col| col.subject == subject_key)
        .filter(|col| col.dir == Vec2::right())
        .map(|col| col.object_rect.right() - col.subject_rect.left())
        .fold(-f32::INFINITY, |a, b| a.max(b));

    let min_dy = collisions.iter().filter(|col| col.subject == subject_key)
        .filter(|col| col.dir == Vec2::down())
        .map(|col| col.object_rect.bot() - col.subject_rect.top())
        .fold(-f32::INFINITY, |a, b| a.max(b));

    return (min_dx, max_dx, min_dy, max_dy);
}

fn clamp(val: f32, min: f32, max: f32) -> f32 {
    match val {
        val if val <= min => min,
        val if val >= max => max,
        _ => val
    }
}

pub fn apply_movement(entities: &mut HashMap<u32, PhysicsComponent>, collisions: &Vec<CollisionEvent>, dt: f32) {
    for (entity_key, entity) in entities.iter_mut() {
        let (min_x, max_x, min_y, max_y) = movement_bounds(*entity_key, collisions);
        let x_movt = clamp(entity.vel.x * dt, min_x, max_x);
        let y_movt = clamp(entity.vel.y * dt, min_y, max_y);

        entity.aabb.x += x_movt;
        entity.aabb.y += y_movt;
    }
}

#[test]
fn test_apply_movement() {
    // y no detect collision??

    let mut entities = HashMap::new();
    let mut collisions = Vec::new();
    entities.insert(0u32, PhysicsComponent{aabb: Rect::new(0.0, 0.0, 1.0, 1.0), vel: Vec2::new(1.0, 0.0)});
    entities.insert(1u32, PhysicsComponent{aabb: Rect::new(1.5, 0.0, 1.0, 1.0), vel: Vec2::new(0.0, 0.0)});
    simulate_collisions(&entities, &mut collisions, 1.0);
    println!("collisions: {:?}", collisions);
    println!("before movt: {:?}", entities);
    apply_movement(&mut entities, &collisions, 1.0);
    println!("after movt: {:?}", entities);

    let subj_x = entities.get(&0u32).unwrap().aabb.x;
    assert_eq!(subj_x, 0.5);
}