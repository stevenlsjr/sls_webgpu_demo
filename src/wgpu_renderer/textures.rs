use image::GenericImageView;
use std::num::NonZeroU32;
#[derive(Debug, Clone)]
pub enum TextureError {
  Img(image::ImageError),
  Other(String),
}

impl From<image::ImageError> for TextureError {
  fn from(err: ImageError) -> Self {
    Self::Img(err)
  }
}

use image::ImageError;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(not(target_arch = "wasm32"))]
mod native {
  use super::*;
  use crate::wgpu::{Queue, Texture};
  use std::path::Path;
  use wgpu::Device;

  pub fn load_image_from_file<P: AsRef<Path>>(
    path: P,
    queue: &Queue,
    device: &Device,
  ) -> Result<Texture, TextureError> {
    let img = image::open(path)?;
    let rgba = img
      .as_rgba8()
      .ok_or_else(|| TextureError::Other(format!("image {:?} cannot be converted to rgba", img)))?;
    let dimensions = img.dimensions();
    let texture_size = wgpu::Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
      size: texture_size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
      label: Some("by__load_image_from_file"),
    });
    queue.write_texture(
      wgpu::ImageCopyTexture {
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
      },
      rgba,
      wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: NonZeroU32::new(4 * dimensions.0),
        rows_per_image: NonZeroU32::new(dimensions.1),
      },
      texture_size,
    );

    Ok(texture)
  }
}
