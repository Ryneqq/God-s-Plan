use bevy::prelude::Vec3;
use std::f32::consts::PI;

/// Assumes that tailing is flat over Z (it is constant)
#[derive(Debug, Clone, Copy, Default)]
pub struct Hexagon {
    positon: Vec3,
}

impl Hexagon {
    /// New gives possiblity to add offset
    pub fn new(positon: Vec3) -> Self {
        Self { positon }
    }

    pub fn flat_vertices(self, edge: f32) -> impl Iterator<Item = Vec3> {
        (0..6).map(|i| PI / 180f32 * 60f32 * i as f32)
            .map(move |angle| {
                let x = self.positon.x + edge * angle.cos();
                let y = self.positon.y + edge * angle.sin();
                let z = self.positon.z;

                Vec3::new(x, y, z)
            })
    }

    pub fn world(self) -> Vec3 {
        self.positon
    }

    pub fn calculate_position(mut self, edge: f32, x: isize, _: isize, z: isize) -> Self {
        let col = x;
        let row = z + (x + (x & 1)) / 2;
        let height = edge * 3f32.sqrt() / 2f32;

        let shift = (col & 1) as f32 * height;
        let y = shift - row as f32 * height * 2f32 + self.positon.y;

        let shift = edge * 3f32 / 2f32;
        let x = col as f32 * shift + self.positon.x;

        self.positon = Vec3::new(x, y, self.positon.z);
        self
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
            position: &'static [f32; 3],
            edge: f32,
            world: &'static [f32; 3],
        }

        const TEST_1: TestCase = TestCase {
            cube: &[0, 0, 0],
            position: &[0f32, 0f32, 0f32],
            edge: 10f32,
            world: &[0f32, 0f32, 0f32],
        };

        const TEST_2: TestCase = TestCase {
            cube: &[0, -1, 1],
            position: &[0f32, 0f32, 0f32],
            edge: 10f32,
            world: &[0f32, -17.320507, 0f32],
        };

        const TEST_3: TestCase = TestCase {
            cube: &[0, -2, 2],
            position: &[0f32, 0f32, 0f32],
            edge: 10f32,
            world: &[0f32, -34.641014, 0f32],
        };

        const TEST_4: TestCase = TestCase {
            cube: &[1, -1, 0],
            position: &[0f32, 0f32, 0f32],
            edge: 10f32,
            world: &[15f32, -8.660254, 0f32],
        };

        const TEST_5: TestCase = TestCase {
            cube: &[-3, 3, 0],
            position: &[0f32, 0f32, 0f32],
            edge: 10f32,
            world: &[-45f32, 25.98076, 0f32],
        };

        const TEST_6: TestCase = TestCase {
            cube: &[0, 0, 0],
            position: &[1f32, 2f32, 3f32],
            edge: 10f32,
            world: &[1f32, 2f32, 3f32],
        };

        const TEST_7: TestCase = TestCase {
            cube: &[0, -1, 1],
            position: &[1f32, 2f32, 3f32],
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

        fn cube_coord_to_world_test(TestCase { cube, position, edge, world }: TestCase) {
            let [x, y, z] = cube.clone();
            let actual = Hexagon::new(position.clone().into())
                .calculate_position(edge, x, y, z)
                .world();

            assert_eq!(Vec3::from(world.clone()), actual);
        }
    }
}