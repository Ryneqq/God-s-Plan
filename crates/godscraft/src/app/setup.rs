use bevy::prelude::*;
use bevy_mod_picking::*;
use crate::WorldMap;
use super::config::{Config, Tiles};

pub (super) fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world: Res<WorldMap>,
    config: Res<Config>,
) {
    let empty_hextile = asset_server.load(config.world_map.get_mesh_path(Tiles::Highlight).unwrap());
    let hextile = asset_server.load(config.world_map.get_mesh_path(Tiles::Forest).unwrap());

    commands
        .spawn(Camera3dBundle {
            transform: {
                let mut transform = Transform::from_translation(config.camera_position());
                let rotation = Quat::from_rotation_x(config.camera_angle());

                transform.rotate(rotation);

                transform
            },
            .. Default::default()
        })
        .with(PickSource::default())
        .spawn(LightBundle {
            transform: Transform::from_translation(config.light_position()),
            light: config.light(),
            ..Default::default()
        });

    let mut transform = Transform::from_translation(Vec3::zero())
        .looking_at(Vec3::unit_x(), Vec3::unit_z());

    // TODO: could it be `spawn_batch`?
    for hexagon_position in world.render() {
        transform.translation = hexagon_position;

        commands
            .spawn((transform.clone(), GlobalTransform::default()))
            .with_children(|parent| {
                parent.spawn(PbrBundle {
                    mesh: empty_hextile.clone(),
                    material: materials.add(config.highlight_default_color().into()),
                    transform: Transform::from_translation(config.highlight_tile_position()),
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
