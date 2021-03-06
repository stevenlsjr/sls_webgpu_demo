use std::{
  fmt,
  fmt::Formatter,
  num::NonZeroU64,
  sync::{Arc, RwLock},
};

use anyhow::anyhow;

use raw_window_handle::HasRawWindowHandle;
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  BindGroup, BindGroupLayoutEntry, BufferDescriptor, BufferSize, BufferUsages, ColorTargetState,
  PrimitiveState, RenderPass, RenderPipeline, Texture,
};

use wgpu::Buffer;

use crate::{
  error::Error,
  game::{
    components::{LightSource, RenderModel, Transform3D},
    resources::Scene,
    GameState,
  },
  renderer_common::{
    allocator::ResourceManager,
    geometry::Vertex,
    handle::{Handle, HandleIndex},
    render_context::DrawModel,
    RenderContext,
  },
  wgpu::{BindGroupLayout, Device, PipelineLayout, TextureFormat},
  wgpu_renderer::{
    material::{Material, RenderMaterial, WgpuMaterial},
    model::{Model, StreamingMesh},
    pipeline_state::{create_render_pipeline, RendererPipelines},
    resource_view::ResourceContext,
    textures::{BindTexture, TextureResource},
    uniforms::{make_light_bind_group_layout, PointLightUniform},
    ModelInstance,
  },
  window::AsWindow,
};

use super::{mesh::Mesh, uniforms::Uniforms};
use crate::wgpu_renderer::pipeline_state::ShaderInfo;
use std::{
  borrow::{Borrow, BorrowMut},
  ops::Deref,
};

pub struct Context {
  pub instance: wgpu::Instance,
  pub surface: wgpu::Surface,
  pub adapter: wgpu::Adapter,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub pipeline_layout: wgpu::PipelineLayout,

  pipelines: RendererPipelines,

  // scene resources
  pub models_to_draw: Vec<Handle<StreamingMesh>>,
  uniforms: Uniforms,
  uniform_buffer: wgpu::Buffer,

  pub resources: ResourceContext,

  pub main_tex_handle: Option<Handle<TextureResource>>,
  pub(crate) fallback_texture: Handle<TextureResource>,
  pub texture_bind_group_layout: BindGroupLayout,
  pub diffuse_bind_group: BindGroup,
  pub uniform_bind_group_layout: BindGroupLayout,
  pub light_bind_group_layout: BindGroupLayout,
  pub light_bind_group: BindGroup,
  pub light_uniform_buffer: Buffer,

  pub(crate) default_material: Handle<WgpuMaterial>,

  uniform_bind_group: wgpu::BindGroup,

  /// Buffer storing instance state for render
  instance_buffer: wgpu::Buffer,
  n_instances: usize,
  instance_buffer_view: Vec<u8>,

  // Depth/stencil buffers
  depth_stencil_texture: TextureResource,
  pub surface_config: wgpu::SurfaceConfiguration,
}

impl fmt::Debug for Context {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("Context")
      .field("instance", &self.instance)
      .finish()
  }
}

impl Context {
  pub fn new<W: AsWindow>(window: &W) -> Builder<W> {
    Builder {
      window,
      instance: None,
      backends: None,
    }
  }

  pub fn on_resize(&mut self, size: (u32, u32)) {
    let (width, height) = size;
    self.surface_config.width = width;
    self.surface_config.height = height;
    self.surface.configure(&self.device, &self.surface_config);

    self.depth_stencil_texture =
      TextureResource::new_depth_stencil_texture(&self.device, size, "depth_stencil_texture");
    // self.swapchain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    //
    // self
    //   .pipelines
    //   .build_pipelines(&self.device, &*self.resources.shaders.read().unwrap())
    //   .unwrap();
  }

  pub fn update(&mut self) {}

  pub fn render(&mut self, game: &mut GameState) -> Result<(), anyhow::Error> {
    self.update_instance_state(game);
    let pbr_pipeline = match self.pipelines.pbr_model_pipeline.as_ref() {
      None => return Ok(()),
      Some(x) => x,
    };
    let camera = game
      .resources()
      .get::<Scene>()
      .map(|s| s.main_camera_components(&game.world()))
      .unwrap_or(Ok(None))
      .map_err(|error| anyhow!("error accessing camera {:?}", error))?;
    let camera = match camera {
      None => {
        log::warn!("no main camera found");
        return Ok(());
      }
      Some(camera) => camera,
    };
    self.uniforms.update_from_camera(camera);
    self.queue.write_buffer(
      &self.uniform_buffer,
      0,
      bytemuck::cast_slice(&[self.uniforms]),
    );

    let frame = self
      .surface
      .get_current_frame()
      .map_err(|e| anyhow!("swapchain error {:?}", e))?
      .output;

    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });
    let mesh_allocator = self.resources.meshes.read().unwrap();
    let model_allocator = self.resources.models.read().unwrap();
    let material_allocator = self.resources.materials.read().unwrap();

    {
      let frame_view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("render pass"),
        color_attachments: &[wgpu::RenderPassColorAttachment {
          view: &frame_view,
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
            store: true,
          }),
          stencil_ops: None,
        }),
      });

      render_pass.set_pipeline(pbr_pipeline);

      render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

      for m in &self.models_to_draw {
        let model = match model_allocator.try_get_ref(m.into_typed()) {
          Ok(e) => e,
          Err(_) => continue,
        };

        for mesh_handle in model.primitives() {
          let mesh = match mesh_handle.read(mesh_allocator.deref()) {
            Some(m) => m,
            None => {
              log::info!("mesh {:?} not found", mesh_handle);
              continue;
            }
          };
          mesh
            .material()
            .and_then(|handle| match material_allocator.try_get_ref(handle) {
              Ok(material) => Some(material),
              Err(e) => {
                log::warn!("could not access material: {:?}", e);
                None
              }
            })
            .map(|material| {
              let material_bg = match &material.bind_group {
                None => panic!("material does not have bind group attached"),
                Some(bg) => bg,
              };

              render_pass.draw_mesh_instanced(
                mesh,
                material_bg,
                &self.uniform_bind_group,
                0..(self.n_instances as u32),
              );
            });
        }
      }
    }
    self.queue.submit(std::iter::once(encoder.finish()));
    Ok(())
  }

  pub fn rebuild_render_pipeline(&mut self) {
    let shaders = self.resources.shaders.read().unwrap();
    if let Some(format) = self.surface.get_preferred_format(&self.adapter) {
      self.pipelines.color_target = format.clone().into();
      self
        .pipelines
        .build_pipelines(&self.device, &*shaders)
        .unwrap_or_else(|e| log::error!("could not rebuild pipelines {:?}", e));
    }
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

    let old_buffer_size: usize = binding
      .size
      .unwrap_or(unsafe { NonZeroU64::new_unchecked(1) })
      .get() as _;
    if old_buffer_size < buffer_data.len() {
      let new_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: buffer_data,
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
      });
      self.instance_buffer = new_buffer;
    } else {
      self
        .queue
        .write_buffer(&self.instance_buffer, 0_u64, buffer_data);
    }

    self
      .queue
      .write_buffer(&self.instance_buffer, 0, &bytemuck::cast_slice(&instances));
    self.n_instances = instances.len();
    self.instance_buffer_view = buffer_data.to_vec();
  }

  fn bind_light_sources(&mut self, game_state: &GameState) {
    use legion::*;
    let mut query = <(&LightSource, &Transform3D)>::query();
    query.for_each(game_state.world(), |(_a, _b)| {});
  }
}

pub struct Builder<'a, W: AsWindow + HasRawWindowHandle> {
  window: &'a W,
  instance: Option<wgpu::Instance>,
  backends: Option<wgpu::Backends>,
}

impl<'a, W: AsWindow + HasRawWindowHandle> Builder<'a, W> {
  pub async fn build(self) -> Result<Context, Error> {
    #[cfg(not(target_os = "linux"))]
    let backends = self.backends.unwrap_or(wgpu::Backends::VULKAN);
    #[cfg(target_os = "linux")]
    let backends = self.backends.unwrap_or(wgpu::Backends::VULKAN);

    log::info!("backend is {:?}", backends);

    let instance = self
      .instance
      .unwrap_or_else(|| wgpu::Instance::new(backends));

    let (width, height) = self.window.size();

    let surface = unsafe { instance.create_surface(self.window) };

    let mut resources = ResourceContext::default();

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
    let surface_config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_preferred_format(&adapter).unwrap(),
      width,
      height,
      present_mode: wgpu::PresentMode::Immediate,
    };
    surface.configure(&device, &surface_config);
    /// uniform buffer setup
    let uniforms = Uniforms::default();
    // let camera = self.camera;
    // uniforms.update_from_camera(&camera);

    let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("Main UBO"),
      contents: bytemuck::cast_slice(&[uniforms]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let ubo_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("ubo_layout"),
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
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
      let mut textures = resources.textures.write().unwrap();
      use super::textures::*;
      let img = image::load_from_memory(super::textures::DEFAULT_TEX_JPEG)
        .map_err(|e| Error::from_other(format!("{:?}", e)))?;
      let tex_resource = TextureResource::from_image(&img, &queue, &device)
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
      create_pipeline_layout(&device, &[&ubo_layout, &model_texture_bind_group_layout]);
    let (w_width, w_height) = self.window.size();
    let debug_light_vert_shader =
      device.create_shader_module(&wgpu::include_spirv!("../shaders/debug_light.vert.spv"));
    let debug_light_frag_shader =
      device.create_shader_module(&wgpu::include_spirv!("../shaders/debug_light.frag.spv"));
    let depth_stencil_texture =
      TextureResource::new_depth_stencil_texture(&device, (w_width, w_height), "depth_stencil_tex");
    let preferred_format = surface
      .get_preferred_format(&adapter)
      .ok_or_else(|| anyhow::anyhow!("could not get preferred texture format for surface"))?;
    // create render pipeline

    let pipelines = {
      let mut shaders = resources.shaders.write().unwrap();
      let debug_light_shaders = ShaderInfo::from_shader_descriptors(
        &device,
        shaders.borrow_mut(),
        &wgpu::include_spirv!("../shaders/debug_light.vert.spv"),
        &wgpu::include_spirv!("../shaders/debug_light.frag.spv"),
      );

      let pbr_model_shaders = ShaderInfo::from_shader_descriptors(
        &device,
        shaders.borrow_mut(),
        &wgpu::include_spirv!("../shaders/main.vert.spv"),
        &wgpu::include_spirv!("../shaders/main.frag.spv"),
      );

      let mut pipelines = RendererPipelines::new(
        &device,
        &[&ubo_layout, &model_texture_bind_group_layout],
        debug_light_shaders,
        &[&ubo_layout, &model_texture_bind_group_layout],
        pbr_model_shaders,
        preferred_format.into(),
      );

      pipelines.build_pipelines(&device, shaders.borrow())?;

      pipelines
    };

    log::info!("I'm alive {}", std::line!());

    // create default mesh to draw

    let instance_buffer = {
      let instance_data: &[ModelInstance] = &[];
      device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&instance_data),
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
      })
    };
    let default_material = {
      let mut textures = resources.textures.write().unwrap();
      WgpuMaterial::from_material(
        &Material::default(),
        &queue,
        &device,
        &model_texture_bind_group_layout,
        &mut textures,
        fallback_texture,
      )
    }?;
    let default_material = {
      let mut materials = resources.materials.write().unwrap();
      materials.insert(default_material)
    };

    let (light_uniform_buffer, light_bind_group, light_bind_group_layout) =
      Self::create_light_bindings(&device);

    let mut result = Context {
      surface,
      surface_config,
      instance,
      adapter,
      device,
      queue,
      pipeline_layout,
      pipelines,
      models_to_draw: Vec::new(),

      uniforms,
      uniform_buffer,
      uniform_bind_group_layout: ubo_layout,
      uniform_bind_group,
      texture_bind_group_layout: model_texture_bind_group_layout,
      diffuse_bind_group,
      resources,
      main_tex_handle: None,
      fallback_texture,

      instance_buffer,
      instance_buffer_view: Vec::new(),

      n_instances: 0,

      depth_stencil_texture,

      default_material,
      light_uniform_buffer,
      light_bind_group,
      light_bind_group_layout,
    };
    result
      .bind_texture(*fallback_texture)
      .map_err(|e| Error::from_other(format!("{:?}", e)))?;
    Ok(result)
  }

  pub fn with_instance(mut self, instance: wgpu::Instance) -> Self {
    self.instance = Some(instance);
    self
  }

  pub fn with_backends(mut self, backends: Option<wgpu::Backends>) -> Self {
    self.backends = backends;
    self
  }

  fn create_light_bindings(
    device: &wgpu::Device,
  ) -> (wgpu::Buffer, wgpu::BindGroup, wgpu::BindGroupLayout) {
    let light_uniform = PointLightUniform {
      position: [2.0, 2.0, 2.0],
      _padding: 0,
      color: [1.0, 1.0, 1.0],
    };

    // We'll want to update our lights position, so we use COPY_DST
    let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Light VB"),
      contents: bytemuck::cast_slice(&[light_uniform]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let layout = make_light_bind_group_layout(device);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: None,
      layout: &layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: light_buffer.as_entire_binding(),
      }],
    });
    (light_buffer, bind_group, layout)
  }
}

pub fn create_pipeline_layout(
  device: &Device,
  bind_group_layouts: &[&BindGroupLayout],
) -> PipelineLayout {
  let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some("pipeline_layout"),
    bind_group_layouts,
    push_constant_ranges: &[],
  });
  pipeline_layout
}

pub fn create_bind_group(device: &Device) {
  device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    label: Some(concat!(std::file!(), ":BindGroupLayout")),
    entries: &[],
  });
}

impl RenderContext for Context {
  fn on_render(&mut self, game: &mut GameState) -> Result<(), Error> {
    self.render(game).map_err(crate::Error::Render)
  }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
  use web_sys::HtmlCanvasElement;

  use crate::platform::html5::FromCanvas;

  use super::*;
}
