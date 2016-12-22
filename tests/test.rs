#![no_std]

extern crate camera_components;
extern crate scene_graph;
extern crate transform_components;


use camera_components::{Camera3D, Camera3DManager};
use scene_graph::{Scene, Entity};
use transform_components::Transform2D;


#[test]
fn test_scene() {
    let mut scene = Scene::new();
    let mut entity = Entity::new();
    let mut camera3d = Camera3D::new();
    let mut transform = Transform2D::new();

    camera3d.set_active();
    transform.set_position(&[10f32, 10f32]);

    entity
        .add_component(camera3d)
        .add_component(transform);

    scene.add_entity(&mut entity);

    let camera3d_manager = scene.get_component_manager::<Camera3DManager>().unwrap();
    let mut entity_camera3d = camera3d_manager.get_active_camera().unwrap();

    assert_eq!(entity_camera3d.get_view(), &[
        1f32, 0f32, 0f32, 0f32,
        0f32, 1f32, 0f32, 0f32,
        0f32, 0f32, 1f32, 0f32,
        -10f32, -10f32, 0f32, 1f32
    ]);
    assert_eq!(entity_camera3d.get_projection(), &[
        3.1715946f32, 0f32, 0f32, 0f32,
        0f32, 3.1715946f32, 0f32, 0f32,
        0f32, 0f32, -1.0000019f32, -1f32,
        0f32, 0f32, -0.002000002f32, 0f32
    ]);
}
