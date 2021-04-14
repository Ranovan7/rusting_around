use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use rand::Rng;

static WIDTH: f32 = 1600.0;
static HEIGHT: f32 = 900.0;
static MIN_SPEED: f32 = 180.0;
static MAX_SPEED: f32 = 250.0;
static ACC_LIMIT: f32 = 50.0;

/// An implementation of the classic game "Breakout"
const TIME_STEP: f32 = 1.0 / 60.0;
pub fn boids_simulation() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(BirdConfig{
            ALIGNMENT_RAD: 100.0,
            COHESION_RAD: 100.0,
            SEPARATION_RAD: 100.0,
        })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(border_wrap_system.system())
        .add_system(emergence_system.system())
        // .add_system(border_avoidance_system.system())
        .add_system(bird_movement_system.system())
        .run();
}

struct BirdConfig {
    ALIGNMENT_RAD: f32,
    COHESION_RAD: f32,
    SEPARATION_RAD: f32,
}

struct BirdVel {
    id: u32,
    velocity: Vec3,
}

struct BirdAcc {
    acceleration: Vec3
}

fn euclidean_distance(source: Vec3, target: Vec3) -> f32 {
    ((target.x - source.x).powf(2.0) + (target.y - source.y).powf(2.0) + (target.z - source.z).powf(2.0)).sqrt()
}

fn vec_set_mag(origin: Vec3, new_mag: f32) -> Vec3 {
    let mut result = Vec3::new(0.0, 0.0, 0.0);
    let mag = (origin.x.powf(2.0) + origin.y.powf(2.0) + origin.z.powf(2.0)).sqrt();

    if mag != 0.0 {
        result.x = origin.x * new_mag / mag;
        result.y = origin.y * new_mag / mag;
        result.z = origin.z * new_mag / mag;
    }

    result
}

fn vec_clip(origin: Vec3, a_min: f32, a_max: f32) -> Vec3 {
    let mut result = Vec3::new(0.0, 0.0, 0.0);
    result.x = origin.x.min(a_max).max(a_min);
    result.y = origin.y.min(a_max).max(a_min);
    result.z = origin.z.min(a_max).max(a_min);

    result
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    // Add the game's entities to our world
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // Add walls
    let wall_material = materials.add(Color::rgb(0.2, 0.2, 0.2).into());
    let wall_thickness = 4.0;
    let bounds = Vec2::new(WIDTH + wall_thickness*2.0, HEIGHT + wall_thickness*2.0);
    // left
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        });
    // right
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        });
    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        });
    // top
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material,
            transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        });
    // Add birds
    let n_birds = 200;
    let bird_size = Vec2::new(15.0, 15.0);
    let bird_material = materials.add(Color::rgb(0.2, 0.4, 1.0).into());
    for bird in 0..n_birds {
        let bird_position = Vec3::new(
            rng.gen_range(0..WIDTH as i32) as f32 - WIDTH/2.0,
            rng.gen_range(0..HEIGHT as i32) as f32 - HEIGHT/2.0,
            0.0,
        );
        commands
            .spawn_bundle(SpriteBundle {
                material: bird_material.clone(),
                sprite: Sprite::new(bird_size),
                transform: Transform::from_translation(bird_position),
                ..Default::default()
            })
            .insert(BirdVel {
                id: bird,
                velocity: Vec3::new(
                    rng.gen_range(0..MAX_SPEED as i32 * 2) as f32 - MAX_SPEED,
                    rng.gen_range(0..MAX_SPEED as i32 * 2) as f32 - MAX_SPEED,
                    0.0,
                )
            })
            .insert(BirdAcc {
                acceleration: Vec3::new(0.0, 0.0, 0.0)
            });
    }
}

fn bird_movement_system(mut bird_query: Query<(&mut BirdVel, &mut BirdAcc, &mut Transform)>) {
    for (mut b_vel, mut b_acc, mut transform) in bird_query.iter_mut() {
        b_vel.velocity += b_acc.acceleration;
        b_vel.velocity = vec_clip(b_vel.velocity, -MAX_SPEED, MAX_SPEED);

        transform.translation += b_vel.velocity * TIME_STEP;

        b_acc.acceleration = Vec3::new(0.0, 0.0, 0.0);
    }
}

fn emergence_system(
    mut bird_query: Query<(&BirdVel, &Transform, &mut BirdAcc)>,
    other_bird_query: Query<(&BirdVel, &Transform)>,
    config: Res<BirdConfig>,
) {
    for (b_vel, b_trans, mut b_acc) in bird_query.iter_mut() {
        let mut steer_align = Vec3::new(0.0, 0.0, 0.0);
        let mut steer_cohesion = Vec3::new(0.0, 0.0, 0.0);
        let mut steer_separation = Vec3::new(0.0, 0.0, 0.0);
        let mut count_align = 0;
        let mut count_cohesion = 0;
        let mut count_separation = 0;

        for (ob_vel, ob_trans) in other_bird_query.iter() {
            if b_vel.id == ob_vel.id {
                continue;
            }

            let distance = euclidean_distance(b_trans.translation, ob_trans.translation);
            if distance < config.ALIGNMENT_RAD {
                count_align += 1;
                steer_align += ob_vel.velocity;
            }

            if distance < config.COHESION_RAD {
                count_cohesion += 1;
                steer_cohesion += ob_trans.translation;
            }

            if distance < config.SEPARATION_RAD {
                count_separation += 1;
                let mut diff = b_trans.translation.clone();
                diff -= ob_trans.translation;
                diff /= Vec3::new(distance, distance, distance);
                steer_separation += diff;
            }
        }

        if count_align > 0 {
            let count = count_align as f32;
            steer_align /= Vec3::new(count, count, count);
            steer_align = vec_set_mag(steer_align, MIN_SPEED);
            steer_align -= b_vel.velocity;
        }

        if count_cohesion > 0 {
            let count = count_cohesion as f32;
            steer_cohesion /= Vec3::new(count, count, count);
            steer_cohesion -= b_trans.translation;
            steer_cohesion = vec_set_mag(steer_cohesion, MIN_SPEED);
            steer_cohesion -= b_vel.velocity;
        }

        if count_separation > 0 {
            let count = count_separation as f32;
            steer_separation /= Vec3::new(count, count, count);
            steer_separation = vec_set_mag(steer_separation, MIN_SPEED);
            steer_separation -= b_vel.velocity;
        }

        steer_align = vec_clip(steer_align, -ACC_LIMIT, ACC_LIMIT);
        b_acc.acceleration += steer_align;

        steer_cohesion = vec_clip(steer_cohesion, -ACC_LIMIT, ACC_LIMIT);
        b_acc.acceleration += steer_cohesion;

        steer_separation = vec_clip(steer_separation, -ACC_LIMIT, ACC_LIMIT);
        b_acc.acceleration += steer_separation;
    }
}

fn border_avoidance_system(
    mut bird_query: Query<(&BirdVel, &Transform, &mut BirdAcc)>,
    config: Res<BirdConfig>,
) {
    for (b_vel, b_trans, mut b_acc) in bird_query.iter_mut() {
        let half_w = WIDTH / 2.0;
        let half_h = HEIGHT / 2.0;
        let mut diff_x = 0.0;
        if (half_w - b_trans.translation.x).abs() < config.SEPARATION_RAD {
            diff_x = 1.0;
        } else if half_w - b_trans.translation.x < config.SEPARATION_RAD {
            diff_x = -1.0;
        }

        let mut diff_y = 0.0;
        if (half_h - b_trans.translation.y).abs() < config.SEPARATION_RAD {
            diff_y = 1.0;
        } else if half_h - b_trans.translation.y < config.SEPARATION_RAD {
            diff_y = -1.0;
        }

        let mut steer = Vec3::new(diff_x, diff_y, 0.0);
        steer = vec_set_mag(steer, MIN_SPEED);
        steer -= b_vel.velocity;

        steer = vec_clip(steer, -ACC_LIMIT, ACC_LIMIT);
        b_acc.acceleration += steer;
    }
}

fn border_wrap_system(
    mut bird_query: Query<(&BirdVel, &mut Transform)>
) {
    for (b_vel, mut b_trans) in bird_query.iter_mut() {
        let half_w = WIDTH / 2.0;
        let half_h = HEIGHT / 2.0;
        if b_trans.translation.x < -half_w {
            b_trans.translation.x = half_w;
        } else if b_trans.translation.x > half_w {
            b_trans.translation.x = -half_w;
        }

        if b_trans.translation.y < -half_h {
            b_trans.translation.y = half_h;
        } else if b_trans.translation.y > half_h {
            b_trans.translation.y = -half_h;
        }
    }
}
