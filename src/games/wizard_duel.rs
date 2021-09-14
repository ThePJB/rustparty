use crate::components::physics_component;
/*
OK wizard duel
facing vector
lmb shoots projectiles
rmb is shield
have mana

nice got some of it working
controls are kind of annoying maybe needs to be twin stick
or auto strafe while held


*/
use crate::scene_manager::Game;
use crate::scene_manager::Signal;
use crate::canvas::Canvas;
use crate::input_state::*;
use crate::vec2::Vec2;
use crate::vec3::Vec3;
use crate::rect::Rect;
use crate::players::player_colours;

use crate::components::physics_component::*;
use crate::systems::collision::*;

use std::collections::HashMap;
use std::f32::INFINITY;
use std::f32::consts::PI;
use rand::Rng;


static player_speed: f32 = 0.25;
static player_size: f32 = 0.05;

static mana_bar_size: f32 = 0.01;
static mana_bar_gap: f32 = 0.005;


static mana_cost_to_fire: f32 = 0.7;
static mana_regen: f32 = 0.2;

static projectile_s: f32 = 0.025;
static projectile_speed: f32 = 0.4;
static projectile_ttl: f32 = 0.7;
static spawn_away_distance: f32 = 0.02;


#[derive(Clone, Copy, Debug)]
enum EntityType {
    Player,
    MagicFire,
}

#[derive(Debug)]
struct PlayerComponent {
    facing: Vec2,
    mana: f32,
}

#[derive(Debug)]
struct ProjectileComponent {
    ttl: f32,
}

#[derive(Debug, Clone, Copy)]
enum SideEffect {
    SpawnProjectile(u32, Vec2, Vec2),
    KillEntity(u32),
}

#[derive(Debug)]
struct EntityManager {
    pub phys_comps: HashMap<u32, PhysicsComponent>,
    pub player_comps: HashMap<u32, PlayerComponent>,
    pub projectile_comps: HashMap<u32, ProjectileComponent>,
    pub team_comps: HashMap<u32, u32>,
    generation: u32,
}

impl EntityManager {
    fn new() -> EntityManager {
        EntityManager {
            phys_comps: HashMap::new(),
            player_comps: HashMap::new(),
            projectile_comps: HashMap::new(),
            team_comps: HashMap::new(),
            generation: 0,
        }
    }

    fn add_player(&mut self, team: u32, pos: Vec2) -> u32 {
        let current_gen = self.generation;
        self.phys_comps.insert(current_gen, PhysicsComponent{aabb: Rect::new_centered(pos.x, pos.y, player_size, player_size), vel: Vec2::zero()});
        self.player_comps.insert(current_gen, PlayerComponent{facing: Vec2::new(1.0, 0.0), mana: 1.0});
        self.team_comps.insert(current_gen, team);
        self.generation = current_gen + 1;
        current_gen
    }

    fn add_projectile(&mut self, team: u32, pos: Vec2, dir: Vec2) -> u32 {
        let spawn_pos = pos.add(dir.mul_scalar(spawn_away_distance));
        let actual_vel = dir
            .rotate(rand::thread_rng().gen_range(-0.1..0.1))
            .mul_scalar(projectile_speed + rand::thread_rng().gen_range(-0.1..0.1));
        let current_gen = self.generation;
        self.phys_comps.insert(current_gen, PhysicsComponent{aabb: Rect::new_centered(spawn_pos.x, spawn_pos.y, projectile_s, projectile_s), vel: actual_vel});
        self.projectile_comps.insert(current_gen, ProjectileComponent{ttl: projectile_ttl});
        self.team_comps.insert(current_gen, team);

        self.generation = current_gen + 1;
        current_gen
    }

    fn delete_entity(&mut self, key: u32) {
        self.phys_comps.remove(&key);
        self.player_comps.remove(&key);
        self.projectile_comps.remove(&key);
    }

    fn resolve_side_effect(&mut self, side_effect: SideEffect) {
        match side_effect {
            SideEffect::KillEntity(id) => {self.delete_entity(id)},
            SideEffect::SpawnProjectile(team, pos, dir) => {self.add_projectile(team, pos, dir);},
        }
    }
}


#[derive(Debug)]
pub struct WizardDuel {
    side_effects: Vec<SideEffect>,
    collision_events: Vec<CollisionEvent>,
    entities: EntityManager,
}

impl WizardDuel {
    pub fn new(a: f32) -> WizardDuel {
        let mut game = WizardDuel{
            entities: EntityManager::new(),
            collision_events: Vec::new(),
            side_effects: Vec::new(),
        };
        let center = Vec2::new(a/2.0, 0.5);
        
        for i in 0..4 {
            let spawn_pos = center.add(Vec2::new(0.3, 0.0).rotate(2.0*PI/4.0 * i as f32));
            game.entities.add_player(i, spawn_pos);
        }

        println!("game: {:?}", game);

        return game;
    }

    fn handle_player_input(&mut self, player: u32, state: &PlayerInputState, dt: f32) {
        if let Some(player_component) = self.entities.player_comps.get_mut(&player) {
            let phys_component = self.entities.phys_comps.get_mut(&player).unwrap();
            let team_component = self.entities.team_comps.get(&player).unwrap();

            let strafe = state.held_inputs.contains(&Input::Action2);

            phys_component.vel.y = 
                if state.held_inputs.contains(&Input::Up) {
                    -1.0
                } else if state.held_inputs.contains(&Input::Down) {
                    1.0
                } else {
                    0.0
                };

            phys_component.vel.x = 
                if state.held_inputs.contains(&Input::Left) {
                    -1.0
                } else if state.held_inputs.contains(&Input::Right) {
                    1.0
                } else {
                    0.0
                };

            if phys_component.vel.x != 0.0 || phys_component.vel.y != 0.0 {
                if !strafe {player_component.facing = phys_component.vel.normalize();}
                phys_component.vel = phys_component.vel.normalize().mul_scalar(player_speed);
            }
                
            if state.held_inputs.contains(&Input::Action1) {
                let mana_cost = mana_cost_to_fire * dt;
                if player_component.mana > mana_cost {
                    player_component.mana -= mana_cost;
                    self.side_effects.push(SideEffect::SpawnProjectile(*team_component, phys_component.aabb.center(), player_component.facing));
                }
            } else {
                player_component.mana = 1.0f32.min(player_component.mana + mana_regen * dt);
            }
        }
    }

}

impl Game for WizardDuel {
    fn update(&mut self, inputs: &InputState, dt: f64) -> Option<Signal> {
        
        // clear arenas
        let mut collision_events = Vec::new();
        //self.collision_events.clear();
        self.side_effects.clear();

        // handle input
        for (player_number, state) in inputs.iter().enumerate() {
            self.handle_player_input(player_number as u32, state, dt as f32);
        }

        simulate_collisions(&self.entities.phys_comps, &mut collision_events, dt as f32);

        // filter out same team collisions
        collision_events.retain(|col| {
            let subject_team = self.entities.team_comps.get(&col.subject).unwrap();
            let object_team = self.entities.team_comps.get(&col.object).unwrap();
            subject_team != object_team
        });

        for col in collision_events.iter() {
            if let Some(subject_proj) = self.entities.projectile_comps.get(&col.subject) {
                if let Some(object_proj) = self.entities.projectile_comps.get(&col.object) {
                    // projectiles kill each other
                    self.side_effects.push(SideEffect::KillEntity(col.subject));
                    self.side_effects.push(SideEffect::KillEntity(col.object));
                } else if let Some(object_player) = self.entities.player_comps.get(&col.object) {
                    // projectiles kill players
                    self.side_effects.push(SideEffect::KillEntity(col.subject));
                    self.side_effects.push(SideEffect::KillEntity(col.object));
                }
            }
        }

        apply_movement(&mut self.entities.phys_comps, &collision_events, dt as f32);
        
        for (projectile_key, projectile_component) in self.entities.projectile_comps.iter_mut() {
            projectile_component.ttl -= dt as f32;
            if projectile_component.ttl <= 0.0 {
                self.side_effects.push(SideEffect::KillEntity(*projectile_key));
            }
        }

        for side_effect in self.side_effects.iter() {
            self.entities.resolve_side_effect(*side_effect);
        }

        if self.entities.player_comps.len() == 1 {
            Some(Signal::Victory(*self.entities.player_comps.keys().nth(0).unwrap()))
        } else {
            None
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        canvas.clear(Vec3::new(0.5, 0.5, 1.0));

        for (key, phys_comp) in self.entities.phys_comps.iter() {
            let team = self.entities.team_comps.get(key).unwrap();
            if let Some(projectile) = self.entities.projectile_comps.get(key) {
                canvas.draw_rect(phys_comp.aabb, player_colours[*team as usize].mul_scalar(0.8));
            } else if let Some(player_comp) = self.entities.player_comps.get(key) {
                canvas.draw_rect(phys_comp.aabb, player_colours[*team as usize].mul_scalar(1.0));
                let mana_bar_bg_rect = Rect::new(phys_comp.aabb.x, phys_comp.aabb.y - mana_bar_gap - mana_bar_size, phys_comp.aabb.w, mana_bar_size);
                let mut mana_bar_rect = mana_bar_bg_rect;
                mana_bar_rect.w *= player_comp.mana;
                canvas.draw_rect(mana_bar_bg_rect, Vec3::zero());
                canvas.draw_rect(mana_bar_rect, Vec3::new(0.0, 0.0, 1.0));
            }
        }
        canvas.present();
    }
}