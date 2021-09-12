// Manager for RenderPipeline state, layouts, and shader loading
use wgpu::*;
use atomic_refcell::AtomicRefCell;
use std::ops::Deref;
use crossbeam::atomic::AtomicCell;
use crate::renderer_common::handle::Handle;
use crate::renderer_common::allocator::ResourceManager;

#[derive(Debug)]
pub struct AllocedRenderPipelineDescriptor {}


#[derive(Debug)]
pub struct PipelineProgram {
  pipeline: Option<RenderPipeline>,
  current_fb_size: Option<(usize, usize)>,

  pub layout: PipelineLayout,

}

#[derive(Debug)]
pub struct RendererPipelines {
  pub(crate) debug_light_pipeline: Option<RenderPipeline>,
  pub(crate) debug_light_layout: PipelineLayout,
  pub(crate) pbr_model_pipeline: Option<RenderPipeline>,
  pub(crate) pbr_model_layout: PipelineLayout,
}

impl RendererPipelines {
  pub(crate) fn new(
    device: &Device,
    debug_light_layouts: &[&BindGroupLayout],
    pbr_model_layouts: &[&BindGroupLayout],
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
    }
  }

  pub(crate) fn create_pipelines(&mut self,
                                 device: &Device,
                                 sc_desc: &SwapChainDescriptor,
  ) {}
}

