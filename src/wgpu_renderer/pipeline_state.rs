// Manager for RenderPipeline state, layouts, and shader loading
use wgpu::*;

#[derive(Debug)]
pub struct RendererBindGroupLayoutDescriptors<'a> {
  pub material_uniform: BindGroupLayoutDescriptor<'a>,
  pub model_uniform: BindGroupLayoutDescriptor<'a>,
  pub light_uniform: BindGroupLayoutDescriptor<'a>,
}

impl RendererBindGroupLayoutDescriptors<'static> {
  pub fn standard_render_model() -> Self {
    let material_uniform = BindGroupLayoutDescriptor {
      label: Some("material_uniform"),
      entries: &[],
    };
    let model_uniform = BindGroupLayoutDescriptor {
      label: Some("material_uniform"),
      entries: &[],
    };
    let light_uniform = BindGroupLayoutDescriptor {
      label: Some("material_uniform"),
      entries: &[],
    };
    RendererBindGroupLayoutDescriptors {
      material_uniform,
      model_uniform,
      light_uniform,
    }
  }
}

#[derive(Debug)]
pub struct PipelineState {
  render_pipeline: RenderPipeline,
  pipeline_layout: PipelineLayout,

  // bind group layouts
  material_uniform_layout: BindGroupLayout,
  model_uniform_layout: BindGroupLayout,
  light_uniform_layout: BindGroupLayout,
}
