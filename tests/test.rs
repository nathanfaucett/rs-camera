#![no_std]

extern crate camera_component;
extern crate scene_graph;
extern crate transform2d_component;


use camera_component::{Camera, CameraManager};
use scene_graph::{Scene, Entity};
use transform2d_component::Transform2D;


#[test]
fn test_scene() {
    let scene = Scene::new();
    let entity = Entity::new();
    let camera = Camera::new();
    let transform = Transform2D::new();

    camera.set_active();
    transform.set_position(&[10f32, 10f32]);

    entity
        .add_component(camera)
        .add_component(transform);

    scene.add_entity(entity.clone());

    let camera_manager = scene.get_component_manager::<CameraManager>().unwrap();
    let entity_camera = camera_manager.active_camera().unwrap();

    assert!(entity_camera.view() == [
        1f32, 0f32, 0f32, 0f32,
        0f32, 1f32, 0f32, 0f32,
        0f32, 0f32, 1f32, 0f32,
        -10f32, -10f32, 0f32, 1f32
    ]);
    assert!(entity_camera.projection() == [
        3.1715946f32, 0f32, 0f32, 0f32,
        0f32, 3.1715946f32, 0f32, 0f32,
        0f32, 0f32, -1f32, -1f32,
        0f32, 0f32, -0.00000023841858f32, 0f32
    ]);
}
