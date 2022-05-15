use bevy::{math::const_vec2, prelude::*};
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Playground".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bouncing)
        .run();
}

#[derive(Component)]
struct Moving {
    size: Vec2,
    speed: Vec2,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let mut rng = rand::thread_rng();
    for _ in 0..rng.gen_range(3..6) {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("vleue.png"),
                ..default()
            })
            .insert(Moving {
                size: const_vec2!([188.0, 162.0]),
                speed: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize()
                    * rng.gen_range(0.5..2.0),
            });
    }
    for _ in 0..rng.gen_range(3..6) {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("birdoggo.png"),
                ..default()
            })
            .insert(Moving {
                size: const_vec2!([200.0; 2]),
                speed: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize()
                    * rng.gen_range(0.5..2.0),
            });
    }
}

fn bouncing(
    mut moving: Query<(&mut Moving, &mut Transform)>,
    time: Res<Time>,
    windows: Res<Windows>,
) {
    for (mut moving, mut transform) in moving.iter_mut() {
        transform.translation += (moving.speed * time.delta_seconds() * 200.0).extend(0.0);
        let window = windows.primary();
        if transform.translation.x + moving.size.x / 2.0 > window.width() / 2.0
            && moving.speed.x > 0.0
        {
            moving.speed.x *= -1.0;
        }
        if transform.translation.x - moving.size.x / 2.0 < -window.width() / 2.0
            && moving.speed.x < 0.0
        {
            moving.speed.x *= -1.0;
        }
        if transform.translation.y + moving.size.y / 2.0 > window.height() / 2.0
            && moving.speed.y > 0.0
        {
            moving.speed.y *= -1.0;
        }
        if transform.translation.y - moving.size.y / 2.0 < -window.height() / 2.0
            && moving.speed.y < 0.0
        {
            moving.speed.y *= -1.0;
        }
    }
}
