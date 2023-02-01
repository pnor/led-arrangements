use kiss3d::{
    camera::{ArcBall, Camera},
    light::Light,
    nalgebra::{OPoint, Point3, Translation3},
    scene::SceneNode,
    window::Window,
};

use super::{LightConfig, LightStrip};
use crate::{
    arrangement::{Arrangement, ArrangementConfig},
    color::Color,
};

const SPHERE_SIZE: f32 = 0.05;
const CENTER_POINT: (f32, f32, f32) = (0.5, 0.5, 0.5);
const CAMERA_START: (f32, f32, f32) = (2.0, 0.5, 2.0);

pub struct TestStrip {
    lights: Vec<(u8, u8, u8)>,
    // #[cfg(feature = "visualizer")]
    window: Window,
    objects: Vec<SceneNode>,
    camera: ArcBall,
}

impl TestStrip {
    pub fn new<const N: usize>(
        arrangement_info: &ArrangementConfig<N>,
        dimension_mask: &[u8; 3],
    ) -> Self {
        let lights = vec![(0, 0, 0); arrangement_info.light_locations.len()];
        let mut window = Window::new("Demo");
        let camera = ArcBall::new(
            Point3::new(CAMERA_START.0, CAMERA_START.1, CAMERA_START.2),
            Point3::new(CENTER_POINT.0, CENTER_POINT.1, CENTER_POINT.2),
        );

        let mut objects: Vec<SceneNode> = vec![];
        for i in 0..arrangement_info.light_locations.len() {
            objects.push(window.add_sphere(SPHERE_SIZE));
            let pt = coord_to_3d_cord(&arrangement_info.light_locations[i].0, &dimension_mask);
            objects[i].prepend_to_local_translation(&Translation3::new(pt[0], pt[1], pt[2]));
        }

        return Self {
            lights,
            window,
            objects,
            camera,
        };
    }
}

impl LightStrip for TestStrip {
    fn get(&self, index: usize) -> Color {
        let (r, g, b) = &self.lights[index];
        return Color::rgb(*r, *g, *b);
    }

    fn set(&mut self, index: usize, color: &Color) {
        self.lights[index] = (color.red, color.green, color.blue);
        let (r, g, b) = color.float_components();
        self.objects[index].set_color(r, g, b);
    }

    fn show(&mut self) {
        self.window.render_with_camera(&mut self.camera);
    }

    fn fill(&mut self, color: &Color) {
        self.lights.iter_mut().for_each(|raw| {
            raw.0 = color.red;
            raw.1 = color.green;
            raw.2 = color.blue;
        });
    }
}

/// Converts `point` to a 3 dimensional represnetation of the same point.
/// If point is under 3 dimensions, will pad missing dimensions to 0. If point is greater than 0
/// dimensions, will use `dimension_mask` to select which dimensions to use
///
/// `dimension_mask` is an array of 3 integers represneting the dimension to use in place. If a
/// dimension in the mask is not present in the point, will replace it with a 0 value
///
/// Example:
/// (point = [0.1, 0.9], dimension_mask = [0, 1, 2]) -> [0.1, 0.9, 0.0]
/// (point = [0.1, 0.2, 0.3], dimension_mask = [0, 1, 2]) -> [0.1, 0.2, 0.3]
/// (point = [0.1, 0.2, 0.3], dimension_mask = [2, 0, 1]) -> [0.3, 0.1, 0.2]
/// (point = [0.1, 0.2, 0.3, 0.4, 0.5], dimension_mask = [1, 3, 4]) -> [0.2, 0.4, 0.5]
fn coord_to_3d_cord<const N: usize>(point: &[f64; N], dimension_mask: &[u8; 3]) -> [f32; 3] {
    let mut output: [f32; 3] = [0.0; 3];
    for (i, d) in dimension_mask.iter().enumerate() {
        if *d as usize >= N {
            output[i] = 0.0
        } else {
            output[i] = point[*d as usize] as f32;
        }
    }
    return output;
}
