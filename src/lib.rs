#![no_std]
#![feature(collections, alloc)]


extern crate alloc;
extern crate collections;

extern crate mat4;
extern crate scene_graph;
extern crate transform2d_component;
extern crate transform3d_component;


mod camera;
mod camera_manager;

pub use camera::Camera;
pub use camera_manager::CameraManager;
