use alloc::boxed::Box;

use shared::Shared;
use scene_graph::{Scene, Component, ComponentManager, Id};
use camera::Camera;


struct CameraManagerData {
    scene: Option<Scene>,
    active_camera: Option<Camera>,
    components: usize,
}


#[derive(Clone)]
pub struct CameraManager {
    data: Shared<CameraManagerData>,
}

impl CameraManager {

    pub fn new() -> CameraManager {
        CameraManager {
            data: Shared::new(CameraManagerData {
                scene: None,
                active_camera: None,
                components: 0usize,
            })
        }
    }

    pub fn set_active_camera(&mut self, camera: &mut Camera) -> &Self {
        if let Some(ref mut active_camera) = self.data.active_camera {
            active_camera.__set_active(false);
        }

        camera.__set_active(true);
        self.data.active_camera = Some(camera.clone());

        self
    }
    pub fn get_active_camera(&self) -> Option<Camera> {
        match self.data.active_camera {
            Some(ref active_camera) => Some(active_camera.clone()),
            None => None,
        }
    }
}

impl ComponentManager for CameraManager {

    fn get_id(&self) -> Id { Id::of::<CameraManager>() }

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
        let ref mut component = component.downcast_mut::<Camera>().unwrap();

        component.set_camera_manager(Some(self.clone()));

        if component.active() {
            self.set_active_camera(component);
        }

        self.data.components += 1;
    }
    fn remove_component(&mut self, component: &mut Box<Component>) {
        let mut component = component.downcast_mut::<Camera>().unwrap();
        self.data.components -= 1;
        component.set_camera_manager(None);
    }
}
