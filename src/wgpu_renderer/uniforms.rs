use crate::camera::Camera;
use nalgebra_glm::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
  pub view_projection: [[f32; 4]; 4],
}

impl Default for Uniforms {
  fn default() -> Self {
    let view_projection = Mat4::identity();
    Self {
      view_projection: *view_projection.as_ref(),
    }
  }
}

impl Uniforms {
  pub fn update_from_camera(&mut self, camera: &Camera) {
    let view_projection = camera.view_projection();
    let proj: [[f32; 4]; 4] = view_projection.into();
    self.view_projection = proj;
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointLightUniform {
  pub position: [f32; 3],
  pub _padding: u32,
  pub color: [f32; 3],
}

pub fn make_light_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
  device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    label: None,
    entries: &[wgpu::BindGroupLayoutEntry {
      binding: 0,
      visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
      ty: wgpu::BindingType::Buffer {
        ty:  wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: None
      },
      count: None
    }],
  })
}

impl Default for PointLightUniform {
  fn default() -> Self {
    Self {
      _padding: 0,
      position: [0.0, 0.0, 0.0],
      color: [1.0, 1.0, 1.0],
    }
  }
}
