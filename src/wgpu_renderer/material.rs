use crate::{
  renderer_common::{allocator::ResourceManager, handle::Handle},
  wgpu_renderer::textures::{basic_texture_bind_group, TextureResource},
};
use gltf::image::Format;
use image::{Bgr, DynamicImage, ImageBuffer};
use nalgebra_glm::{vec3, vec4, Vec3, Vec4};

use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Device, Queue};

#[derive(Debug, Copy, Clone)]
pub enum AlphaMode {
  Opaque,
  Mask,
  Blend,
}

impl From<gltf::material::AlphaMode> for AlphaMode {
  fn from(mode: gltf::json::material::AlphaMode) -> Self {
    match mode {
      gltf::material::AlphaMode::Opaque => Self::Opaque,
      gltf::material::AlphaMode::Mask => Self::Mask,
      gltf::material::AlphaMode::Blend => Self::Blend,
    }
  }
}

#[derive(Debug)]
pub struct Sampler {}

impl Sampler {
  pub fn from_gltf(_gltf_sampler: &gltf::texture::Sampler) -> Self {
    Self {}
  }
}

#[derive(Debug)]
pub struct TextureInfoData {
  pub rgba: Option<DynamicImage>,
  pub tex_coord_index: u32,
  pub name: Option<String>,
  pub sampler: Sampler,
  pub index: usize,
  pub scale_or_strength: f32,
  pub texture_resource_handle: Option<Handle<TextureResource>>,
}

impl TextureInfoData {
  pub fn load_texture(
    &mut self,
    textures: &mut ResourceManager<TextureResource>,
    queue: &Queue,
    device: &Device,
  ) -> anyhow::Result<()> {
    match (self.texture_resource_handle, self.rgba.as_ref()) {
      (Some(_handle), _) => Ok(()),
      (None, Some(rbga)) => {
        let texture = TextureResource::from_image(&rbga, queue, device)?;
        let texture_handle = textures.insert(texture);
        self.texture_resource_handle = Some(texture_handle);
        Ok(())
      }
      (None, None) => anyhow::bail!("rgba data is missing!"),
    }
  }
}

#[derive(Debug)]
pub struct Material {
  pub double_sided: bool,
  pub index: usize,
  pub name: Option<String>,
  ///
  ///
  pub alpha_cutoff: Option<f32>,
  pub alpha_mode: AlphaMode,
  pub albedo_factor: Vec4,
  pub albedo_tex: Option<TextureInfoData>,

  pub normal_tex: Option<TextureInfoData>,
  pub metallic_factor: f32,
  pub roughness_factor: f32,
  pub metallic_roughness_tex: Option<TextureInfoData>,

  pub occlusion_tex: Option<TextureInfoData>,

  /// index of reflection
  pub ior: Option<f32>,
  pub transmission_factor: Option<f32>,
  pub transmission_tex: Option<TextureInfoData>,

  pub emissive_factor: Vec3,
  pub emissive_tex: Option<TextureInfoData>,
}

impl Default for Material {
  fn default() -> Self {
    Self {
      double_sided: false,
      index: 0,
      name: None,
      alpha_cutoff: None,
      alpha_mode: AlphaMode::Opaque,
      albedo_factor: vec4(1.0, 1.0, 1.0, 1.0),
      albedo_tex: None,
      normal_tex: None,
      metallic_factor: 0.0,
      roughness_factor: 0.0,
      metallic_roughness_tex: None,
      occlusion_tex: None,
      ior: None,
      transmission_factor: None,
      transmission_tex: None,
      emissive_factor: vec3(0.0, 0.0, 0.0),
      emissive_tex: None,
    }
  }
}

impl Material {
  pub fn from_gltf(
    document: &gltf::Document,
    images: &[gltf::image::Data],
  ) -> anyhow::Result<Vec<Self>> {
    let mut materials = Vec::new();
    for i in document.materials() {
      materials.push(Self::from_gltf_material(&i, images)?);
    }
    Ok(materials)
  }

  pub fn from_gltf_material(
    material: &gltf::Material,
    images: &[gltf::image::Data],
  ) -> anyhow::Result<Self> {
    let pbr = material.pbr_metallic_roughness();
    let mut new_mat: Self = Self {
      double_sided: false,
      index: material.index().unwrap_or(0),
      name: material.name().map(|s| s.to_owned()),
      alpha_cutoff: material.alpha_cutoff(),
      alpha_mode: material.alpha_mode().into(),

      albedo_factor: pbr.base_color_factor().into(),
      albedo_tex: None,
      normal_tex: None,
      metallic_factor: pbr.metallic_factor(),
      roughness_factor: pbr.roughness_factor(),
      metallic_roughness_tex: None,
      occlusion_tex: None,
      ior: None,
      transmission_factor: None,
      transmission_tex: None,
      emissive_factor: material.emissive_factor().into(),
      emissive_tex: None,
      ..Default::default()
    };
    // if let Some(tx) = material.transmission() {
    //   new_mat.transmission_factor = tx.transmission_factor();
    //   new_mat.transmission_tex = texture_from_info(tx.transmission_tex().as_ref(), images)?;
    // }

    new_mat.albedo_tex = texture_from_info(pbr.base_color_texture().as_ref(), images)?;
    new_mat.metallic_roughness_tex =
      texture_from_info(pbr.metallic_roughness_texture().as_ref(), images)?;
    new_mat.emissive_tex = texture_from_info(material.emissive_texture().as_ref(), images)?;
    if let Some(occlusion) = material.occlusion_texture() {
      let tex = occlusion.texture();
      new_mat.occlusion_tex = Some(TextureInfoData {
        rgba: Some(rgba_from_texture(&tex, images)?),
        tex_coord_index: occlusion.tex_coord(),
        name: tex.name().map(&str::to_owned),
        sampler: Sampler::from_gltf(&tex.sampler()),
        index: tex.index(),
        scale_or_strength: occlusion.strength(),
        texture_resource_handle: None,
      });
    }

    if let Some(normal) = material.normal_texture() {
      let tex = normal.texture();
      new_mat.occlusion_tex = Some(TextureInfoData {
        rgba: Some(rgba_from_texture(&tex, images)?),
        tex_coord_index: normal.tex_coord(),
        name: tex.name().map(&str::to_owned),
        sampler: Sampler::from_gltf(&tex.sampler()),
        index: tex.index(),
        scale_or_strength: normal.scale(),
        texture_resource_handle: None,
      });
    }

    Ok(new_mat)
  }
}

fn texture_from_info(
  info: Option<&gltf::texture::Info>,
  images: &[gltf::image::Data],
) -> anyhow::Result<Option<TextureInfoData>> {
  match info {
    None => Ok(None),
    Some(info) => {
      let tex = info.texture();

      let index = tex.index();
      let tex_coord_index = info.tex_coord();
      let name: Option<String> = tex.name().map(&str::to_owned);
      let rgba = rgba_from_texture(&tex, images)?;

      let _sampler = Sampler {};
      Ok(Some(TextureInfoData {
        rgba: Some(rgba),
        index,
        tex_coord_index,
        name,
        sampler: Sampler::from_gltf(&tex.sampler()),
        scale_or_strength: 0.0,
        texture_resource_handle: None,
      }))
    }
  }
}

fn rgba_from_texture(
  tex: &gltf::Texture,
  images: &[gltf::image::Data],
) -> anyhow::Result<DynamicImage> {
  let img_index = tex.source().index();
  if img_index > images.len() {
    anyhow::bail!(
      "image index {} exceded images loaded {}",
      img_index,
      images.len()
    );
  }
  let img = &images[img_index];
  let dyn_image = match img.format {
    Format::R8 => image::GrayImage::from_raw(img.width, img.height, img.pixels.clone())
      .map(DynamicImage::ImageLuma8),
    Format::R8G8 => image::GrayAlphaImage::from_raw(img.width, img.height, img.pixels.clone())
      .map(DynamicImage::ImageLumaA8),
    Format::R8G8B8 => image::RgbImage::from_raw(img.width, img.height, img.pixels.clone())
      .map(DynamicImage::ImageRgb8),
    Format::R8G8B8A8 => image::RgbaImage::from_raw(img.width, img.height, img.pixels.clone())
      .map(DynamicImage::ImageRgba8),
    Format::B8G8R8 => {
      ImageBuffer::<Bgr<u8>, Vec<u8>>::from_raw(img.width, img.height, img.pixels.clone())
        .map(DynamicImage::ImageBgr8)
    }
    Format::B8G8R8A8 => {
      ImageBuffer::<image::Bgra<u8>, Vec<u8>>::from_raw(img.width, img.height, img.pixels.clone())
        .map(DynamicImage::ImageBgra8)
    }
    fmt => anyhow::bail!(
      "format {:?} not supported. Only supports 8 bit images right now",
      fmt
    ), // Format::R16 => {
       // }
       // Format::R16G16 => {None}
       // Format::R16G16B16 => {None}
       // Format::R16G16B16A16 => {None}
  }
  .ok_or_else(|| anyhow::anyhow!("could not create image buffer for image"))?;
  Ok(dyn_image)
}

#[derive(Debug)]
pub struct RenderMaterial<TextureT: 'static> {
  pub double_sided: bool,
  pub index: usize,
  pub name: Option<String>,
  ///
  ///
  pub alpha_cutoff: Option<f32>,
  pub alpha_mode: AlphaMode,
  pub albedo_factor: Vec4,
  pub albedo_tex: Option<Handle<TextureT>>,

  pub normal_tex: Option<Handle<TextureT>>,
  pub metallic_factor: f32,
  pub roughness_factor: f32,
  pub metallic_roughness_tex: Option<Handle<TextureT>>,

  pub occlusion_tex: Option<Handle<TextureT>>,

  /// index of reflection
  pub ior: Option<f32>,
  pub transmission_factor: Option<f32>,
  pub transmission_tex: Option<Handle<TextureT>>,

  pub emissive_factor: Vec3,
  pub emissive_tex: Option<Handle<TextureT>>,

  pub bind_group: Option<wgpu::BindGroup>,
}

pub type WgpuMaterial = RenderMaterial<TextureResource>;

impl RenderMaterial<TextureResource> {
  ///
  /// @param default_texture. Texture handle to use for bind groups if
  /// material does not have defined texture.
  pub fn from_material<'ax>(
    material: &Material,
    queue: &Queue,
    device: &Device,
    bind_group_layout: &BindGroupLayout,
    textures: &mut ResourceManager<TextureResource>,
    default_texture: Handle<TextureResource>,
  ) -> anyhow::Result<Self> {
    let mut gpu_resource = Self {
      double_sided: material.double_sided,
      index: material.index,
      name: material.name.clone(),
      alpha_cutoff: material.alpha_cutoff,
      alpha_mode: material.alpha_mode,
      albedo_factor: material.albedo_factor,
      albedo_tex: None,
      normal_tex: None,
      metallic_factor: material.metallic_factor,
      roughness_factor: material.roughness_factor,
      metallic_roughness_tex: None,
      occlusion_tex: None,
      ior: material.ior,
      transmission_factor: material.transmission_factor,
      transmission_tex: None,
      emissive_factor: material.emissive_factor,
      emissive_tex: None,
      bind_group: None,
    };
    let mut texture_infos = [
      (&material.albedo_tex, &mut gpu_resource.albedo_tex),
      (
        &material.metallic_roughness_tex,
        &mut gpu_resource.metallic_roughness_tex,
      ),
      // &material.metallic_roughness_tex,
      // &material.transmission_tex,
      // &material.emissive_tex,
      // &material.occlusion_tex,
      // &material.normal_tex
    ];
    for (info_opt, gpu_tex) in texture_infos.iter_mut() {
      let get_tex = info_opt.as_ref().map(|info| (info, &info.rgba));
      match get_tex {
        Some((_info, Some(rgba))) => {
          let resource = TextureResource::from_image(rgba, queue, device)?;
          let handle = textures.insert(resource);
          **gpu_tex = Some(handle)
        }
        _ => (),
      }
    }

    gpu_resource.init_bind_group(queue, device, textures, default_texture, bind_group_layout)?;
    Ok(gpu_resource)
  }
  fn init_bind_group(
    &mut self,
    _queue: &Queue,
    device: &Device,
    textures: &ResourceManager<TextureResource>,
    default_texture: Handle<TextureResource>,
    layout: &BindGroupLayout,
  ) -> anyhow::Result<()> {
    let albedo_tex = textures.get_ref(self.albedo_tex.unwrap_or(default_texture))?;
    let bind_group = basic_texture_bind_group(albedo_tex, layout, device);
    self.bind_group = Some(bind_group);
    Ok(())
  }
}
