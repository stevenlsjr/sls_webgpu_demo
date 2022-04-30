use crate::gltf::buffer::Target;
use gltf::Document;

#[derive(Debug)]
pub struct GltfScene {
  pub(crate) scene_id: usize,

  pub(crate) document: Document,
  pub(crate) buffers: Vec<gltf::buffer::Data>,
  pub(crate) images: Vec<gltf::image::Data>,
  pub(crate) wgpu_resources: Option<WgpuResources>,
}

impl GltfScene {
  pub fn from_import(
    document: Document,
    scene_id: usize,
    buffers: Vec<gltf::buffer::Data>,
    images: Vec<gltf::image::Data>,
  ) -> Self {
    let instance = Self {
      scene_id,
      document,
      buffers,
      images,
      wgpu_resources: None,
    };
    instance
  }

  pub fn init_buffers(
    &mut self,
    _queue: &wgpu::Queue,
    _device: &wgpu::Device,
  ) -> anyhow::Result<()> {
    if let Some(_res) = &self.wgpu_resources {
      anyhow::bail!("wgpu buffers already loaded");
    }
    let _resources = Some(WgpuResources::default());
    for (_i, buffer_view) in self.document.views().enumerate() {
      match buffer_view.target() {
        None => {}
        Some(Target::ArrayBuffer) => {}
        Some(Target::ElementArrayBuffer) => {}
      }
    }

    Ok(())
  }
}

#[derive(Default, Debug)]
pub struct WgpuResources {
  buffers: Vec<wgpu::Buffer>,
}
