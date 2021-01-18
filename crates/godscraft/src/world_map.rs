use bevy::prelude::Vec3;
use crate::Hexagon;
use itertools::Itertools;
use std::collections::HashMap;

type Position = (isize, isize, isize);

#[derive(Debug, Clone)]
pub struct WorldMap {
    edge: f32,
    world_map: HashMap<Position, Hexagon>,
}

impl WorldMap {
    pub fn new(edge: f32, size: isize) -> Self {
        let world_map = WorldMap::create_map(size, edge).collect();

        Self { edge, world_map }
    }

    pub fn render(&'_ self) -> impl Iterator<Item = Vec3> + '_ {
        self.world_map.values()
            .map(|hexagon| hexagon.world())
    }

    pub fn create_map(size: isize, edge: f32) -> impl Iterator<Item = (Position, Hexagon)> {
        let iter = -size..=size;

        iter.clone()
            .cartesian_product(iter.clone())
            .cartesian_product(iter)
            .filter_map(|((x, y), z)| if x + y + z == 0 { Some((x, y, z)) } else { None })
            .map(move |(x, y, z)| {
                let hexagon = Hexagon::default().calculate_position(edge, x, y, z);
                let position = (x, y, z);

                (position, hexagon)
            })
    }
}
