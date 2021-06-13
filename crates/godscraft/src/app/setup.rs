use bevy::prelude::*;
use bevy_mod_picking::{PickingCameraBundle, PickableBundle};
use crate::WorldMap;
use super::config::{Config, Tiles};

pub (super) fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut highlight_params: ResMut<PickHighlightParams>,
    world: Res<WorldMap>,
    config: Res<Config>,
) {
    let empty_hextile = asset_server.load(config.world_map.get_mesh_path(Tiles::Highlight).unwrap());
    let forest_hextile = asset_server.load(config.world_map.get_mesh_path(Tiles::Forest).unwrap());
    let grassland_hextile = asset_server.load(config.world_map.get_mesh_path(Tiles::Grassland).unwrap());

    // highlight_params.set_hover_color(Color::rgb(1.0, 0.0, 0.0));
    // highlight_params.set_selection_color(Color::rgb(0.0, 1.0, 0.0));

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: {
                let mut transform = Transform::from_translation(config.camera_position());
                let rotation = Quat::from_rotation_x(config.camera_angle());

                transform.rotate(rotation);

                transform
            },
            .. Default::default()
        })
        .insert_bundle(PickingCameraBundle::default());

    for (pos, light) in config.lights() {
        commands.spawn_bundle(LightBundle {
            light,
            transform: Transform::from_translation(pos),
            ..Default::default()
        });
    }


    let mut transform = Transform::from_translation(Vec3::zero())
        .looking_at(Vec3::unit_x(), Vec3::unit_z());

    // TODO: could it be `spawn_batch`?
    for hexagon_position in world.render() {
        transform.translation = hexagon_position;

        commands
            .spawn_bundle((transform.clone(), GlobalTransform::default()))
            .with_children(|parent| {
                parent.spawn_bundle(PbrBundle {
                    mesh: empty_hextile.clone(),
                    material: materials.add(config.highlight_default_color().into()),
                    transform: Transform::from_translation(config.highlight_tile_position()),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default());
            })
            .with_children(|parent| {
                if rand::random() {
                    parent.spawn_scene(forest_hextile.clone());
                } else {
                    parent.spawn_scene(grassland_hextile.clone());
                }
            });
    }
}
