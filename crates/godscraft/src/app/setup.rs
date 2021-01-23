use bevy::prelude::*;
use bevy_mod_picking::*;
use crate::WorldMap;
use super::config::Config;

pub (super) fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world: Res<WorldMap>,
    config: Res<Config>,
) {
    let empty_hextile = asset_server.load("tiles/empty/plane/empty_hextile_plane.gltf#Mesh0/Primitive0"); // TODO: Should be in config files
    let hextile = asset_server.load("tiles/forest_4/lasiglasty.gltf");
    let mut transform = Transform::from_translation(Vec3::zero())
        .looking_at(Vec3::unit_x(), Vec3::unit_z());

    commands
        .spawn(Camera3dBundle {
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(0.0, -2.0, 25.0)); // TODO: Config file
                let rotation = Quat::from_rotation_x(0.1); // TODO: Same
                // let rotation = Quat::from_rotation_x(0.2);

                // * side view
                // let mut transform = Transform::from_translation(Vec3::new(0.0, -5.0, 0.0));
                // let rotation = Quat::from_rotation_x(3.14 / 2f32);

                transform.rotate(rotation);

                transform
            },
            .. Default::default()
        })
        .with(PickSource::default())
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        });
        // .spawn(PbrBundle {
        //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        //     material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.5)),
        //     ..Default::default()
        // });

    for hexagon_position in world.render() {
        transform.translation = hexagon_position;

        commands
            .spawn((transform.clone(), GlobalTransform::default()))
            .with_children(|parent| {
                parent.spawn(PbrBundle {
                    mesh: empty_hextile.clone(),
                    material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.19, 0.0)), // TODO: Config
                    ..Default::default()
                })
                .with(PickableMesh::default())
                .with(InteractableMesh::default())
                .with(HighlightablePickMesh::default())
                .with(SelectablePickMesh::default());
            })
            .with_children(|parent| {
                parent.spawn_scene(hextile.clone());
            });
    }
}
