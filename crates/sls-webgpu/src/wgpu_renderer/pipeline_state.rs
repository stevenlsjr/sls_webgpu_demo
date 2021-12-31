// Manager for RenderPipeline state, layouts, and shader loading
use crate::{
  renderer_common::{allocator::ResourceManager, geometry::Vertex, handle::Handle},
  wgpu_renderer::{textures::TextureResource, ModelInstance},
};
use atomic_refcell::AtomicRefCell;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use wgpu::*;

#[derive(Debug)]
pub struct AllocedRenderPipelineDescriptor {}

#[derive(Debug)]
pub struct PipelineProgram {
  pipeline: Option<RenderPipeline>,
  current_fb_size: Option<(usize, usize)>,

  pub layout: PipelineLayout,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShaderInfo {
  pub frag_shader: Handle<ShaderModule>,
  pub frag_entrypoint: Option<String>,
  pub vert_shader: Handle<ShaderModule>,
  pub vert_entrypoint: Option<String>,
}

impl ShaderInfo {
  pub fn new(frag_shader: Handle<ShaderModule>, vert_shader: Handle<ShaderModule>) -> Self {
    Self {
      frag_shader,
      vert_shader,
      frag_entrypoint: None,
      vert_entrypoint: None,
    }
  }
  /**
   * Given shader module descriptors, compiles and loads the shaders into the resorce manager, and creates the shaderInfo
   * object
   */
  pub fn from_shader_descriptors(
    device: &Device,
    shaders: &mut ResourceManager<ShaderModule>,
    vert_descriptor: &wgpu::ShaderModuleDescriptor,
    frag_descriptor: &wgpu::ShaderModuleDescriptor,
  ) -> Self {
    let vert_shader = shaders.insert(device.create_shader_module(vert_descriptor));
    let frag_shader = shaders.insert(device.create_shader_module(frag_descriptor));
    Self::new(frag_shader, vert_shader)
  }
  pub fn frag_entrypoint_name(&self) -> &str {
    self.frag_entrypoint.as_deref().unwrap_or("main")
  }

  pub fn vert_entrypoint_name(&self) -> &str {
    self.vert_entrypoint.as_deref().unwrap_or("main")
  }
}

#[derive(Debug)]
pub struct RendererPipelines {
  pub(crate) debug_light_pipeline: Option<RenderPipeline>,
  pub(crate) debug_light_layout: PipelineLayout,
  pub(crate) debug_light_shaders: ShaderInfo,

  pub(crate) pbr_model_pipeline: Option<RenderPipeline>,
  pub(crate) pbr_model_layout: PipelineLayout,
  pub(crate) pbr_model_shaders: ShaderInfo,

  pub(crate) color_target: wgpu::ColorTargetState,
}

impl RendererPipelines {
  pub(crate) fn new(
    device: &Device,
    debug_light_layouts: &[&BindGroupLayout],
    debug_light_shaders: ShaderInfo,
    pbr_model_layouts: &[&BindGroupLayout],
    pbr_model_shaders: ShaderInfo,
    color_target: ColorTargetState,
  ) -> Self {
    let debug_light_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
      label: Some("debug_light renderer"),
      bind_group_layouts: debug_light_layouts,
      push_constant_ranges: &[],
    });

    let pbr_model_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
      label: Some("pbr_model renderer"),
      bind_group_layouts: pbr_model_layouts,
      push_constant_ranges: &[],
    });
    Self {
      debug_light_pipeline: None,
      debug_light_layout,
      pbr_model_pipeline: None,
      pbr_model_layout,
      color_target,
      debug_light_shaders,
      pbr_model_shaders,
    }
  }

  pub fn build_pipelines(
    &mut self,
    device: &wgpu::Device,
    shaders: &ResourceManager<ShaderModule>,
  ) -> anyhow::Result<()> {
    self.pbr_model_pipeline = {
      let vert_shader = shaders.try_get_ref(self.pbr_model_shaders.vert_shader)?;
      let frag_shader = shaders.try_get_ref(self.pbr_model_shaders.frag_shader)?;
      Some(create_render_pipeline(
        device,
        &self.pbr_model_layout,
        vert_shader,
        frag_shader,
        self.color_target.clone(),
      ))
    };
    self.debug_light_pipeline = {
      let vert_shader = shaders.try_get_ref(self.debug_light_shaders.vert_shader)?;
      let frag_shader = shaders.try_get_ref(self.debug_light_shaders.frag_shader)?;
      Some(create_render_pipeline(
        device,
        &self.debug_light_layout,
        vert_shader,
        frag_shader,
        self.color_target.clone(),
      ))
    };

    Ok(())
  }

  pub fn resize(&mut self, (width, height): (u32, u32)) {}
}

pub fn create_render_pipeline(
  device: &wgpu::Device,
  layout: &wgpu::PipelineLayout,
  vert_shader: &wgpu::ShaderModule,
  frag_shader: &wgpu::ShaderModule,
  color_target: ColorTargetState,
) -> RenderPipeline {
  let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("Render Pipeline"),
    layout: Some(layout),
    vertex: wgpu::VertexState {
      module: vert_shader,
      entry_point: "main",
      buffers: &[Vertex::desc(), ModelInstance::desc()],
    },
    fragment: Some(wgpu::FragmentState {
      module: &frag_shader,
      entry_point: "main",
      targets: &[color_target],
    }),
    primitive: wgpu::PrimitiveState {
      cull_mode: None,
      // cull_mode: Some(Face::Back),
      ..wgpu::PrimitiveState::default()
    },
    depth_stencil: Some(wgpu::DepthStencilState {
      format: TextureResource::DEPTH_TEXTURE_FORMAT,
      depth_write_enabled: true,
      depth_compare: wgpu::CompareFunction::Less,
      stencil: Default::default(),
      bias: Default::default(),
    }),
    multisample: wgpu::MultisampleState::default(),
  });
  render_pipeline
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum ShadingModel {
  Pbr,
  DebugLight,
}

impl Default for ShadingModel {
  fn default() -> Self {
    Self::Pbr
  }
}
