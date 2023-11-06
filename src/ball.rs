use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::Paddle;
const BALL_SPEED: f32 = 50.0;
const HALF_BALL_WIDTH: f32 = 3.0 / 2.0;
const HALF_BALL_HEIGHT: f32 = 3.0 / 2.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, move_balls);
            //.add_systems(Update, reset);
    }
}

#[derive(Component)]
struct Ball {
    velocity: Vec3,
}

fn spawn_ball (
        mut commands: Commands, 
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>
) {
        // Circle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(15.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Ball {
            velocity: Vec3::new(BALL_SPEED, BALL_SPEED, 0.0),
        },
    ));
}

fn move_balls(
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    time: Res<Time>,
    paddle_query: Query<(&Transform, &Paddle), Without<Ball>>,
) {
    for (mut ball_transform, mut ball) in ball_query.iter_mut() {
        ball_transform.translation += ball.velocity * time.delta_seconds();
        let ball_y = ball_transform.translation.y;
        if ball_y + 15.0/2. > 200. {
            ball.velocity *= -1.0;
            ball_y -=1.
        }
        if ball_y - 15.0/2. < -200. {
            ball.velocity *= -1.0;
            ball_y +=1.
        }
        if (ball_y == -150.) 
            || (ball_y == 150.) {
                if ball_y <= paddle  

        } 
    }
    

}
