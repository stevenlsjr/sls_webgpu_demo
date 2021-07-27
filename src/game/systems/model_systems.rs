use anyhow::{anyhow, Error as AnyError, Result as AnyResult};

use crate::{
  game::components::{RenderModel, Transform3D},
  renderer_common::allocator::Handle,
};
use rand::{distributions::Uniform, prelude::IteratorRandom, random, seq::SliceRandom, Rng};
#[cfg(feature = "wgpu_renderer")]
pub use wgpu_renderer::*;

#[cfg(feature = "wgpu_renderer")]
mod wgpu_renderer {
  use super::*;
  use crate::{
    game::{components::RenderModel, resources::MeshLookup},
    renderer_common::allocator::Handle,
    wgpu_renderer::mesh::{Mesh, MeshGeometry},
    Context,
  };
  use legion::{systems::CommandBuffer, world::SubWorld, *};
  use rand::seq::SliceRandom;
  use std::{
    borrow::BorrowMut,
    sync::{Arc, RwLock},
  };

  fn load_mesh_lookup<C: BorrowMut<Context>>(context: C, lookup: &mut MeshLookup) -> AnyResult<()> {
    Ok(())
  }

  #[system]
  pub fn create_models(
    #[resource] context: &Arc<RwLock<Context>>,
    #[resource] mesh_lookup: &mut MeshLookup,
    cmd: &mut CommandBuffer,
  ) {
    let res = context
      .write()
      .map_err(|e| anyhow!("Could not access context RwLock: Poisoned {:?}", e))
      .and_then(|context| {
        let mut meshes = context
          .meshes
          .write()
          .map_err(|e| anyhow!("Could not access context RwLock: Poisoned {:?}", e))?;
        let cube = Mesh::from_geometry(MeshGeometry::cube(), &context.device)?;
        let cube = meshes.insert(cube);
        let sphere = Mesh::from_geometry(MeshGeometry::unit_shere(30, 30), &context.device)?;
        let sphere = meshes.insert(sphere);

        mesh_lookup.0.insert("sphere".to_owned(), sphere);
        mesh_lookup.0.insert("cube".to_owned(), cube);

        let mesh_choices = [cube, sphere];
        let n_models = 10usize;
        for i in 0..n_models {
          let components = create_random_model(&mesh_choices);
          cmd.push(components);
        }
        Ok(())
      });
    if let Err(e) = res {
      log::error!("error setting up graphics resources! {:?}", e)
    }
  }
}

fn create_random_model(mesh_choices: &[Handle]) -> (RenderModel, Transform3D) {
  use nalgebra_glm::*;
  use rand::{prelude::*, thread_rng};
  let mut rng = thread_rng();

  let mesh = SliceRandom::choose(mesh_choices, &mut rng).cloned();
  let mut transform = Transform3D::default();
  let rand_dist = Uniform::new(-20.0, 20.0);
  transform.position = vec3(
    rng.sample(rand_dist),
    rng.sample(rand_dist),
    rng.sample(rand_dist),
  );
  let model = RenderModel {
    mesh,
    label: Some(format!("mesh_{:?}", mesh)),
  };
  (model, transform)
}

#[cfg(not(feature = "wgpu_renderer"))]
pub fn create_models() {
  // noop
}
