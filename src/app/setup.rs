use bevy::prelude::*;
use bevy_mod_picking::*;
use crate::WorldMap;

pub (super) fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world: Res<WorldMap>,
) {
    let hextile = asset_server.load("tiles/empty/hextile.gltf#Mesh0/Primitive0");
    let mut transform = Transform::from_translation(Vec3::zero())
        .looking_at(Vec3::unit_x(), Vec3::unit_z());

    commands
        .spawn(Camera3dBundle {
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(0.0, -5.0, 25.0));
                let rotation = Quat::from_rotation_x(0.2);

                transform.rotate(rotation);

                transform
            },
            .. Default::default()
        })
        .with(PickSource::default())
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        });


    for hexagon_position in world.render() {
        transform.translation = hexagon_position;

        commands
            .spawn(PbrBundle {
                mesh: hextile.clone(),
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                transform: transform.clone(),
                ..Default::default()
            })
            .with(PickableMesh::default())
            .with(InteractableMesh::default())
            .with(HighlightablePickMesh::default())
            .with(SelectablePickMesh::default());
    }
}
