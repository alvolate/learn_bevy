
use bevy::{prelude::*, render::camera::ScalingMode};
use crate::{Money, Player};
pub struct AttackPlugin;

impl Plugin for AttackPlugin{
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup, setAttack)
        .add_systems(Update,(attack, attack_lifetime))
        .register_type::<Attack_Player>();
    }   
}

#[derive(Component,Default,Reflect)]
#[reflect(Component)]
pub struct Attack_Player{
   pub lifetime: Timer,
}

#[derive(Component)]
pub struct AttackParent;

fn setAttack( mut commands:Commands){
    commands.spawn((SpatialBundle::default(), AttackParent, Name::new("Attack Parent")));
}

fn attack(
    mut commands: Commands, 
    asset_server:Res<AssetServer>, 
    input:Res<Input<KeyCode>>, 
    mut money:ResMut<Money>, 
    player:Query<&Transform, With<Player>>,
    parent: Query<Entity, With<AttackParent>>)
    {
        if !input.just_pressed(KeyCode::Space)  {
            return;
        }

        let player_transform = player.single();
        if money.0 >= 10.0 {
            money.0 -= 10.0;
            info!("Spent $10 to attack, remaing money: {}", money.0);
            let texture = asset_server.load("attack.png");
            let parent = parent.single();

            commands.entity(parent).with_children(|commands| {
                commands.spawn( (
                    SpriteBundle{
                        sprite: Sprite{
                            custom_size: Some(Vec2::new(50.0, 50.0)),
                            ..default()
                        },
                        texture, 
                        transform: *player_transform, 
                        ..default()},
                    Attack_Player{lifetime: Timer::from_seconds(2.0, TimerMode::Once)},
                    Name::new("Attack"),
                ));
            });
        }
}

fn attack_lifetime(mut commands: Commands, time:Res<Time>,parent: Query<Entity, &AttackParent>,
     mut attack_player: Query<(Entity, &mut Attack_Player)>, mut money: ResMut<Money>,){

    let parent = parent.single();

    for (attack_entity, mut attack) in &mut attack_player{
        attack.lifetime.tick(time.delta());

        if attack.lifetime.finished(){
            money.0 += 15.0;
            commands.entity(parent).remove_children(&[attack_entity]);
            commands.entity(attack_entity).despawn();
            info!("Attack sucess and earned $15, remaing money: {}", money.0);
        }
    }

}