#![feature(alloc)]
#![no_std]


extern crate alloc;

extern crate shared;
extern crate mat4;
extern crate scene_graph;
extern crate transform_components;


mod camera;
mod camera_manager;

pub use camera::Camera;
pub use camera_manager::CameraManager;
