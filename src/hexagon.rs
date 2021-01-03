use bevy::prelude::Vec3;
use std::f32::consts::PI;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct Hexagon {
    edge: f32,
    center: Vec3,
}

impl Hexagon {
    pub fn new(edge: f32, center: Vec3) -> Self {
        Self { edge, center }
    }

    /// Assumes that tailing is flat over Z (it is constant)
    pub fn flat_vertices(self) -> impl Iterator<Item = Vec3> {
        (0..6).map(|i| PI / 180f32 * 60f32 * i as f32)
            .map(move |angle| {
                let x = self.center.x + self.edge * angle.cos();
                let y = self.center.y + self.edge * angle.sin();
                let z = self.center.z;

                Vec3::new(x, y, z)
            })
    }

    pub fn world(self) -> Vec3 {
        self.center
    }

    pub fn neighbors(self, distance: isize) -> impl Iterator<Item = Self> {
        let iter = -distance..=distance;

        iter.clone()
            .cartesian_product(iter.clone())
            .cartesian_product(iter)
            .filter_map(|((x, y), z)| if x + y + z == 0 { Some((x, y, z)) } else { None })
            .map(move |(x, y, z)| {
                let edge = self.edge;
                let center = self.cube_coord_to_world(x, y, z);

                Hexagon {
                    edge,
                    center,
                }
            })
    }

    fn cube_coord_to_world(&self, x: isize, _: isize, z: isize) -> Vec3 {
        let col = x;
        let row = z + (x + (x & 1)) / 2;
        let height = self.edge * 3f32.sqrt() / 2f32;

        let shift = (col & 1) as f32 * height;
        let y = shift - row as f32 * height * 2f32 + self.center.y;

        let shift = self.edge * 3f32 / 2f32;
        let x = col as f32 * shift + self.center.x;

        Vec3::new(x, y, self.center.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod cube_to_world {
        use super::*;
        use test_case::test_case;

        struct TestCase {
            cube: &'static [isize; 3],
            center: &'static [f32; 3],
            edge: f32,
            world: &'static [f32; 3],
        }

        const TEST_1: TestCase = TestCase {
            cube: &[0, 0, 0],
            center: &[0f32, 0f32, 0f32],
            edge: 10f32,
            world: &[0f32, 0f32, 0f32],
        };

        const TEST_2: TestCase = TestCase {
            cube: &[0, -1, 1],
            center: &[0f32, 0f32, 0f32],
            edge: 10f32,
            world: &[0f32, -17.320507, 0f32],
        };

        const TEST_3: TestCase = TestCase {
            cube: &[0, -2, 2],
            center: &[0f32, 0f32, 0f32],
            edge: 10f32,
            world: &[0f32, -34.641014, 0f32],
        };

        const TEST_4: TestCase = TestCase {
            cube: &[1, -1, 0],
            center: &[0f32, 0f32, 0f32],
            edge: 10f32,
            world: &[15f32, -8.660254, 0f32],
        };

        const TEST_5: TestCase = TestCase {
            cube: &[-3, 3, 0],
            center: &[0f32, 0f32, 0f32],
            edge: 10f32,
            world: &[-45f32, 25.98076, 0f32],
        };

        const TEST_6: TestCase = TestCase {
            cube: &[0, 0, 0],
            center: &[1f32, 2f32, 3f32],
            edge: 10f32,
            world: &[1f32, 2f32, 3f32],
        };

        const TEST_7: TestCase = TestCase {
            cube: &[0, -1, 1],
            center: &[1f32, 2f32, 3f32],
            edge: 10f32,
            world: &[1f32, -15.320507, 3f32],
        };

        #[test_case(TEST_1)]
        #[test_case(TEST_2)]
        #[test_case(TEST_3)]
        #[test_case(TEST_4)]
        #[test_case(TEST_5)]
        #[test_case(TEST_6)]
        #[test_case(TEST_7)]

        fn cube_coord_to_world_test(TestCase { cube, center, edge, world }: TestCase) {
            let target = Hexagon::new(edge, center.clone().into());
            let [x, y, z] = cube.clone();
            let actual = target.cube_coord_to_world(x, y, z);

            assert_eq!(Vec3::from(world.clone()), actual);
        }
    }
}