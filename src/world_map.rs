use bevy::prelude::Vec3;
use crate::Hexagon;

#[derive(Debug, Clone)]
pub struct WorldMap {
    edge: f32,
    normal: f32,
    hexagon: Hexagon
}

impl WorldMap {
    pub fn new(edge: f32) -> Self {
        let normal = edge as f32 * 3f32.sqrt() / 2f32;
        let hexagon = Hexagon::new(edge, Vec3::zero());

        Self { edge, normal, hexagon }
    }

    pub fn render(&'_ self) -> impl Iterator<Item = Vec3> + '_ {
        self.hexagon
            .neighbors(4)
            .map(|hexagon| hexagon.world())
    }
}
