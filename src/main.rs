use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const SCALE_RATE: f32 = 0.005;
const RIGHT_HAND_DISTANCE: f32 = -32.;
const LEFT_HAND_DISTANCE: f32 = 32.;
const ENEMY_DISPLACEMENT_FROM_CENTER: f32 = -200.;
const HAND_DISPLACEMENT: f32 = 24.;
const HAND_SCALE_RATIO: f32 = 1.5;

const WORD1: &str = "help";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, (setup, spawn_enemy, spawn_background))
        .add_systems(
            Update,
            (enemy_approach, enemy_approach_left, enemy_approach_right),
        )
        .add_systems(Update, text_input)
        .run()
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct RightHand;

#[derive(Component)]
struct LeftHand;


fn setup(
    mut commands: Commands,
){
    commands.spawn(Camera2dBundle::default());
}

fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemy_texture: Handle<Image> = asset_server.load("enemy.png");
    let left_hand_texture: Handle<Image> = asset_server.load("left_hand.png");
    let right_hand_texture: Handle<Image> = asset_server.load("right_hand.png");

    commands
        .spawn(SpriteBundle{
            sprite: Sprite { custom_size: Some(Vec2::new(64., 128.)), ..default()},
            transform: Transform::from_xyz(0., ENEMY_DISPLACEMENT_FROM_CENTER, 5.),
            texture: enemy_texture,
            ..default()
        })
        .insert(Enemy);

    commands
        .spawn(SpriteBundle{
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)), ..default()},
            transform: Transform::from_xyz(RIGHT_HAND_DISTANCE, ENEMY_DISPLACEMENT_FROM_CENTER + HAND_DISPLACEMENT, 10.),
            texture: right_hand_texture,
            ..default()
        })
        .insert(RightHand);

    commands
        .spawn(SpriteBundle{
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)), ..default()},
            transform: Transform::from_xyz(LEFT_HAND_DISTANCE, ENEMY_DISPLACEMENT_FROM_CENTER + HAND_DISPLACEMENT, 10.),
            texture: left_hand_texture,
            ..default()
        })
        .insert(LeftHand);
}

fn spawn_background(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.1, 0.1, 0.8),
            custom_size: Some(Vec2::new(5000., 5000.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., -10.),
        ..default()
    });

    commands
        .spawn(SpriteBundle{
            sprite: Sprite { color: Color::rgb(0., 0., 0.), custom_size: Some(Vec2::new(3000., 5000.)), ..default()},
            transform: Transform::from_xyz(1850., 0., -10.),
            ..default()
        });

    commands
        .spawn(SpriteBundle{
            sprite: Sprite { color: Color::rgb(0., 0., 0.), custom_size: Some(Vec2::new(300., 5000.)), ..default()},
            transform: Transform::from_xyz(-500., 0., -10.),
            ..default()
        });
}


fn enemy_approach(
    mut enemy: Query<&mut Transform ,With<Enemy>>,
){
    for mut transform in enemy.iter_mut(){
        transform.scale.x += SCALE_RATE;
        transform.scale.y += SCALE_RATE;
    }
}
fn enemy_approach_left(mut left_hand: Query<&mut Transform, With<LeftHand>>) {
    for mut left_transform in left_hand.iter_mut() {
        left_transform.scale.x += SCALE_RATE * HAND_SCALE_RATIO;
        left_transform.scale.y += SCALE_RATE * HAND_SCALE_RATIO;
        left_transform.translation.y =
            ENEMY_DISPLACEMENT_FROM_CENTER + (HAND_DISPLACEMENT * left_transform.scale.y);
        left_transform.translation.x = LEFT_HAND_DISTANCE * left_transform.scale.x;
    }
}

fn enemy_approach_right(mut right_hand: Query<&mut Transform, With<RightHand>>) {
    for mut right_transform in right_hand.iter_mut() {
        right_transform.scale.x += SCALE_RATE * HAND_SCALE_RATIO;
        right_transform.scale.y += SCALE_RATE * HAND_SCALE_RATIO;
        right_transform.translation.y =
            ENEMY_DISPLACEMENT_FROM_CENTER + (HAND_DISPLACEMENT * right_transform.scale.y);
        right_transform.translation.x = RIGHT_HAND_DISTANCE * right_transform.scale.x;
    }
}

//got this code here https://bevy-cheatbook.github.io/input/char.html?highlight=text#text--character-input
fn text_input(
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    mut string: Local<String>,
) {
    if kbd.just_pressed(KeyCode::Return) {
        println!("Text input: {}", &*string);
        if *string == WORD1 {
            println!("correct");
        }
        string.clear();
    }
    if kbd.just_pressed(KeyCode::Back) {
        string.pop();
    }
    for ev in evr_char.read() {
        if !ev.char.is_control() {
            string.push(ev.char);
        }
    }
}
