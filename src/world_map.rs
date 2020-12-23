use bevy::prelude::Vec3;

const CENTER: Vec3 = Vec3::zero();

#[derive(Debug, Clone)]
pub struct WorldMap {
    edge: usize,
}

impl WorldMap {
    pub fn new(edge: usize) -> Self {
        Self { edge }
    }

    pub fn offset(&self, point: Vec3) -> Vec3 {
        let x = point.x as usize % self.edge;
        let y = point.y as usize % self.edge;

        unimplemented!() // TODO
    }
}
