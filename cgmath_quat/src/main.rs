#[macro_use] extern crate assert_approx_eq;

extern crate cgmath;
use cgmath::{Vector3, Quaternion, Rad, Angle, Rotation3, Rotation};



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn rot_90_z_1() {
        let a: Vector3<f32> = Vector3::unit_x();
        let a_rot = rot_90_z(a);
        assert_approx_eq!(0.0, a_rot.x);
        assert_approx_eq!(1.0, a_rot.y);
        assert_approx_eq!(0.0, a_rot.z);
        // 0.000000059604645 0.99999994 0
    }
}

pub fn rot_90_z(vec: Vector3<f32> ) -> Vector3<f32> {
    let rot: Quaternion<f32> = Quaternion::from_axis_angle(Vector3::unit_z(),
                                                            Rad::turn_div_4());
    rot.rotate_vector(vec)
}

fn main() {
    let a: Vector3<f32> = Vector3::unit_x();

    // fn test(src: Vector3<f32>, dst: Vector3<f32>) {
    //     let q = Quaternion::from_arc(src, dst, None);
    //     let v = q.rotate_vector(src);
    //     assert_ulps_eq!(v.normalize(), dst.normalize());

    println!("{} {} {}", a.x, a.y, a.z);
    let a_rot = rot_90_z(a);
    println!("{} {} {}", a_rot.x, a_rot.y, a_rot.z);

}
