use std::{cell::RefCell, rc::Rc};

use crate::base_work::App;

pub trait Scene {
    fn prepare_pipeline(&self, app: &App) -> Option<wgpu::RenderPipeline>;
    fn render_frame(&self, app: &App, target: &wgpu::TextureView);
    fn on_resize(&self, app: &App, width: u32, height: u32) {}
}

pub mod buffer_first_bind;
pub mod color_screen;
pub mod half_screen;
pub mod red_triangle;
pub struct SceneSelector {
    pub scenes: Vec<Box<dyn Scene>>,
    pub current_scene_index: usize,
    pub current_pipeline: Option<wgpu::RenderPipeline>,
}
impl SceneSelector {
    pub fn select(&mut self, index: usize) -> &Box<dyn Scene> {
        let scene = match self.scenes.get(index) {
            Some(scene) => {
                self.current_scene_index = index;
                scene
            }
            None => {
                self.current_scene_index = 0;
                self.scenes.get(0).unwrap()
            }
        };
        scene
    }
    pub fn get_current_scene(&self) -> &Box<dyn Scene> {
        self.scenes.get(self.current_scene_index).unwrap()
    }
    pub fn new() -> Self {
        SceneSelector {
            scenes: vec![
                Box::new(buffer_first_bind::Scene1 {
                    name: "buffer_first_bind".to_string(),
                    bind_group: RefCell::new(None),
                }),
                Box::new(red_triangle::Scene1 {
                    name: "red_triangle".to_string(),
                }),
                Box::new(half_screen::Scene1 {
                    name: "half_screen".to_string(),
                }),
                Box::new(color_screen::Scene1 {
                    name: "color_screen".to_string(),
                }),
            ],
            current_scene_index: 0,
            current_pipeline: None,
        }
    }
}
