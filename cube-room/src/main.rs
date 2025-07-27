use bevy::prelude::*;

#[derive(Component)]
struct PlayableCube;

#[derive(Component)]
struct Placeholder;

fn main() {
    App::new()
        .add_plugins( DefaultPlugins )
        .add_plugins( ( bevy_spectator::SpectatorPlugin, bevy_egui::EguiPlugin::default(), bevy_inspector_egui::quick::WorldInspectorPlugin::new() ) )
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
        Camera3d::default(), bevy_spectator::Spectator,
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn camera_following( player_cube: Query< &mut Transform, With< PlayableCube > >, camera: Query< &mut Transform, (With<Camera3d>, Without<PlayableCube>) >  ) {

}

fn movement( player_cube: Query< &mut Transform, With<PlayableCube> > ) {
}