pub mod vec3d;
pub mod vec4d;
pub mod mat4x4;
pub mod triangle;
pub mod plane;

pub use self::vec3d::Vec3d;
pub use self::vec4d::Vec4d;
pub use self::triangle::Triangle;
pub use self::plane::Plane;

pub struct Mesh {
    pub tris: Vec<Triangle>
}
