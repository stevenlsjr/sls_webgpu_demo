use image::{RgbaImage, DynamicImage, ImageBuffer, Bgr};
use crate::image::RgbImage;
use crate::nalgebra_glm::{Vec4, vec4, vec3, Vec3};
use crate::gltf::Image;
use crate::gltf::image::{Source, Format};
use crate::renderer_common::handle::{HandleIndex, Handle};
use crate::Context;
use crate::wgpu_renderer::textures::TextureResource;
use crate::wgpu::{Queue, Device, BindingResource, BindGroup};
use crate::wgpu_renderer::resource_view::ReadWriteResources;
use crate::renderer_common::allocator::ResourceManager;
use wgpu::{BindGroupLayout, BindGroupDescriptor, BindGroupEntry};


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
  pub fn from_gltf(gltf_sampler: &gltf::texture::Sampler) -> Self {
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
  pub fn load_texture(&mut self,
                      textures: &mut ResourceManager<TextureResource>,
                      queue: &Queue,
                      device: &Device) -> anyhow::Result<()> {
    match (self.texture_resource_handle, self.rgba.as_ref()) {
      (Some(handle), _) => Ok(()),
      (None, Some(rbga)) => {
        let texture = TextureResource::from_image(&rbga,
                                                  queue, device)?;
        let texture_handle = textures.insert(texture);
        self.texture_resource_handle = Some(texture_handle);
        Ok(())
      }
      (None, None) => anyhow::bail!("rgba data is missing!")
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
    new_mat.metallic_roughness_tex = texture_from_info(pbr.metallic_roughness_texture().as_ref(), images)?;
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


fn texture_from_info(info: Option<&gltf::texture::Info>,
                     images: &[gltf::image::Data]) -> anyhow::Result<Option<TextureInfoData>> {
  match info {
    None => Ok(None),
    Some(info) => {
      let tex = info.texture();

      let index = tex.index();
      let tex_coord_index = info.tex_coord();
      let name: Option<String> = tex.name().map(&str::to_owned);
      let rgba = rgba_from_texture(&tex, images)?;

      let sampler = Sampler {};
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

fn rgba_from_texture(tex: &gltf::Texture, images: &[gltf::image::Data])
                     -> anyhow::Result<DynamicImage> {
  let img_index = tex.source().index();
  if img_index > images.len() {
    anyhow::bail!("image index {} exceded images loaded {}", img_index, images.len());
  }
  let img = &images[img_index];
  let dyn_image = match img.format {
    Format::R8 => {
      image::GrayImage::from_raw(img.width, img.height, img.pixels.clone())
        .map(|buff| DynamicImage::ImageLuma8(buff))
    }
    Format::R8G8 => {
      image::GrayAlphaImage::from_raw(img.width, img.height, img.pixels.clone())
        .map(|buff| DynamicImage::ImageLumaA8(buff))
    }
    Format::R8G8B8 => {
      image::RgbImage::from_raw(img.width, img.height, img.pixels.clone())
        .map(|buff| DynamicImage::ImageRgb8(buff))
    }
    Format::R8G8B8A8 => {
      image::RgbaImage::from_raw(img.width, img.height, img.pixels.clone())
        .map(|buff| DynamicImage::ImageRgba8(buff))
    }
    Format::B8G8R8 => {
      ImageBuffer::<Bgr<u8>, Vec<u8>>::from_raw(img.width, img.height, img.pixels.clone())
        .map(|buff| DynamicImage::ImageBgr8(buff))
    }
    Format::B8G8R8A8 => {
      ImageBuffer::<image::Bgra<u8>, Vec<u8>>::from_raw(img.width, img.height, img.pixels.clone())
        .map(|buff| DynamicImage::ImageBgra8(buff))
    }
    fmt => anyhow::bail!("format {:?} not supported. Only supports 8 bit images right now", fmt)
    // Format::R16 => {
    // }
    // Format::R16G16 => {None}
    // Format::R16G16B16 => {None}
    // Format::R16G16B16A16 => {None}
  }.ok_or_else(|| anyhow::anyhow!("could not create image buffer for image"))?;
  Ok(dyn_image)
}


#[derive(Debug)]
pub struct WgpuMaterial {
  pub double_sided: bool,
  pub index: usize,
  pub name: Option<String>,
  ///
  ///
  pub alpha_cutoff: Option<f32>,
  pub alpha_mode: AlphaMode,
  pub albedo_factor: Vec4,
  pub albedo_tex: Option<Handle<TextureResource>>,

  pub normal_tex: Option<Handle<TextureResource>>,
  pub metallic_factor: f32,
  pub roughness_factor: f32,
  pub metallic_roughness_tex: Option<Handle<TextureResource>>,

  pub occlusion_tex: Option<Handle<TextureResource>>,

  /// index of reflection
  pub ior: Option<f32>,
  pub transmission_factor: Option<f32>,
  pub transmission_tex: Option<Handle<TextureResource>>,

  pub emissive_factor: Vec3,
  pub emissive_tex: Option<Handle<TextureResource>>,

  pub bind_group: Handle<TextureResource>,
}

impl WgpuMaterial {
  pub fn from_material(material: &Material,
                       queue: &Queue,
                       device: &Device,
                       bind_group_layout: &BindGroupLayout,
                       textures: &mut ResourceManager<TextureResource>,
  ) -> anyhow::Result<Self> {
    let texture_infos = &[
      &material.albedo_tex,
      // &material.metallic_roughness_tex,
      // &material.transmission_tex,
      // &material.emissive_tex,
      // &material.occlusion_tex,
      // &material.normal_tex
    ];
    let mut texture_handles: [Option<Handle<TextureResource>>; 1] = [None; 1];
    for (i, &info_opt) in texture_infos.iter().enumerate() {
      let get_tex = info_opt
        .as_ref()
        .map(|info| (info, &info.rgba));
      match get_tex {
        Some((info, Some(rgba))) => {
          let resource = TextureResource::from_image(rgba, queue, device)?;
          let handle = textures.insert(resource);
          texture_handles[i] = Some(handle);
        }
        _ => ()
      }
    }
    todo!()
  }
}