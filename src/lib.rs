use bevy::prelude::*;
/*
 * example camera setup
fn camera_setup(mut comms:Commands){
    comms.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 32., 16.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    }).insert(MainCamera);
}
*/
#[derive(Component)]
    struct MainCamera;
    #[derive(Component)]
    struct Player{
        id:usize,
    }
    
    
    
    pub struct CameraFolloPlugin;
    impl Plugin for CameraFolloPlugin{
        fn build(&self,app:&mut App){
            app
            .add_plugins(DefaultPlugins)
          
            .add_systems(Update, camera_follow);

        }
            
    }

/*
 example player movement 
fn player_movement(mut player_query:Query<(&mut Transform,&Speed),With<Player>>,
     input:Res<ButtonInput<KeyCode>>,
    time:Res<Time>,
    ){
    for (mut t, p) in &mut player_query{
         let mut movement_vector = Vec3::ZERO;
    for key in input.get_pressed(){
        match key {
            KeyCode::KeyW => movement_vector.z -= p.base_speed ,
            KeyCode::KeyA => movement_vector.x -= p.base_speed ,
            KeyCode::KeyS => movement_vector.z += p.base_speed ,
            KeyCode::KeyD => movement_vector.x += p.base_speed ,
            _ =>(),
            
        }
    }
    // im doing additive speed multipliers 
    let total_multiplier:f32 = p.speed_multipliers.as_slice().iter().sum();
    //movement is normalised so diagonal movement isn't faster

    t.translation += movement_vector.normalize_or_zero() * time.delta_seconds() * p.base_speed * total_multiplier;

    }

}
 */

fn camera_follow(mut camera_query:Query<&mut Transform,(With<MainCamera>,Without<Player>)>,
    player_query:Query<&Transform,(Without<MainCamera>,With<Player>)>,
    time:Res<Time>,
    )
    {
    let mut cam = camera_query.single_mut();
    let player_t = player_query.single();
    cam.translation.x +=(player_t.translation.x - cam.translation.x)   * time.delta_seconds(); 
    
    let y_offset = 64.;
    
    cam.translation.y +=(player_t.translation.y - cam.translation.y + y_offset )  * time.delta_seconds() ; 
    //same thing for z
    let z_offset = y_offset / 2.;
    cam.translation.z +=(player_t.translation.z - cam.translation.z + z_offset )   * time.delta_seconds() ; 
}
