use anyhow::anyhow;
use legion::*;
use rand::distributions::Uniform;

#[cfg(feature = "wgpu_renderer")]
pub use wgpu_renderer::*;

use crate::game::{
  asset_loading::resources::MainSceneAssets,
  components::{RenderModel, Transform3D},
};

#[cfg(feature = "wgpu_renderer")]
mod wgpu_renderer {
  use std::{
    borrow::BorrowMut,
    sync::{Arc, RwLock},
  };

  use legion::systems::CommandBuffer;

  use crate::{game::resources::MeshLookup, Context};

  use super::*;
  use crate::game::asset_loading::resources::MainSceneAssets;

  fn load_mesh_lookup<C: BorrowMut<Context>>(
    _context: C,
    _lookup: &mut MeshLookup,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  #[system]
  pub fn create_models_wgpu(
    #[resource] context: &Arc<RwLock<Context>>,
    #[resource] assets: &MainSceneAssets,
    #[resource] _mesh_lookup: &mut MeshLookup,
    cmd: &mut CommandBuffer,
  ) {
    let res = context
      .write()
      .map_err(|e| anyhow!("Could not access context RwLock: Poisoned {:?}", e))
      .and_then(|_context| {
        let n_models = 10;
        for _i in 0..n_models {
          let components = create_random_model(assets);
          cmd.push(components);
        }
        Ok(())
      });
    if let Err(e) = res {
      log::error!("error setting up graphics resources! {:?}", e)
    }
  }
}

fn create_random_model(assets: &MainSceneAssets) -> (RenderModel, Transform3D) {
  use nalgebra_glm::*;
  use rand::{prelude::*, thread_rng};
  let mut rng = thread_rng();

  let mesh = assets.avocado_model;
  let mut transform = Transform3D::default();
  let rand_dist = Uniform::new(-10.0, 10.0);
  transform.position = vec3(rng.sample(rand_dist), 0.0, rng.sample(rand_dist));
  let model = RenderModel {
    model: Some(mesh),
    is_shown: true,
  };
  (model, transform)
}

#[cfg(not(feature = "wgpu_renderer"))]
#[system]
pub fn create_models() {
  // noop
}
