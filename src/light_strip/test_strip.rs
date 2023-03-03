#[cfg(feature = "visualizer")]
use kiss3d::{
    camera::ArcBall,
    nalgebra::{Point3, Translation3},
    scene::SceneNode,
    window::Window,
};

use super::LightStrip;
use crate::{arrangement::ArrangementConfig, color::Color};

pub struct TestStrip {
    lights: Vec<(u8, u8, u8)>,
    #[cfg(feature = "visualizer")]
    window: Window,
    #[cfg(feature = "visualizer")]
    objects: Vec<SceneNode>,
    #[cfg(feature = "visualizer")]
    camera: ArcBall,
}

impl TestStrip {
    #[cfg(feature = "visualizer")]
    pub fn new<const N: usize>(
        arrangement_config: &ArrangementConfig<N>,
        display_config: &TestStripDisplayConfig,
    ) -> Self {
        let lights = vec![(0, 0, 0); arrangement_config.light_locations.len()];
        let mut window = Window::new("Demo");
        let camera_start = &display_config.camera_start;
        let center_point = &display_config.center_point;
        let camera = ArcBall::new(
            Point3::new(camera_start.0, camera_start.1, camera_start.2),
            Point3::new(center_point.0, center_point.1, center_point.2),
        );

        let mut objects: Vec<SceneNode> = vec![];
        for i in 0..arrangement_config.light_locations.len() {
            objects.push(window.add_sphere(display_config.sphere_size));
            let pt = coord_to_3d_cord(
                &arrangement_config.light_locations[i].0,
                &display_config.dimension_mask,
            );
            objects[i].prepend_to_local_translation(&Translation3::new(pt[0], pt[1], pt[2]));
        }

        return Self {
            lights,
            window,
            objects,
            camera,
        };
    }

    #[cfg(not(feature = "visualizer"))]
    pub fn new<const N: usize>(
        arrangement_config: &ArrangementConfig<N>,
        display_config: &TestStripDisplayConfig,
    ) -> Self {
        let lights = vec![(0, 0, 0); arrangement_config.light_locations.len()];
        return Self { lights };
    }
}

impl LightStrip for TestStrip {
    #[inline]
    fn get(&self, index: usize) -> Color {
        let (r, g, b) = &self.lights[index];
        return Color::rgb(*r, *g, *b);
    }

    #[inline]
    fn set(&mut self, index: usize, color: &Color) {
        self.lights[index] = (color.red, color.green, color.blue);
        #[cfg(feature = "visualizer")]
        let (r, g, b) = color.float_components();
        #[cfg(feature = "visualizer")]
        self.objects[index].set_color(r, g, b);
    }

    #[cfg(feature = "visualizer")]
    fn show(&mut self) {
        self.window.render_with_camera(&mut self.camera);
    }

    #[cfg(not(feature = "visualizer"))]
    fn show(&mut self) {}

    #[cfg(not(feature = "visualizer"))]
    fn fill(&mut self, color: &Color) {
        self.lights.iter_mut().for_each(|raw| {
            raw.0 = color.red;
            raw.1 = color.green;
            raw.2 = color.blue;
        });
    }

    #[cfg(feature = "visualizer")]
    fn fill(&mut self, color: &Color) {
        self.lights.iter_mut().for_each(|raw| {
            raw.0 = color.red;
            raw.1 = color.green;
            raw.2 = color.blue;
        });
        let (r, g, b) = color.float_components();
        self.objects.iter_mut().for_each(|obj| {
            obj.set_color(r, g, b);
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

pub struct TestStripDisplayConfig {
    sphere_size: f32,
    center_point: (f32, f32, f32),
    camera_start: (f32, f32, f32),
    dimension_mask: [u8; 3],
}

impl TestStripDisplayConfig {
    pub fn default() -> Self {
        TestStripDisplayConfig {
            sphere_size: 0.02,
            center_point: (0.5, 0.5, 0.5),
            camera_start: (2.0, 0.5, 2.0),
            dimension_mask: [0, 1, 2],
        }
    }

    pub fn new(sphere_size: f32, camera_start: (f32, f32, f32), dimension_mask: [u8; 3]) -> Self {
        TestStripDisplayConfig {
            sphere_size,
            center_point: (0.5, 0.5, 0.5),
            camera_start,
            dimension_mask,
        }
    }
}
