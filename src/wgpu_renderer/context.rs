use crate::{
  error::Error,
  game::{resources::Scene, GameState},
};
use anyhow::{anyhow};

use crate::window::AsWindow;

use super::{
  mesh::{Mesh, MeshGeometry},
  uniforms::Uniforms,
};
use crate::renderer_common::geometry::Vertex;

use crate::{
  renderer_common::allocator::{Handle, ResourceManager},
  wgpu::{BindGroupLayout, Device, PipelineLayout},
  wgpu_renderer::{
    textures::{BindTexture, TextureResource},
    ModelInstance,
  },
};
use std::{
  fmt,
  fmt::Formatter,
  sync::{Arc, RwLock},
};
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroup, BindGroupLayoutEntry, BufferDescriptor, PrimitiveState, RenderPass, RenderPipeline, Texture, BufferUsage, BufferSize};
use crate::game::components::{Transform3D, RenderModel};
use legion::query::ChunkView;
use imgui::StyleColor::TableRowBgAlt;
use std::num::NonZeroU64;
use crate::wgpu::TextureFormat;

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
  // scene resources
  mesh: Mesh,
  uniforms: Uniforms,
  uniform_buffer: wgpu::Buffer,

  main_vert_shader: wgpu::ShaderModule,
  main_frag_shader: wgpu::ShaderModule,

  pub main_tex_handle: Option<Handle>,
  fallback_texture: Handle,

  pub meshes: Arc<RwLock<ResourceManager<Mesh>>>,
  pub textures: Arc<RwLock<ResourceManager<TextureResource>>>,
  pub texture_bind_group_layout: BindGroupLayout,
  pub diffuse_bind_group: BindGroup,
  pub uniform_bind_group_layout: BindGroupLayout,
  uniform_bind_group: wgpu::BindGroup,

  /// Buffer storing instance state for render
  instance_buffer: wgpu::Buffer,
  n_instances: usize,
  instance_buffer_view: Vec<u8>,

  // Depth/stencil buffers
  depth_stencil_texture: TextureResource,
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
    self.depth_stencil_texture = TextureResource::new_depth_stencil_texture(
      &self.device,
      &self.sc_desc,
      "depth_stencil_texture",
    );
    self.swapchain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
  }

  pub fn update(&mut self) {}

  #[cfg(feature = "wgpu_imgui")]
  pub fn render_with_ui(
    &mut self,
    game: &GameState,
    mut imgui_frame: imgui::Ui,
    imgui_renderer: &mut imgui_wgpu::Renderer,
  ) -> Result<(), anyhow::Error> {
    self.update_instance_state(game);
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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
              view: self.depth_stencil_texture.view(),
              depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true
              }),
              stencil_ops: None
            }),
          });

          render_pass.set_pipeline(&self.render_pipeline);
          render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
          render_pass.set_bind_group(1, &self.diffuse_bind_group, &[]);

          if let Some(mesh) = self.mesh.buffers() {
            let n_indices = self.mesh.geometry().indices.len() as u32;
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            // instance matrix data
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);


            render_pass.draw_indexed(0..n_indices, 0, 0..(self.n_instances as u32));
          };
          // {
          //   let draw_data = imgui_frame.render();
          //   imgui_renderer.render(draw_data, &self.queue, &self.device, &mut render_pass)?;
          // }
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
      }
    }
  }
  pub fn rebuild_render_pipeline(&mut self) {
    self.pipeline_layout = create_pipeline_layout(
      &self.device,
      &self.uniform_bind_group_layout,
      &self.texture_bind_group_layout,
    );
    self.render_pipeline = create_render_pipeline(
      &self.device,
      &self.sc_desc,
      &self.pipeline_layout,
      &self.main_vert_shader,
      &self.main_frag_shader,
    );
  }
  /// get instance data from game state
  fn update_instance_state(&mut self, game: &GameState) {
    use legion::*;
    let mut query = <(&Transform3D, &RenderModel)>::query();
    let mut instances: Vec<ModelInstance> = Vec::with_capacity(10);
    for item in query.iter(game.world()) {
      let (xform, model): (&Transform3D, &RenderModel) = item;
      if model.is_shown {
        instances.push(xform.into());
      }
    }
    let binding = self.instance_buffer.as_entire_buffer_binding();
    let buffer_data: &[u8] = bytemuck::cast_slice(&instances);
    if self.instance_buffer_view == buffer_data {
      // if instances haven't changed, don't update buffers
      return;
    }

    let old_buffer_size: usize = binding.size
      .unwrap_or(unsafe { NonZeroU64::new_unchecked(1) }).get() as _;
    if old_buffer_size < buffer_data.len() {
      let new_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: buffer_data,
        usage: BufferUsage::VERTEX | BufferUsage::COPY_DST,
      });
      self.instance_buffer = new_buffer;
    } else {
      self.queue.write_buffer(&self.instance_buffer, 0 as _, buffer_data);
    }


    self.queue.write_buffer(&self.instance_buffer, 0, &bytemuck::cast_slice(&instances));
    self.n_instances = instances.len();
    self.instance_buffer_view = buffer_data.iter().cloned().collect();
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
    let mut textures: ResourceManager<TextureResource> = Default::default();
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

    /// uniform buffer setup
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
    // default diffuse texture setup
    let model_texture_bind_group_layout =
      super::textures::create_texture_bind_group_layout(&device);
    let (fallback_texture, diffuse_bind_group) = {
      use super::textures::*;
      let img = image::load_from_memory(super::textures::DEFAULT_TEX_JPEG)
        .map_err(|e| Error::from_other(format!("{:?}", e)))?;
      let tex_resource = TextureResource::from_image(img, &queue, &device)
        .map_err(|e| Error::from_other(format!("{:?}", e)))?;
      let bg = super::textures::basic_texture_bind_group(
        &tex_resource,
        &model_texture_bind_group_layout,
        &device,
      );
      (textures.insert(tex_resource), bg)
    };

    // setup pipeline and swapchain

    let pipeline_layout =
      create_pipeline_layout(&device, &ubo_layout, &model_texture_bind_group_layout);
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

    let depth_stencil_texture = TextureResource::new_depth_stencil_texture(&device, &sc_desc, "depth_stencil_tex");

    // create render pipeline
    let render_pipeline = create_render_pipeline(
      &device,
      &sc_desc,
      &pipeline_layout,
      &main_vert_shader,
      &main_frag_shader,
    );

    // create default mesh to draw

    let mesh = {
      let geom = MeshGeometry::unit_plane();
      Mesh::from_geometry(geom, &device)?
    };

    let instance_buffer = {
      let instance_data: &[ModelInstance] = &[];
      device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&instance_data),
        usage: BufferUsage::VERTEX | BufferUsage::COPY_DST,
      })
    };

    let mut result = Context {
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
      uniform_bind_group_layout: ubo_layout,
      uniform_bind_group,
      texture_bind_group_layout: model_texture_bind_group_layout,
      diffuse_bind_group,
      meshes: Default::default(),
      textures: Arc::new(RwLock::new(textures)),
      main_tex_handle: None,
      fallback_texture,
      main_frag_shader,
      main_vert_shader,
      instance_buffer,
      instance_buffer_view: Vec::new(),

      n_instances: 0,

      depth_stencil_texture,
    };
    result
      .bind_texture(fallback_texture)
      .map_err(|e| Error::from_other(format!("{:?}", e)))?;
    Ok(result)
  }

  pub fn with_size(mut self, size: (i32, i32)) -> Self {
    self.size = Some(size);
    self
  }
}

pub fn create_pipeline_layout(
  device: &Device,
  ubo_layout: &BindGroupLayout,
  texture_layout: &BindGroupLayout,
) -> PipelineLayout {
  let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some("pipeline_layout"),
    bind_group_layouts: &[ubo_layout, texture_layout],
    push_constant_ranges: &[],
  });
  pipeline_layout
}

fn create_render_pipeline(
  device: &wgpu::Device,
  sc_desc: &wgpu::SwapChainDescriptor,
  layout: &wgpu::PipelineLayout,
  vert_shader: &wgpu::ShaderModule,
  frag_shader: &wgpu::ShaderModule,
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
      targets: &[sc_desc.format.into()],
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

pub fn createe_bind_group(device: &Device) {
  device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    label: Some(concat!(std::file!(), ":BindGroupLayout")),
    entries: &[],
  });
}

#[cfg(target_arch = "wasm32")]
mod wasm {
  use super::*;
  use crate::platform::html5::FromCanvas;
  use web_sys::HtmlCanvasElement;
}
