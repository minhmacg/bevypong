mod ball;
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle {
    side: Side
}

#[derive(Component,PartialEq)]
enum Side {
    Left,
    Right,
}

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 500;

use ball::BallPlugin;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BallPlugin))
        .add_systems(Startup, spawn_things)
        .add_systems(Update, move_paddle)
        .run();
}           
fn spawn_things(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(250.0, 5.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., -200., 0.)),
        ..default()
        },
        ));
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(250.0, 5.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 200., 0.)),
        ..default()
        },
        ));
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(20.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
        },
        Paddle {
            side: Side::Left
            },
        ));
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(20.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
        ..default()
        },
        Paddle {
            side: Side::Right
            },
        ));
}
fn move_paddle(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Paddle)>,
    input: Res<Input<KeyCode>>
) {
    for (mut transform, paddle) in query.iter_mut(){
        if paddle.side == Side::Left {
            if input.pressed(KeyCode::W){
                transform.translation.y += 200. * time.delta_seconds();
            }
            if input.pressed(KeyCode::S){
                transform.translation.y -= 200. * time.delta_seconds();
            }
        }
        if paddle.side == Side::Right {
            if input.pressed(KeyCode::U){
                transform.translation.y += 200. * time.delta_seconds();
            }
            if input.pressed(KeyCode::J){
                transform.translation.y -= 200. * time.delta_seconds();
            }
        }
        if transform.translation.y - 50. < -200. {
            transform.translation.y = -200. + 50.;
            }

        if transform.translation.y + 50. > 200. {
            transform.translation.y = 200. - 50.;
            }
    }
}
fn control_paddle (
    keyinput: Res<Input<KeyCode>>,
    mut command: Commands
    ){
}

fn control_ball(
    keys : Res<Input<KeyCode>>,
    _command: Commands,
){
}
