use bevy::{prelude::*, render::camera::ScalingMode, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod attack;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some( Window{
                    title:"First game".into(),
                    resolution:(640.0,480.0).into(),
                    resizable:false,
                    ..default()
                }),
                ..default()
            }).build(),
        )
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement))
        .add_plugins(ui::GameUI)
        .add_plugins(attack::AttackPlugin)
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
        .run();
 }


 fn setup(mut commands:Commands, asset_server: Res<AssetServer>){
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle{
        sprite: Sprite{
            custom_size: Some(Vec2::new(25.0, 50.0)),
            ..default()
        },
        texture,
        ..default()
    }, Player {speed: 100.0}, Name::new("Player-Otis")));
 }


 #[derive(Component)]
 pub struct Player{
    pub speed:f32,
 }


#[derive(Resource)]
pub struct Money(pub f32);


 fn character_movement (mut characters: Query<(&mut Transform, &Player)>, input:Res<Input<KeyCode>>, time:Res<Time>){
    for (mut transform, player ) in &mut characters{
        if input.pressed(KeyCode::W){
            transform.translation.y += player.speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::S){
            transform.translation.y -= player.speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::A){
            transform.translation.x -=  player.speed* time.delta_seconds();
        }
        if input.pressed(KeyCode::D){
            transform.translation.x += player.speed * time.delta_seconds();
        }

    }

 }
