pub mod camera;
pub mod context;
pub mod error;
pub mod geometry;
pub mod mesh;
pub mod window;
pub mod uniforms;
pub mod game;

pub use legion;

pub use context::Context;
pub use error::Error;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
