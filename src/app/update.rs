use crate::Point;
use crate::Scene;
use bevy::prelude::*;
use itertools::Itertools;

pub fn update(
    mut scene: ResMut<Scene>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    node_query: Query<(&Point, &Handle<ColorMaterial>)>
) {
    node_query.iter()
        .sorted_by(|(x, _), (y, _)| scene.get_value(**y).cmp(&scene.get_value(**x)))
        .for_each(|(node, material)| {
            let mut material = materials.get_mut(material).unwrap();

        });
}
