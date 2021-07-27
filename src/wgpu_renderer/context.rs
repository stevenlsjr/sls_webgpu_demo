use crate::{
  error::Error,
  game::{resources::Scene, GameState},
};
use anyhow::{anyhow, Error as AnyError};

use crate::window::AsWindow;

use super::{
  mesh::{Mesh, MeshGeometry},
  uniforms::Uniforms,
};
use crate::renderer_common::geometry::Vertex;

use crate::{
  game::components::{RenderModel, Transform3D},
  renderer_common::allocator::ResourceManager,
};
use std::{
  fmt,
  fmt::Formatter,
  sync::{Arc, RwLock},
};
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  Face, RenderPass, RenderPipeline, Texture,
};

pub struct Context {
  pub instance: wgpu::Instance,
  pub surface: wgpu::Surface,
  pub adapter: wgpu::Adapter,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub swapchain: wgpu::SwapChain,
  pub sc_desc: wgpu::SwapChainDescriptor,
  pub pipeline_layout: wgpu::PipelineLayout,

  render_pipeline: wgpu::RenderPipeline,
  uniform_bind_group: wgpu::BindGroup,
  // scene resources
  mesh: Mesh,
  uniforms: Uniforms,
  uniform_buffer: wgpu::Buffer,
  pub meshes: Arc<RwLock<ResourceManager<Mesh>>>,
  pub textures: Arc<RwLock<ResourceManager<Texture>>>,
}

impl fmt::Debug for Context {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("Context")
      .field("instance", &self.instance)
      .finish()
  }
}

impl Context {
  pub fn new<W: AsWindow>(window: &mut W) -> Builder<W> {
    Builder { window, size: None }
  }

  pub fn on_resize(&mut self, size: (u32, u32)) {
    let (width, height) = size;
    self.sc_desc.width = width;
    self.sc_desc.height = height;
    self.swapchain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
  }

  pub fn update(&mut self) {}

  pub fn render(&mut self, game: &GameState) -> Result<(), AnyError> {
    let camera = game
      .resources()
      .get::<Scene>()
      .map(|s| s.main_camera_components(&game.world()))
      .unwrap_or(Ok(None))
      .map_err(|error| anyhow!("error accessing camera {:?}", error))?;

    match camera {
      None => {
        log::warn!("no main camera found");
        Ok(())
      }
      Some(camera) => {
        self.uniforms.update_from_camera(camera);
        self.queue.write_buffer(
          &self.uniform_buffer,
          0,
          bytemuck::cast_slice(&[self.uniforms]),
        );

        let frame = self
          .swapchain
          .get_current_frame()
          .map_err(|e| anyhow!("swapchain error {:?}", e))?
          .output;

        let mut encoder = self
          .device
          .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
          });
        {
          let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
              view: &frame.view,
              resolve_target: None,
              ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                  r: 0.1,
                  g: 0.2,
                  b: 0.3,
                  a: 1.0,
                }),
                store: true,
              },
            }],
            depth_stencil_attachment: None,
          });

          render_pass.set_pipeline(&self.render_pipeline);
          render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

          if let Some(mesh) = self.mesh.buffers() {
            let n_indices = self.mesh.geometry().indices.len() as u32;
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..n_indices, 0, 0..1);
          };
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
      }
    }
  }

  #[cfg(feature = "wgpu_imgui")]
  pub fn render_with_ui(
    &mut self,
    game: &GameState,
    mut imgui_frame: imgui::Ui,
    imgui_renderer: &mut imgui_wgpu::Renderer,
  ) -> Result<(), anyhow::Error> {
    let camera = game
      .resources()
      .get::<Scene>()
      .map(|s| s.main_camera_components(&game.world()))
      .unwrap_or(Ok(None))
      .map_err(|error| anyhow!("error accessing camera {:?}", error))?;
    match camera {
      None => {
        log::warn!("no main camera found");
        Ok(())
      }
      Some(camera) => {
        self.uniforms.update_from_camera(camera);
        self.queue.write_buffer(
          &self.uniform_buffer,
          0,
          bytemuck::cast_slice(&[self.uniforms]),
        );

        let frame = self
          .swapchain
          .get_current_frame()
          .map_err(|e| anyhow!("swapchain error {:?}", e))?
          .output;

        let mut encoder = self
          .device
          .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
          });
        {
          let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
              view: &frame.view,
              resolve_target: None,
              ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                  r: 0.1,
                  g: 0.2,
                  b: 0.3,
                  a: 1.0,
                }),
                store: true,
              },
            }],
            depth_stencil_attachment: None,
          });

          render_pass.set_pipeline(&self.render_pipeline);
          render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

          if let Some(mesh) = self.mesh.buffers() {
            let n_indices = self.mesh.geometry().indices.len() as u32;
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..n_indices, 0, 0..1);
          };
          {
            let draw_data = imgui_frame.render();
            imgui_renderer.render(draw_data, &self.queue, &self.device, &mut render_pass)?;
          }
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
      }
    }
  }
}

pub struct Builder<'a, W: AsWindow> {
  size: Option<(i32, i32)>,
  window: &'a mut W,
}

impl<'a, W: AsWindow> Builder<'a, W> {
  pub async fn build(self) -> Result<Context, Error> {
    #[cfg(not(target_os = "linux"))]
    let backends = wgpu::BackendBit::all();
    #[cfg(target_os = "linux")]
    let backends = wgpu::BackendBit::VULKAN;

    let instance = wgpu::Instance::new(backends);
    let surface = unsafe { instance.create_surface(self.window) };
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
      })
      .await
      .ok_or_else(|| Error::Create {
        reason: "could not create adapter".into(),
      })?;

    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          label: None,
          features: wgpu::Features::empty(),
          limits: wgpu::Limits::default(),
        },
        None,
      )
      .await
      .map_err(|e| crate::Error::from_error(Box::new(e)))?;

    let uniforms = Uniforms::default();
    // let camera = self.camera;
    // uniforms.update_from_camera(&camera);

    let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("Main UBO"),
      contents: bytemuck::cast_slice(&[uniforms.clone()]),
      usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    });

    let ubo_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("ubo_layout"),
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStage::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
    });
    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &ubo_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: uniform_buffer.as_entire_binding(),
      }],
      label: Some("ubo_bind_group"),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("pipeline_layout"),
      bind_group_layouts: &[&ubo_layout],
      push_constant_ranges: &[],
    });
    let (w_width, w_height) = self.window.size();
    let sc_desc = wgpu::SwapChainDescriptor {
      usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
      format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
      width: w_width,
      height: w_height,
      present_mode: wgpu::PresentMode::Fifo,
    };
    let swapchain = device.create_swap_chain(&surface, &sc_desc);
    let main_vert_shader =
      device.create_shader_module(&wgpu::include_spirv!("../shaders/main.vert.spv"));
    let main_frag_shader =
      device.create_shader_module(&wgpu::include_spirv!("../shaders/main.frag.spv"));

    let render_pipeline = create_render_pipeline(
      &device,
      &sc_desc,
      &pipeline_layout,
      &main_vert_shader,
      &main_frag_shader,
    )?;

    let mesh = {
      let geom = MeshGeometry::unit_shere(10, 10);
      Mesh::from_geometry(geom, &device)?
    };

    let result = Ok(Context {
      surface,
      instance,
      adapter,
      device,
      queue,
      pipeline_layout,
      sc_desc,
      swapchain,
      render_pipeline,
      mesh,
      uniforms,
      uniform_buffer,
      uniform_bind_group,
      meshes: Default::default(),
      textures: Default::default(),
    });
    result
  }

  pub fn with_size(mut self, size: (i32, i32)) -> Self {
    self.size = Some(size);
    self
  }
}

fn create_render_pipeline(
  device: &wgpu::Device,
  sc_desc: &wgpu::SwapChainDescriptor,
  layout: &wgpu::PipelineLayout,
  vert_shader: &wgpu::ShaderModule,
  frag_shader: &wgpu::ShaderModule,
) -> Result<RenderPipeline, Error> {
  let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("Render Pipeline"),
    layout: Some(layout),
    vertex: wgpu::VertexState {
      module: vert_shader,
      entry_point: "main",
      buffers: &[Vertex::desc()],
    },
    fragment: Some(wgpu::FragmentState {
      module: &frag_shader,
      entry_point: "main",
      targets: &[sc_desc.format.into()],
    }),
    primitive: wgpu::PrimitiveState {
      cull_mode: Some(Face::Back),
      ..wgpu::PrimitiveState::default()
    },
    depth_stencil: None,
    multisample: wgpu::MultisampleState::default(),
  });

  Ok(render_pipeline)
}

#[cfg(target_arch = "wasm32")]
mod wasm {
  use super::*;
  use crate::platform::html5::FromCanvas;
  use web_sys::HtmlCanvasElement;
}
