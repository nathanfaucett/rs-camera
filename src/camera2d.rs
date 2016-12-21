use alloc::boxed::Box;

use core::f32::EPSILON;

use shared::Shared;
use mat32;
use transform_components::{Transform2D, Transform3D};
use scene_graph::{Entity, Component, ComponentManager, Id};

use camera2d_manager::Camera2DManager;


struct Camera2DData {

    entity: Option<Entity>,
    camera_manager: Option<Camera2DManager>,

    width: usize,
    height: usize,
    inv_width: f32,
    inv_height: f32,

    aspect: f32,

    auto_resize: bool,
    background: [f32; 4],

    orthographic_size: f32,

    projection: [f32; 6],
    view: [f32; 6],

    needs_update: bool,
    active: bool,
}


#[derive(Clone)]
pub struct Camera2D {
    data: Shared<Camera2DData>,
}

impl Camera2D {
    pub fn new() -> Self {
        Camera2D {
            data: Shared::new(Camera2DData {

                entity: None,
                camera_manager: None,

                width: 512usize,
                height: 512usize,
                inv_width: 1f32 / 512f32,
                inv_height: 1f32 / 512f32,

                auto_resize: true,
                background: [0f32, 0f32, 0f32, 1f32],

                aspect: 1f32,

                orthographic_size: 2f32,

                projection: mat32::new_identity(),
                view: mat32::new_identity(),

                needs_update: true,
                active: true,
            })
        }
    }

    pub fn get_manager(&self) -> Option<Camera2DManager> {
        self.data.camera_manager.clone()
    }
    pub fn __set_manager(&mut self, camera_manager: Option<Camera2DManager>) {
        self.data.camera_manager = camera_manager;
    }

    pub fn set(&mut self, width: usize, height: usize) -> &mut Self {
        {
            let ref mut data = self.data;
            let fwidth = width as f32;
            let fheight = height as f32;

            data.width = width;
            data.height = height;

            data.inv_width = 1f32 / fwidth;
            data.inv_height = 1f32 / fheight;

            data.aspect = fwidth / fheight;
            data.needs_update = true;
        }
        self
    }

    pub fn __set_active(&mut self, active: bool) {
        self.data.active = active;
    }

    pub fn active(&self) -> bool {
        self.data.active
    }
    pub fn set_active(&mut self) -> &Self {
        if let Some(ref mut camera_manager) = self.get_manager() {
            camera_manager.set_active_camera(self);
        } else {
            self.data.active = true;
        }
        self
    }

    pub fn get_auto_resize(&self) -> bool {
        self.data.auto_resize
    }
    pub fn set_auto_resize(&mut self, auto_resize: bool) {
        self.data.auto_resize = auto_resize;
    }

    pub fn get_background(&self) -> &[f32; 4] {
        &self.data.background
    }
    pub fn set_background(&mut self, background: &[f32; 4]) {
        self.data.background = *background;
    }

    pub fn set_width(&mut self, width: usize) -> &mut Self {
        {
            let ref mut data = self.data;
            let fwidth = width as f32;

            data.width = width;
            data.inv_width = 1f32 / fwidth;
            data.aspect = fwidth / data.height as f32;
            data.needs_update = true;
        }
        self
    }
    pub fn set_height(&mut self, height: usize) -> &mut Self {
        {
            let ref mut data = self.data;
            let fheight = height as f32;

            data.height = height;
            data.inv_height = 1f32 / fheight;
            data.aspect = data.width as f32 / fheight;
            data.needs_update = true;
        }
        self
    }

    pub fn get_width(&self) -> usize {
        self.data.width
    }
    pub fn get_height(&self) -> usize {
        self.data.height
    }

    pub fn set_orthographic_size(&mut self, orthographic_size: f32) -> &mut Self {
        {
            let ref mut data = self.data;
            data.orthographic_size = if orthographic_size > 0f32 {orthographic_size} else {EPSILON};
            data.needs_update = true;
        }
        self
    }
    pub fn get_orthographic_size(&self) -> f32 {
        self.data.orthographic_size
    }

    pub fn get_view(&mut self) -> &[f32; 6] {
        let world_matrix = self.get_world_matrix();

        if let Some(m) = world_matrix {
            mat32::inverse(&mut self.data.view, &m);
        } else {
            mat32::identity(&mut self.data.view);
        }

        &self.data.view
    }
    fn get_world_matrix(&self) -> Option<[f32; 6]> {
        if let Some(entity) = self.get_entity() {
            if let Some(ref mut transform2d) = entity.get_component::<Transform2D>() {
                Some(*transform2d.get_matrix())
            } else if let Some(ref mut transform3d) = entity.get_component::<Transform3D>() {
                let mut m = mat32::new_identity();
                mat32::from_mat4(&mut m, transform3d.get_matrix());
                Some(m)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_projection(&mut self) -> &[f32; 6] {
        if self.data.needs_update {
            self.update_projection();
        }
        &self.data.projection
    }
    fn update_projection(&mut self) {
        let ref mut data = self.data;

        let orthographic_size = data.orthographic_size;
        let right = orthographic_size * data.aspect;
        let left = -right;
        let top = orthographic_size;
        let bottom = -top;

        mat32::orthographic(&mut data.projection, top, right, bottom, left);
    }
}

impl Component for Camera2D {
    fn get_id(&self) -> Id {
        Id::of::<Camera2D>()
    }
    fn new_component_manager(&self) -> Box<ComponentManager> {
        Box::new(Camera2DManager::new())
    }
    fn get_component_manager_id(&self) -> Id {
        Id::of::<Camera2DManager>()
    }
    fn get_entity(&self) -> Option<Entity> {
        self.data.entity.clone()
    }
    fn set_entity(&mut self, entity: Option<Entity>) {
        self.data.entity = entity;
    }
}

impl PartialEq<Camera2D> for Camera2D {
    fn eq(&self, other: &Camera2D) -> bool {
        (&*self.data as *const _) == (&*other.data as *const _)
    }
    fn ne(&self, other: &Camera2D) -> bool {
        !self.eq(other)
    }
}
