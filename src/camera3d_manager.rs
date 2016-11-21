use alloc::boxed::Box;

use shared::Shared;
use scene_graph::{Scene, Component, ComponentManager, Id};

use camera3d::Camera3D;


struct Camera3DManagerData {
    scene: Option<Scene>,
    active_camera: Option<Camera3D>,
    components: usize,
}


#[derive(Clone)]
pub struct Camera3DManager {
    data: Shared<Camera3DManagerData>,
}

impl Camera3DManager {

    pub fn new() -> Camera3DManager {
        Camera3DManager {
            data: Shared::new(Camera3DManagerData {
                scene: None,
                active_camera: None,
                components: 0usize,
            })
        }
    }

    pub fn set_active_camera(&mut self, camera: &mut Camera3D) -> &Self {
        if let Some(ref mut active_camera) = self.data.active_camera {
            active_camera.__set_active(false);
        }

        camera.__set_active(true);
        self.data.active_camera = Some(camera.clone());

        self
    }
    pub fn get_active_camera(&self) -> Option<Camera3D> {
        match self.data.active_camera {
            Some(ref active_camera) => Some(active_camera.clone()),
            None => None,
        }
    }
    pub fn has_active_camera(&self) -> bool {
        self.data.active_camera.is_some()
    }
}

impl ComponentManager for Camera3DManager {

    fn get_id(&self) -> Id { Id::of::<Camera3DManager>() }

    fn get_scene(&self) -> Option<Scene> {
        match self.data.scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    fn set_scene(&mut self, scene: Option<Scene>) {
        self.data.scene = scene;
    }

    fn get_order(&self) -> usize { 0 }
    fn is_empty(&self) -> bool {
        self.data.components == 0usize
    }

    fn clear(&mut self) {}
    fn init(&mut self) {}
    fn update(&mut self) {}

    fn add_component(&mut self, component: &mut Box<Component>) {
        let ref mut component = component.downcast_mut::<Camera3D>().unwrap();

        component.__set_manager(Some(self.clone()));

        if component.active() || !self.has_active_camera() {
            self.set_active_camera(component);
        }

        self.data.components += 1;
    }
    fn remove_component(&mut self, component: &mut Box<Component>) {
        let mut component = component.downcast_mut::<Camera3D>().unwrap();

        self.data.components -= 1;

        if component.active() {
            component.__set_manager(None);
        }
    }
}
