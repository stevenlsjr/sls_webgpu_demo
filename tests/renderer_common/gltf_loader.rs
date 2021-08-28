use gltf::Primitive;
use sls_webgpu::renderer_common::{geometry::MeshGeometry, gltf_loader::LoadPrimitive};
use std::path::{Path, PathBuf};

fn relative_path<P: AsRef<Path>>(p: P) -> Option<PathBuf> {
  Path::new(file!()).parent().map(|parent| parent.join(p))
}

#[test]
fn test_render_primitive() {
  let path = relative_path("./simple_meshes.gltf").unwrap();
  let (doc, buffs, _images) = gltf::import(&path).expect("could not load gltf doc");
  let primitive: &Primitive = &doc.meshes().nth(0).unwrap().primitives().nth(0).unwrap();
  let mesh = <MeshGeometry as LoadPrimitive>::load_primitive(&primitive, &buffs);
  assert!(mesh.is_ok());
  let mesh = mesh.unwrap();
  let positions = mesh
    .vertices
    .iter()
    .map(|v| v.position.clone())
    .collect::<Vec<_>>();

  let expected: Vec<[f32; 3]> = Vec::new();
  assert_eq!(positions, expected);
}
