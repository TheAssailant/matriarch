//! # Matriarch
//!
//! Matriarch is a Linear Algebra and Matrix library written in pure Rust.

mod mat2;
mod mat3;
mod mat4;
mod vec2;
mod vec3;
mod vec4;

pub use self::mat2::Mat2;
pub use self::mat3::Mat3;
pub use self::mat4::Mat4;
pub use self::vec2::Vec2;
pub use self::vec3::Vec3;
pub use self::vec4::Vec4;
