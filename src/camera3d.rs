use alloc::boxed::Box;

use shared::Shared;
use mat4;
use to_radians::ToRadians;
use transform_components::{Transform2D, Transform3D};
use scene_graph::{Entity, Component, ComponentManager, Id};

use camera3d_manager::Camera3DManager;


struct Camera3DData {

    entity: Option<Entity>,
    camera_manager: Option<Camera3DManager>,

    width: usize,
    height: usize,
    inv_width: f32,
    inv_height: f32,

    auto_resize: bool,
    background: [f32; 4],

    aspect: f32,
    fov: f32,

    orthographic_mode: bool,
    orthographic_size: f32,

    near: f32,
    far: f32,

    projection: [f32; 16],
    view: [f32; 16],

    needs_update: bool,
    active: bool,
}


#[derive(Clone)]
pub struct Camera3D {
    data: Shared<Camera3DData>,
}

impl Camera3D {
    pub fn new() -> Self {
        Camera3D {
            data: Shared::new(Camera3DData {

                entity: None,
                camera_manager: None,

                width: 512usize,
                height: 512usize,
                inv_width: 1f32 / 512f32,
                inv_height: 1f32 / 512f32,

                auto_resize: true,
                background: [0f32, 0f32, 0f32, 1f32],

                aspect: 1f32,
                fov: 35f32,

                orthographic_mode: false,
                orthographic_size: 2f32,

                near: 0.001f32,
                far: 1000f32,

                projection: mat4::new_identity(),
                view: mat4::new_identity(),

                needs_update: true,
                active: true,
            })
        }
    }

    pub fn get_manager(&self) -> Option<Camera3DManager> {
        self.data.camera_manager.clone()
    }
    pub fn __set_manager(&mut self, camera_manager: Option<Camera3DManager>) {
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

    pub fn set_fov(&mut self, fov: f32) -> &mut Self {
        {
            let ref mut data = self.data;
            data.fov = fov;
            data.needs_update = true;
        }
        self
    }
    pub fn get_fov(&self) -> f32 {
        self.data.fov
    }

    pub fn set_near(&mut self, near: f32) -> &mut Self {
        {
            let ref mut data = self.data;
            if near >= 0.0001f32 {
                data.near = near;
            }
            data.needs_update = true;
        }
        self
    }
    pub fn get_near(&self) -> f32 {
        self.data.near
    }

    pub fn set_far(&mut self, far: f32) -> &mut Self {
        {
            let ref mut data = self.data;
            data.far = far;
            data.needs_update = true;
        }
        self
    }
    pub fn get_far(&self) -> f32 {
        self.data.far
    }

    pub fn set_orthographic_mode(&mut self, orthographic_mode: bool) -> &mut Self {
        {
            let ref mut data = self.data;
            data.orthographic_mode = orthographic_mode;
            data.needs_update = true;
        }
        self
    }
    pub fn get_orthographic_mode(&self) -> bool {
        self.data.orthographic_mode
    }

    pub fn set_orthographic_size(&mut self, orthographic_size: f32) -> &mut Self {
        {
            let ref mut data = self.data;
            data.orthographic_size = if orthographic_size > 0.0001f32 {orthographic_size} else {0.0001f32};
            data.needs_update = true;
        }
        self
    }
    pub fn get_orthographic_size(&self) -> f32 {
        self.data.orthographic_size
    }

    pub fn get_view(&mut self) -> &[f32; 16] {
        let world_matrix = self.get_world_matrix();

        if world_matrix.is_some() {
            mat4::inverse(&mut self.data.view, &world_matrix.unwrap());
        } else {
            mat4::identity(&mut self.data.view);
        }
        &self.data.view
    }
    fn get_world_matrix(&self) -> Option<[f32; 16]> {
        if let Some(entity) = self.get_entity() {
            if let Some(ref mut transform3d) = entity.get_component::<Transform3D>() {
                Some(*(transform3d.get_world_matrix()))
            } else if let Some(ref mut transform2d) = entity.get_component::<Transform2D>() {
                Some(transform2d.get_world_matrix())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_projection(&mut self) -> &[f32; 16] {
        if self.data.needs_update {
            self.update_projection();
        }
        &self.data.projection
    }
    fn update_projection(&mut self) {
        if self.get_orthographic_mode() {
            let ref mut data = self.data;

            let orthographic_size = data.orthographic_size;
            let right = orthographic_size * data.aspect;
            let left = -right;
            let top = orthographic_size;
            let bottom = -top;
            let near = data.near;
            let far = data.far;

            mat4::orthographic(&mut data.projection, left, right, top, bottom, near, far);
        } else {
            let ref mut data = self.data;

            let fov = data.fov;
            let aspect = data.aspect;
            let near = data.near;
            let far = data.far;

            mat4::perspective(&mut data.projection, fov.to_radians(), aspect, near, far);
        }
    }
}

impl Component for Camera3D {
    fn get_id(&self) -> Id {
        Id::of::<Camera3D>()
    }
    fn new_component_manager(&self) -> Box<ComponentManager> {
        Box::new(Camera3DManager::new())
    }
    fn get_component_manager_id(&self) -> Id {
        Id::of::<Camera3DManager>()
    }
    fn get_entity(&self) -> Option<Entity> {
        self.data.entity.clone()
    }
    fn set_entity(&mut self, entity: Option<Entity>) {
        self.data.entity = entity;
    }
}

impl PartialEq<Camera3D> for Camera3D {
    fn eq(&self, other: &Camera3D) -> bool {
        (&*self.data as *const _) == (&*other.data as *const _)
    }
    fn ne(&self, other: &Camera3D) -> bool {
        !self.eq(other)
    }
}
