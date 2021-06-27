use crate::error::Error;
use crate::geometry::{Vertex, TRIANGLE_INDICES, TRIANGLE_VERT};
use crate::mesh::{Mesh, MeshGeometry};
use crate::window::AsWindow;
use raw_window_handle::HasRawWindowHandle;
use std::borrow::Cow;
use std::fmt::Formatter;
use std::{error, fmt};
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use wgpu::RenderPipeline;
use crate::camera::Camera;
use crate::uniforms::Uniforms;

pub struct Context<W: AsWindow> {
  pub window: W,
  pub instance: wgpu::Instance,
  pub surface: wgpu::Surface,
  pub adapter: wgpu::Adapter,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub swapchain: wgpu::SwapChain,
  pub sc_desc: wgpu::SwapChainDescriptor,
  pub pipeline_layout: wgpu::PipelineLayout,

  render_pipeline: wgpu::RenderPipeline,

  // scene resources
  mesh: Mesh,
  uniforms: Uniforms,
  uniform_buffer: wgpu::Buffer,
  camera: Camera,
}

impl<W: AsWindow> fmt::Debug for Context<W> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("Context")
      .field("instance", &self.instance)
      .finish()
  }
}

impl<W: AsWindow> Context<W> {
  pub fn new(window: W) -> Builder<W> {
    Builder { window, size: None }
  }

  pub fn on_resize(&mut self, size: (u32, u32)) {
    let (width, height) = size;
    self.sc_desc.width = width;
    self.sc_desc.height = height;
    self.swapchain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
  }

  pub fn update(&mut self) {}

  pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
    let frame = self.swapchain.get_current_frame()?.output;

    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });
    {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("render pass"),
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
          attachment: &frame.view,
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

      if let Some(mesh) = self.mesh.buffers() {
        let n_indices = self.mesh.geometry().indices.len() as u32;
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass
          .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..n_indices, 0, 0..1);
      }
    };

    self.queue.submit(std::iter::once(encoder.finish()));
    Ok(())
  }
}

impl<W: AsWindow> Drop for Context<W> {
  fn drop(&mut self) {}
}

pub struct Builder<W: AsWindow> {
  size: Option<(i32, i32)>,
  window: W,
}

impl<W: AsWindow> Builder<W> {
  pub async fn build(mut self) -> Result<Context<W>, Error> {
    let instance = wgpu::Instance::new(wgpu::BackendBit::all());
    let surface = unsafe { instance.create_surface(&self.window) };
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
      .map_err(Error::from_error)?;
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: None,
      bind_group_layouts: &[],
      push_constant_ranges: &[],
    });
    let (w_width, w_height) = self.window.size();
    let sc_desc = wgpu::SwapChainDescriptor {
      usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
      format: adapter.get_swap_chain_preferred_format(&surface),
      width: w_width,
      height: w_height,
      present_mode: wgpu::PresentMode::Fifo,
    };
    let swapchain = device.create_swap_chain(&surface, &sc_desc);
    let main_vert_shader =
      device.create_shader_module(&wgpu::include_spirv!("shaders/main.vert.spv"));
    let main_frag_shader =
      device.create_shader_module(&wgpu::include_spirv!("shaders/main.frag.spv"));

    let render_pipeline = create_render_pipeline(
      &device,
      &sc_desc,
      &pipeline_layout,
      &main_vert_shader,
      &main_frag_shader,
    )?;

    let mesh = {
      let geom = MeshGeometry {
        vertices: TRIANGLE_VERT.to_vec(),
        indices: TRIANGLE_INDICES.to_vec(),
        label: Some("Triangle".to_string()),
      };
      Mesh::from_geometry(geom, &device)?
    };
    let mut uniforms = Uniforms::default();
    let camera = Camera::default();
    uniforms.update_from_camera(&camera);

    let uniform_buffer =
      device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Main UBO"),
        contents: bytemuck::cast_slice(&[uniforms.clone()]),
        usage: wgpu::BufferUsage::UNIFORM,
      });
    let result = Ok(Context {
      window: self.window,
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
      camera,
      uniforms,
      uniform_buffer,
    });
    result
  }

  pub fn with_size(mut self, size: (i32, i32)) -> Self {
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
    primitive: wgpu::PrimitiveState::default(),
    depth_stencil: None,
    multisample: wgpu::MultisampleState::default(),
  });

  Ok(render_pipeline)
}
