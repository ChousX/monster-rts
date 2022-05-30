use bevy::prelude::*;
use crate::controls::LeftStickEvent;
pub struct CameraMoveEvent{
    pub inputs: [bool; 6]
}

pub fn move_camera(
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>, 
    mut move_event: EventReader<CameraMoveEvent>,
    time: Res<Time>,
){
    for event in move_event.iter(){
        for (mut transform, mut ortho) in query.iter_mut() {
            let mut direction = Vec3::ZERO;
            //left
            if  event.inputs[0] {
                direction -= Vec3::new(1.0, 0.0, 0.0);
            }
            //right
            if event.inputs[1] {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }
            //up
            if event.inputs[2] {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }
            //down
            if event.inputs[3] {
                direction -= Vec3::new(0.0, 1.0, 0.0);
            }

            //scale up
            if event.inputs[4] {
                ortho.scale += 0.1;
            }
            //scale down
            if event.inputs[5] {
                ortho.scale -= 0.1;
            }
            // 
            if ortho.scale < 0.5 {
                ortho.scale = 0.5;
            }

            let z = transform.translation.z; // keep the z
            transform.translation += time.delta_seconds() * direction * 500.;
            transform.translation.z = z;
        }
    }
}

pub fn move_camera_gamepad(
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>, 
    mut move_event: EventReader<LeftStickEvent>,
    time: Res<Time>,
){
    for event in move_event.iter(){
        for (mut transform, mut ortho) in query.iter_mut() {
            let mut direction = Vec3::ZERO;
            direction.x += event.0.x;
            direction.y += event.0.y;
            let z = transform.translation.z;
            transform.translation += time.delta_seconds() * direction * 500.;
            transform.translation.z = z;
        }
    }
}