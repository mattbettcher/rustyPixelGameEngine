use super::vec3d::Vec3d;
use super::vec4d::Vec4d;
use std::ops::{Mul};

#[derive(Debug, Clone, Copy)]
pub struct Mat4x4 {
    pub m: [[f32; 4]; 4],
}

impl Mat4x4 {
    pub fn make_identity() -> Self {
        Mat4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn make_rotation_x(rad: f32) -> Self {
        Mat4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, rad.cos(), rad.sin(), 0.0],
                [0.0, -rad.sin(), rad.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn make_rotation_y(rad: f32) -> Self {
        Mat4x4 {
            m: [
                [rad.cos(), 0.0, rad.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-rad.sin(), 0.0, rad.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn make_rotation_z(rad: f32) -> Self {
        Mat4x4 {
            m: [
                [rad.cos(), rad.sin(), 0.0, 0.0],
                [-rad.sin(), rad.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn make_scale(x: f32, y: f32, z: f32) -> Self {
        Mat4x4 {
            m: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn make_translation(x: f32, y: f32, z: f32) -> Self {
        Mat4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ],
        }
    }

    pub fn make_projection(fov: f32, ar: f32, near: f32, far: f32) -> Self {
        let fov_rad = 1.0 / (fov * 0.5 / 180.0 * 3.14159).tan();
        Mat4x4 {
            m: [
                [ar * fov_rad, 0.0, 0.0, 0.0],
                [0.0, fov_rad, 0.0, 0.0],
                [0.0, 0.0, far / (far - near), 1.0],
                [0.0, 0.0, (-far * near) / (far - near), 0.0],
            ],
        }
    }

    pub fn make_point_at(pos: Vec3d, target: Vec3d, up: Vec3d) -> Self {
        let new_forward = (target - pos).norm();
        let t = up.dot(&new_forward);
        let new_up = (up - (new_forward * t)).norm();
        let new_right = new_up.cross(&new_forward);

        Mat4x4 {
            m: [
                [new_right.x, new_right.y, new_right.z, 0.0],
                [new_up.x, new_up.y, new_up.z, 0.0],
                [new_forward.x, new_forward.y, new_forward.z, 0.0],
                [pos.x, pos.y, pos.z, 1.0],
            ]
        }
    }

    pub fn inverse(&self) -> Mat4x4 {
        let mut inv = Mat4x4::make_identity();

		inv.m[0][0] =  self.m[1][1] * self.m[2][2] * self.m[3][3] - self.m[1][1] * self.m[2][3] * self.m[3][2] - self.m[2][1] * self.m[1][2] * self.m[3][3] + self.m[2][1] * self.m[1][3] * self.m[3][2] + self.m[3][1] * self.m[1][2] * self.m[2][3] - self.m[3][1] * self.m[1][3] * self.m[2][2];
		inv.m[1][0] = -self.m[1][0] * self.m[2][2] * self.m[3][3] + self.m[1][0] * self.m[2][3] * self.m[3][2] + self.m[2][0] * self.m[1][2] * self.m[3][3] - self.m[2][0] * self.m[1][3] * self.m[3][2] - self.m[3][0] * self.m[1][2] * self.m[2][3] + self.m[3][0] * self.m[1][3] * self.m[2][2];
		inv.m[2][0] =  self.m[1][0] * self.m[2][1] * self.m[3][3] - self.m[1][0] * self.m[2][3] * self.m[3][1] - self.m[2][0] * self.m[1][1] * self.m[3][3] + self.m[2][0] * self.m[1][3] * self.m[3][1] + self.m[3][0] * self.m[1][1] * self.m[2][3] - self.m[3][0] * self.m[1][3] * self.m[2][1];
		inv.m[3][0] = -self.m[1][0] * self.m[2][1] * self.m[3][2] + self.m[1][0] * self.m[2][2] * self.m[3][1] + self.m[2][0] * self.m[1][1] * self.m[3][2] - self.m[2][0] * self.m[1][2] * self.m[3][1] - self.m[3][0] * self.m[1][1] * self.m[2][2] + self.m[3][0] * self.m[1][2] * self.m[2][1];
		inv.m[0][1] = -self.m[0][1] * self.m[2][2] * self.m[3][3] + self.m[0][1] * self.m[2][3] * self.m[3][2] + self.m[2][1] * self.m[0][2] * self.m[3][3] - self.m[2][1] * self.m[0][3] * self.m[3][2] - self.m[3][1] * self.m[0][2] * self.m[2][3] + self.m[3][1] * self.m[0][3] * self.m[2][2];
		inv.m[1][1] =  self.m[0][0] * self.m[2][2] * self.m[3][3] - self.m[0][0] * self.m[2][3] * self.m[3][2] - self.m[2][0] * self.m[0][2] * self.m[3][3] + self.m[2][0] * self.m[0][3] * self.m[3][2] + self.m[3][0] * self.m[0][2] * self.m[2][3] - self.m[3][0] * self.m[0][3] * self.m[2][2];
		inv.m[2][1] = -self.m[0][0] * self.m[2][1] * self.m[3][3] + self.m[0][0] * self.m[2][3] * self.m[3][1] + self.m[2][0] * self.m[0][1] * self.m[3][3] - self.m[2][0] * self.m[0][3] * self.m[3][1] - self.m[3][0] * self.m[0][1] * self.m[2][3] + self.m[3][0] * self.m[0][3] * self.m[2][1];
		inv.m[3][1] =  self.m[0][0] * self.m[2][1] * self.m[3][2] - self.m[0][0] * self.m[2][2] * self.m[3][1] - self.m[2][0] * self.m[0][1] * self.m[3][2] + self.m[2][0] * self.m[0][2] * self.m[3][1] + self.m[3][0] * self.m[0][1] * self.m[2][2] - self.m[3][0] * self.m[0][2] * self.m[2][1];
		inv.m[0][2] =  self.m[0][1] * self.m[1][2] * self.m[3][3] - self.m[0][1] * self.m[1][3] * self.m[3][2] - self.m[1][1] * self.m[0][2] * self.m[3][3] + self.m[1][1] * self.m[0][3] * self.m[3][2] + self.m[3][1] * self.m[0][2] * self.m[1][3] - self.m[3][1] * self.m[0][3] * self.m[1][2];
		inv.m[1][2] = -self.m[0][0] * self.m[1][2] * self.m[3][3] + self.m[0][0] * self.m[1][3] * self.m[3][2] + self.m[1][0] * self.m[0][2] * self.m[3][3] - self.m[1][0] * self.m[0][3] * self.m[3][2] - self.m[3][0] * self.m[0][2] * self.m[1][3] + self.m[3][0] * self.m[0][3] * self.m[1][2];
		inv.m[2][2] =  self.m[0][0] * self.m[1][1] * self.m[3][3] - self.m[0][0] * self.m[1][3] * self.m[3][1] - self.m[1][0] * self.m[0][1] * self.m[3][3] + self.m[1][0] * self.m[0][3] * self.m[3][1] + self.m[3][0] * self.m[0][1] * self.m[1][3] - self.m[3][0] * self.m[0][3] * self.m[1][1];
		inv.m[3][2] = -self.m[0][0] * self.m[1][1] * self.m[3][2] + self.m[0][0] * self.m[1][2] * self.m[3][1] + self.m[1][0] * self.m[0][1] * self.m[3][2] - self.m[1][0] * self.m[0][2] * self.m[3][1] - self.m[3][0] * self.m[0][1] * self.m[1][2] + self.m[3][0] * self.m[0][2] * self.m[1][1];
		inv.m[0][3] = -self.m[0][1] * self.m[1][2] * self.m[2][3] + self.m[0][1] * self.m[1][3] * self.m[2][2] + self.m[1][1] * self.m[0][2] * self.m[2][3] - self.m[1][1] * self.m[0][3] * self.m[2][2] - self.m[2][1] * self.m[0][2] * self.m[1][3] + self.m[2][1] * self.m[0][3] * self.m[1][2];
		inv.m[1][3] =  self.m[0][0] * self.m[1][2] * self.m[2][3] - self.m[0][0] * self.m[1][3] * self.m[2][2] - self.m[1][0] * self.m[0][2] * self.m[2][3] + self.m[1][0] * self.m[0][3] * self.m[2][2] + self.m[2][0] * self.m[0][2] * self.m[1][3] - self.m[2][0] * self.m[0][3] * self.m[1][2];
		inv.m[2][3] = -self.m[0][0] * self.m[1][1] * self.m[2][3] + self.m[0][0] * self.m[1][3] * self.m[2][1] + self.m[1][0] * self.m[0][1] * self.m[2][3] - self.m[1][0] * self.m[0][3] * self.m[2][1] - self.m[2][0] * self.m[0][1] * self.m[1][3] + self.m[2][0] * self.m[0][3] * self.m[1][1];
		inv.m[3][3] =  self.m[0][0] * self.m[1][1] * self.m[2][2] - self.m[0][0] * self.m[1][2] * self.m[2][1] - self.m[1][0] * self.m[0][1] * self.m[2][2] + self.m[1][0] * self.m[0][2] * self.m[2][1] + self.m[2][0] * self.m[0][1] * self.m[1][2] - self.m[2][0] * self.m[0][2] * self.m[1][1];

		let mut det = self.m[0][0] * inv.m[0][0] + self.m[0][1] * inv.m[1][0] + self.m[0][2] * inv.m[2][0] + self.m[0][3] * inv.m[3][0];

		det = 1.0 / det;

        for i in 0..4 {
            for j in 0..4 {
                inv.m[i][j] *= det;
            }
        }

		return inv;
    }
}

impl Mul<Vec3d> for Mat4x4 {
    type Output = Vec3d;

    fn mul(self, rhs: Vec3d) -> Vec3d {
        Vec3d {
            x: rhs.x * self.m[0][0] + rhs.y * self.m[1][0] + rhs.z * self.m[2][0] + 1.0 * self.m[3][0],
            y: rhs.x * self.m[0][1] + rhs.y * self.m[1][1] + rhs.z * self.m[2][1] + 1.0 * self.m[3][1],
            z: rhs.x * self.m[0][2] + rhs.y * self.m[1][2] + rhs.z * self.m[2][2] + 1.0 * self.m[3][2],
            //w: rhs.x * self.m[0][3] + rhs.y * self.m[1][3] + rhs.z * self.m[2][3] + 1.0 * self.m[3][3],
        }
    }
}

impl Mul<Vec4d> for Mat4x4 {
    type Output = Vec4d;

    fn mul(self, rhs: Vec4d) -> Vec4d {
        Vec4d {
            x: rhs.x * self.m[0][0] + rhs.y * self.m[1][0] + rhs.z * self.m[2][0] + rhs.w * self.m[3][0],
            y: rhs.x * self.m[0][1] + rhs.y * self.m[1][1] + rhs.z * self.m[2][1] + rhs.w * self.m[3][1],
            z: rhs.x * self.m[0][2] + rhs.y * self.m[1][2] + rhs.z * self.m[2][2] + rhs.w * self.m[3][2],
            w: rhs.x * self.m[0][3] + rhs.y * self.m[1][3] + rhs.z * self.m[2][3] + rhs.w * self.m[3][3],
        }
    }
}

impl Mul for Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, rhs: Mat4x4) -> Mat4x4 {
        let mut mat: Mat4x4 = Mat4x4::make_identity();

        for c in 0..4 {
            for r in 0..4 {
                mat.m[r][c] = self.m[r][0] * rhs.m[0][c]
                    + self.m[r][1] * rhs.m[1][c]
                    + self.m[r][2] * rhs.m[2][c]
                    + self.m[r][3] * rhs.m[3][c];
            }
        }

        mat
    }
}
