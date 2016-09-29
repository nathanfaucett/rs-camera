#![no_std]

extern crate camera_component;
extern crate scene_graph;
extern crate transform_components;


use camera_component::{Camera, CameraManager};
use scene_graph::{Scene, Entity};
use transform_components::Transform2D;


#[test]
fn test_scene() {
    let mut scene = Scene::new();
    let mut entity = Entity::new();
    let mut camera = Camera::new();
    let mut transform = Transform2D::new();

    camera.set_active();
    transform.set_position(&[10f32, 10f32]);

    entity
        .add_component(camera)
        .add_component(transform);

    scene.add_entity(&mut entity);

    let camera_manager = scene.get_component_manager::<CameraManager>().unwrap();
    let mut entity_camera = camera_manager.get_active_camera().unwrap();

    assert_eq!(entity_camera.get_view(), &[
        1f32, 0f32, 0f32, 0f32,
        0f32, 1f32, 0f32, 0f32,
        0f32, 0f32, 1f32, 0f32,
        -10f32, -10f32, 0f32, 1f32
    ]);
    assert_eq!(entity_camera.get_projection(), &[
        3.1715946f32, 0f32, 0f32, 0f32,
        0f32, 3.1715946f32, 0f32, 0f32,
        0f32, 0f32, -1f32, -1f32,
        0f32, 0f32, -0.00000023841858f32, 0f32
    ]);
}
