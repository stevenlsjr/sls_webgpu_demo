use image::{GenericImageView, ImageError};
use std::num::NonZeroU32;
use thiserror::Error;
use wgpu::{Device, Queue, Sampler, Texture, TextureView};

#[derive(Debug, Error)]
pub enum TextureError {
  #[error("imageError {0:?}")]
  Img(#[from] image::ImageError),
  #[error("miscelaneous {0}")]
  Other(String),
}

/// Texture Wrapper object
///
#[derive(Debug)]
pub struct TextureResource {
  texture: Texture,
  view: TextureView,
  sampler: Sampler,
}

impl TextureResource {
  pub fn from_texture(tex: Texture, queue: &Queue, device: &Device) -> Result<Self, TextureError> {
    let texture_view = tex.create_view(&wgpu::TextureViewDescriptor {
      label: Some(concat!(std::file!(), ":", std::line!())),
      ..Default::default()
    });
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::FilterMode::Nearest,
      ..Default::default()
    });
    Ok(Self {
      texture: tex,
      view: texture_view,
      sampler,
    })
  }

  // accessors
  pub fn texture(&self) -> &wgpu::Texture {
    &self.texture
  }
  pub fn set_texture(&mut self, texture: wgpu::Texture) {
    self.texture = texture;
  }

  pub fn view(&self) -> &wgpu::TextureView {
    &self.view
  }
  pub fn set_view(&mut self, view: wgpu::TextureView) {
    self.view = view;
  }

  pub fn sampler(&self) -> &wgpu::Sampler {
    &self.sampler
  }
  pub fn set_sampler(&mut self, sampler: wgpu::Sampler) {
    self.sampler = sampler;
  }
}

pub fn load_texture_from_image(
  img: image::DynamicImage,
  queue: &Queue,
  device: &Device,
) -> Result<Texture, TextureError> {
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

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(not(target_arch = "wasm32"))]
mod native {
  use super::*;
  use std::path::Path;

  pub fn load_image_from_file<P: AsRef<Path>>(
    path: P,
    queue: &Queue,
    device: &Device,
  ) -> Result<Texture, TextureError> {
    let img = image::open(path)?;
    let texture = load_texture_from_image(img, queue, device)?;
    Ok(texture)
  }
}
