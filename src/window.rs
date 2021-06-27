use raw_window_handle::HasRawWindowHandle;

pub trait AsWindow: HasRawWindowHandle {
    fn size(&self) -> (u32, u32);
    fn set_size(&mut self, size: (u32, u32));
}

#[cfg(feature = "sdl2")]
mod sdl_impl {
    use crate::window::AsWindow;
    use sdl2::video::Window;

    impl AsWindow for Window {
        #[inline]
        fn size(&self) -> (u32, u32) {
            self.vulkan_drawable_size()
        }

        #[inline]
        fn set_size(&mut self, size: (u32, u32)) {
            self.set_size(size.0, size.1);
        }
    }
}
#[cfg(feature = "sdl2")]
pub use sdl_impl::*;
