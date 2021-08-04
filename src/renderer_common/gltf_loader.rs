use super::geometry::{MeshGeometry, Vertex};
use gltf::Primitive;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GltfLoaderError {
  #[error("caused by error {0:?}")]
  FromAnyErr(#[from] anyhow::Error),

  #[error("unsupported format: '{reason}'")]
  UnsupportedFormat { reason: String },
}

impl GltfLoaderError {
  pub fn unsupported_format(reason: String) -> Self {
    Self::UnsupportedFormat { reason }
  }
}

pub trait LoadPrimitive: Sized {
  fn load_primitive(
    primitive: &gltf::Primitive,
    buffers: &[gltf::buffer::Data],
  ) -> Result<Self, GltfLoaderError>;
}

impl LoadPrimitive for MeshGeometry {
  fn load_primitive(
    primitive: &Primitive,
    buffers: &[gltf::buffer::Data],
  ) -> Result<Self, GltfLoaderError> {
    let mut verts: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();
    let reader = primitive.reader(|buffer_data| Some(&buffers[buffer_data.index()]));
    let read_positions = reader
      .read_positions()
      .ok_or(GltfLoaderError::unsupported_format(
        "Primitives must have a POSITION attribute".to_owned(),
      ))?;

    let positions: Vec<_> = read_positions.into_iter().collect();
    let mut normals = reader.read_normals();
    let mut tangents = reader.read_tangents();

    for (i, position) in positions.iter().enumerate() {
      let normal = normals
        .as_mut()
        .and_then(|mut iter| iter.next())
        .unwrap_or([0.0, 1.0, 0.0]);
      let tangent = tangents
        .as_mut()
        .and_then(|mut iter| iter.next())
        .unwrap_or([0.0, 1.0, 0.0, 1.0]);
      verts.push(Vertex {
        position: *position,
        normal: normal,
        tangent: tangent,
        ..Default::default()
      })
    }
    let mut tex_coord_set = 0;
    while let Some(tex_coords) = reader.read_tex_coords(tex_coord_set) {
      let current_set = tex_coord_set;
      tex_coord_set += 1;
      if current_set >= 1 {
        log::warn!("This renderer only supports one tex coord set");
        continue;
      }
      for (i, tex_coord) in tex_coords.into_f32().enumerate() {
        match current_set {
          0 => verts[i].uv = tex_coord.clone(),
          _ => unreachable!(),
        }
      }
    }

    let mut color_set = 0;

    while let Some(colors) = reader.read_colors(color_set) {
      let current_set = color_set;
      color_set += 1;
      if current_set >= 1 {
        log::warn!("This renderer only supports one tex coord set");
        continue;
      }
      for (i, color) in colors.into_rgba_f32().enumerate() {
        match current_set {
          0 => verts[i].color = color.clone(),
          _ => unreachable!(),
        }
      }
    }
    todo!()
  }
}
