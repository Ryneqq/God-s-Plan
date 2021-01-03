use bevy::prelude::*;
use crate::WorldMap;

pub (super) fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world: Res<WorldMap>,
) {
    let hextile = asset_server.load("tiles/empty/hextile.gltf#Mesh0/Primitive0");
    let material = materials.add(StandardMaterial {
        albedo: Color::rgb(1.0, 1.0, 1.0),
        ..Default::default()
    });
    let mut transform = Transform::from_translation(Vec3::zero())
        .looking_at(Vec3::unit_x(), Vec3::unit_z());

    for hexagon_position in world.render() {
        transform.translation = hexagon_position;

        // ! This is causing a problem with rotation calculations
        let transform = Transform::from_translation(hexagon_position)
            .looking_at(Vec3::unit_x(), Vec3::unit_z());

        commands.spawn(PbrBundle {
            mesh: hextile.clone(),
            material: material.clone(),
            transform: transform.clone(),
            ..Default::default()
        });
    }

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 25.0)),
        .. Default::default()
    }).spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
        ..Default::default()
    });
}
