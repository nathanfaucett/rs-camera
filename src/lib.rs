#![feature(alloc)]
#![no_std]


extern crate alloc;

extern crate shared;
extern crate to_radians;
extern crate mat32;
extern crate mat4;
extern crate scene_graph;
extern crate transform_components;


mod camera3d;
mod camera3d_manager;

mod camera2d;
mod camera2d_manager;

pub use camera3d::Camera3D;
pub use camera3d_manager::Camera3DManager;

pub use camera2d::Camera2D;
pub use camera2d_manager::Camera2DManager;
