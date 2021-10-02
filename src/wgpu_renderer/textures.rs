use std::num::NonZeroU32;

use image::{DynamicImage, GenericImageView};
use thiserror::Error;

use wgpu::{
  BindGroup, BindGroupDescriptor, BindGroupEntry, Device, Queue, Sampler, ShaderStages, Texture,
  TextureSampleType, TextureView,
};

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

use crate::{
  renderer_common::handle::{Handle, HandleIndex},
  wgpu::{BindGroupLayout, BindingResource, FilterMode, TextureViewDimension},
  Context,
};

pub const DEFAULT_TEX_JPEG: &[u8] = include_bytes!("../../assets/uv_grid_opengl.jpg");

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
  pub const DEPTH_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

  pub fn from_image(
    img: &DynamicImage,
    queue: &Queue,
    device: &Device,
  ) -> Result<Self, TextureError> {
    let tex = load_texture_from_image(img, queue, device)?;
    Self::from_texture(tex, queue, device)
  }

  /// Creates a new texture resource with sampler and view
  /// from a wgpu texture object
  pub fn from_texture(tex: Texture, _queue: &Queue, device: &Device) -> Result<Self, TextureError> {
    let texture_view = tex.create_view(&wgpu::TextureViewDescriptor {
      label: Some(concat!(std::file!(), ":", std::line!())),
      ..Default::default()
    });
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Linear,
      mipmap_filter: wgpu::FilterMode::Linear,
      lod_min_clamp: 0.1,
      lod_max_clamp: f32::MAX,
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

  pub fn new_depth_stencil_texture(
    device: &Device,
    (width, height): (u32, u32),
    label: &str,
  ) -> Self {
    let size = wgpu::Extent3d {
      width,
      height,
      depth_or_array_layers: 1,
    };
    let desc = wgpu::TextureDescriptor {
      label: Some(label),
      size,
      mip_level_count: 1,
      sample_count: 1,

      dimension: wgpu::TextureDimension::D2,
      format: Self::DEPTH_TEXTURE_FORMAT,
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT // 3.
                | wgpu::TextureUsages::TEXTURE_BINDING,
    };
    let texture = device.create_texture(&desc);
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: FilterMode::Linear,
      min_filter: FilterMode::Linear,
      mipmap_filter: FilterMode::Linear,
      compare: Some(wgpu::CompareFunction::LessEqual),
      lod_min_clamp: 0.0,
      lod_max_clamp: 100.0,
      ..Default::default()
    });
    Self {
      texture,
      view,
      sampler,
    }
  }
}

pub fn load_texture_from_image(
  img: &image::DynamicImage,
  queue: &Queue,
  device: &Device,
) -> Result<Texture, TextureError> {
  let rgba = img.to_rgba8();

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
    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    label: Some("by__load_image_from_file"),
  });
  queue.write_texture(
    wgpu::ImageCopyTexture {
      texture: &texture,
      mip_level: 0,
      origin: wgpu::Origin3d::ZERO,
      aspect: Default::default(),
    },
    &rgba,
    wgpu::ImageDataLayout {
      offset: 0,
      bytes_per_row: NonZeroU32::new(4 * dimensions.0),
      rows_per_image: NonZeroU32::new(dimensions.1),
    },
    texture_size,
  );

  Ok(texture)
}

pub fn create_texture_bind_group_layout(device: &Device) -> BindGroupLayout {
  let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    label: Some("create_texture_bind_group"),
    entries: &[
      wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
          multisampled: false,
          view_dimension: TextureViewDimension::D2,
          sample_type: TextureSampleType::Float { filterable: true },
        },
        count: None,
      },
      wgpu::BindGroupLayoutEntry {
        binding: 1,
        visibility: ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Sampler {
          comparison: false,
          filtering: true,
        },
        count: None,
      },
    ],
  });
  layout
}

pub trait BindTexture {
  fn bind_texture(&mut self, tex: HandleIndex) -> Result<(), anyhow::Error>;
}

pub fn basic_texture_bind_group(
  tex: &TextureResource,
  bgl: &BindGroupLayout,
  device: &Device,
) -> BindGroup {
  device.create_bind_group(&BindGroupDescriptor {
    label: Some("basic_texture_bind_group"),
    layout: bgl,
    entries: &[
      BindGroupEntry {
        binding: 0,
        resource: BindingResource::TextureView(&tex.view),
      },
      BindGroupEntry {
        binding: 1,
        resource: BindingResource::Sampler(&tex.sampler),
      },
    ],
  })
}

impl BindTexture for Context {
  fn bind_texture(&mut self, tex_handle: HandleIndex) -> Result<(), anyhow::Error> {
    let tex_handle: Handle<TextureResource> = tex_handle.into_typed();
    self.main_tex_handle = Some(tex_handle);
    let textures_arc = self.resources.textures.clone();
    let textures = textures_arc
      .read()
      .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    let tex = textures.get_ref(tex_handle)?;
    let _bind_group = basic_texture_bind_group(tex, &self.texture_bind_group_layout, &self.device);
    Ok(())
  }
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
  use std::path::Path;

  use super::*;

  pub fn load_image_from_file<P: AsRef<Path>>(
    path: P,
    queue: &Queue,
    device: &Device,
  ) -> Result<Texture, TextureError> {
    let img = image::open(path)?;
    let texture = load_texture_from_image(&img, queue, device)?;
    Ok(texture)
  }
}
