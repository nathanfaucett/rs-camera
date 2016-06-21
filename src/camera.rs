use collections::boxed::Box;
use alloc::arc::Arc;
use core::cell::RefCell;
use core::f32::EPSILON;
use core::f32::consts::PI;

use mat4;
use transform2d::Transform2D;
use scene_graph::{Entity, Component, ComponentManager, Id};
use camera_manager::CameraManager;


static TO_RADS: f32 = PI / 180f32;


struct CameraData {

    entity: Option<Entity>,
    camera_manager: Option<CameraManager>,

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
pub struct Camera {
    data: Arc<RefCell<CameraData>>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            data: Arc::new(RefCell::new(CameraData {

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

                near: EPSILON,
                far: 1024f32,

                projection: mat4::new_identity(),
                view: mat4::new_identity(),

                needs_update: true,
                active: false,
            }))
        }
    }

    pub fn camera_manager(&self) -> Option<CameraManager> {
        self.data.borrow().camera_manager.clone()
    }
    pub fn set_camera_manager(&self, camera_manager: Option<CameraManager>) {
        self.data.borrow_mut().camera_manager = camera_manager;
    }

    pub fn set(&self, width: usize, height: usize) -> &Self {
        let mut data = self.data.borrow_mut();
        let fwidth = width as f32;
        let fheight = height as f32;

        data.width = width;
        data.height = height;

        data.inv_width = 1f32 / fwidth;
        data.inv_height = 1f32 / fheight;

        data.aspect = fwidth / fheight;
        data.needs_update = true;

        self
    }

    pub fn __set_active(&self, active: bool) {
        self.data.borrow_mut().active = active;
    }

    pub fn set_active(&self) -> &Self {
        if let Some(camera_manager) = self.camera_manager() {
            camera_manager.set_active_camera(self.clone());
        } else {
            self.data.borrow_mut().active = true;
        }
        self
    }
    pub fn active(&self) -> bool {
        self.data.borrow().active
    }

    pub fn auto_resize(&self) -> bool {
        self.data.borrow().auto_resize
    }
    pub fn set_auto_resize(&self, auto_resize: bool) {
        self.data.borrow_mut().auto_resize = auto_resize;
    }

    pub fn background(&self) -> [f32; 4] {
        self.data.borrow().background
    }
    pub fn set_background(&self, background: [f32; 4]) {
        self.data.borrow_mut().background = background;
    }

    pub fn set_width(&self, width: usize) -> &Self {
        let mut data = self.data.borrow_mut();
        let fwidth = width as f32;

        data.width = width;
        data.inv_width = 1f32 / fwidth;
        data.aspect = fwidth / data.height as f32;
        data.needs_update = true;

        self
    }
    pub fn set_height(&self, height: usize) -> &Self {
        let mut data = self.data.borrow_mut();
        let fheight = height as f32;

        data.height = height;
        data.inv_height = 1f32 / fheight;
        data.aspect = fheight / data.height as f32;
        data.needs_update = true;

        self
    }

    pub fn width(&self) -> usize {
        self.data.borrow().width
    }
    pub fn height(&self) -> usize {
        self.data.borrow().height
    }

    pub fn set_fov(&self, fov: f32) -> &Self {
        let mut data = self.data.borrow_mut();
        data.fov = fov;
        data.needs_update = true;
        self
    }
    pub fn fov(&self) -> f32 {
        self.data.borrow().fov
    }

    pub fn set_near(&self, near: f32) -> &Self {
        let mut data = self.data.borrow_mut();
        data.near = near;
        data.needs_update = true;
        self
    }
    pub fn near(&self) -> f32 {
        self.data.borrow().near
    }

    pub fn set_far(&self, far: f32) -> &Self {
        let mut data = self.data.borrow_mut();
        data.far = far;
        data.needs_update = true;
        self
    }
    pub fn far(&self) -> f32 {
        self.data.borrow().far
    }

    pub fn set_orthographic_mode(&self, orthographic_mode: bool) -> &Self {
        let mut data = self.data.borrow_mut();
        data.orthographic_mode = orthographic_mode;
        data.needs_update = true;
        self
    }
    pub fn orthographic_mode(&self) -> bool {
        self.data.borrow().orthographic_mode
    }

    pub fn set_orthographic_size(&self, orthographic_size: f32) -> &Self {
        let mut data = self.data.borrow_mut();
        data.orthographic_size = if orthographic_size > 0f32 {orthographic_size} else {EPSILON};
        data.needs_update = true;
        self
    }
    pub fn orthographic_size(&self) -> f32 {
        self.data.borrow().orthographic_size
    }

    pub fn view(&self) -> [f32; 16] {
        self.update_view();
        self.data.borrow().view
    }
    fn update_view(&self) {
        if let Some(entity) = self.entity() {
            if let Some(transform2d) = entity.get_component::<Transform2D>() {
                let mut data = self.data.borrow_mut();
                mat4::inverse(&mut data.view, mat4::from_mat32(transform2d.matrix()));
            }
        }
    }

    pub fn projection(&self) -> [f32; 16] {
        if self.data.borrow().needs_update {
            self.update_projection();
        }
        self.data.borrow().projection
    }
    fn update_projection(&self) {
        if self.orthographic_mode() {
            let mut data = self.data.borrow_mut();

            let orthographic_size = data.orthographic_size;
            let right = orthographic_size * data.aspect;
            let left = -right;
            let top = orthographic_size;
            let bottom = -top;
            let near = data.near;
            let far = data.far;

            mat4::orthographic(&mut data.projection, left, right, top, bottom, near, far);
        } else {
            let mut data = self.data.borrow_mut();
            let fov = data.fov;
            let aspect = data.aspect;
            let near = data.near;
            let far = data.far;
            mat4::perspective(&mut data.projection, fov * TO_RADS, aspect, near, far);
        }
    }
}

impl Component for Camera {
    fn id(&self) -> Id {
        Id::of::<Camera>()
    }
    fn new_component_manager(&self) -> Box<ComponentManager> {
        Box::new(CameraManager::new())
    }
    fn component_manager_id(&self) -> Id {
        Id::of::<CameraManager>()
    }
    fn entity(&self) -> Option<Entity> {
        self.data.borrow().entity.clone()
    }
    fn set_entity(&self, entity: Option<Entity>) {
        self.data.borrow_mut().entity = entity;
    }
}

impl PartialEq<Camera> for Camera {
    fn eq(&self, other: &Camera) -> bool {
        (&*self.data.borrow() as *const _) == (&*other.data.borrow() as *const _)
    }
    fn ne(&self, other: &Camera) -> bool {
        !self.eq(other)
    }
}
