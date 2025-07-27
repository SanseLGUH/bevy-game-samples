use bevy::prelude::*;

#[derive(Component)]
struct PlayableCube;

#[derive(Component)]
struct Placeholder;

fn main() {
    App::new()
        .add_plugins( DefaultPlugins )
        .add_plugins( ( bevy_egui::EguiPlugin::default(), bevy_inspector_egui::quick::WorldInspectorPlugin::new() ) )
        .add_systems( Startup, setup )
        .add_systems( Update, ( movement, camera_following ) )
        .run();
}


fn setup( 
    mut commands: Commands, 
    mut meshes: ResMut< Assets< Mesh > >, 
    mut materials: ResMut< Assets< StandardMaterial > > 
) {
    commands.spawn((
        PlayableCube, Mesh3d(meshes.add(  Cuboid::new( 1., 1., 1. ) )),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        PointLight {
            color: Color::srgb(1., 0., 0.),
            shadows_enabled: true,
            ..default()
        }, 
        Camera3d::default(),
        Transform {
            translation: Vec3::new( 0., 2., 2.5 ),
            rotation: Quat::from_rotation_x( -0.5 ),
            ..default()
        },
    ));
}

fn camera_following( player_cube: Query< &mut Transform, With< PlayableCube > >, mut camera: Query< &mut Transform, (With<Camera3d>, Without<PlayableCube>) >  ) {
    let player_transform = player_cube.single().unwrap();
    let mut camera_transform = camera.single_mut().unwrap();

    // if camera_transform.translation.x == player_transform.translation.x {
    //     return;
    // }

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y + 2.;
    camera_transform.translation.z = player_transform.translation.z + 3.5 ;
}

fn movement( player_cube: Query< &mut Transform, With<PlayableCube> > ) {

}
