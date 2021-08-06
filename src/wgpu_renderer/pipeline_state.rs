// Manager for RenderPipeline state, layouts, and shader loading
use wgpu::*;

pub struct PipelineState {
  render_pipeline: RenderPipeline,
  pipeline_layout: PipelineLayout,

  // bind group layouts
  texture_bind_group_layout: BindGroupLayout,
  uniform_bind_group_layout: BindGroupLayout,
}
