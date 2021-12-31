#![allow(dead_code)]
use crate::{
  ecs::{
    systems::{CommandBuffer, ParallelRunnable},
    world::SubWorld,
    *,
  },
  scene_graph::components::*,
};

pub fn build() -> impl ParallelRunnable {
  SystemBuilder::<()>::new("LocalToWorldPropagateSystem")
    // Entities with a `Children` and `LocalToWorld` but NOT a `Parent` (ie those that are
    // roots of a hierarchy).
    .with_query(<(Read<Children>, Read<LocalToWorld>)>::query().filter(!component::<Parent>()))
    .read_component::<Children>()
    .read_component::<LocalToParent>()
    .build(move |commands, world, _resource, query| {
      for (children, local_to_world) in query.iter(world) {
        for child in children.0.iter() {
          propagate_recursive(*local_to_world, world, *child, commands);
        }
      }
    })
}

fn propagate_recursive(
  parent_local_to_world: LocalToWorld,
  world: &SubWorld,
  entity: Entity,
  commands: &mut CommandBuffer,
) {
  log::trace!("Updating LocalToWorld for {:?}", entity);
  let local_to_parent = {
    if let Some(local_to_parent) = world
      .entry_ref(entity)
      .ok()
      .and_then(|entry| entry.into_component::<LocalToParent>().ok())
    {
      *local_to_parent
    } else {
      log::warn!(
        "Entity {:?} is a child in the hierarchy but does not have a LocalToParent",
        entity
      );
      return;
    }
  };

  let new_local_to_world = LocalToWorld(parent_local_to_world.0 * local_to_parent.0);
  commands.add_component(entity, new_local_to_world);

  // Collect children
  let children = if let Some(entry) = world.entry_ref(entity).ok() {
    entry
      .get_component::<Children>()
      .map(|e| e.0.iter().cloned().collect::<Vec<_>>())
      .unwrap_or_default()
  } else {
    Vec::default()
  };

  for child in children {
    propagate_recursive(new_local_to_world, world, child, commands);
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{
    local_to_parent_system, local_to_world_propagate_system, local_to_world_system,
    missing_previous_parent_system, parent_update_system,
  };

  #[test]
  fn did_propagate() {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut resources = Resources::default();
    let mut world = World::default();

    let mut schedule = Schedule::builder()
      .add_system(missing_previous_parent_system::build())
      .flush()
      .add_system(parent_update_system::build())
      .flush()
      .add_system(local_to_parent_system::build())
      .flush()
      .add_system(local_to_world_system::build())
      .flush()
      .add_system(local_to_world_propagate_system::build())
      .build();

    // Root entity
    let parent = world.push((Translation::new(1.0, 0.0, 0.0), LocalToWorld::identity()));

    let children = world.extend(vec![
      (
        Translation::new(0.0, 2.0, 0.0),
        LocalToParent::identity(),
        LocalToWorld::identity(),
      ),
      (
        Translation::new(0.0, 0.0, 3.0),
        LocalToParent::identity(),
        LocalToWorld::identity(),
      ),
    ]);
    let (e1, e2) = (children[0], children[1]);

    // Parent `e1` and `e2` to `parent`.
    world.entry(e1).unwrap().add_component(Parent(parent));
    world.entry(e2).unwrap().add_component(Parent(parent));

    // Run systems
    schedule.execute(&mut world, &mut resources);

    assert_eq!(
      world
        .entry(e1)
        .unwrap()
        .get_component::<LocalToWorld>()
        .unwrap()
        .0,
      Translation::new(1.0, 0.0, 0.0).to_homogeneous()
        * Translation::new(0.0, 2.0, 0.0).to_homogeneous()
    );

    assert_eq!(
      world
        .entry(e2)
        .unwrap()
        .get_component::<LocalToWorld>()
        .unwrap()
        .0,
      Translation::new(1.0, 0.0, 0.0).to_homogeneous()
        * Translation::new(0.0, 0.0, 3.0).to_homogeneous()
    );
  }
}
