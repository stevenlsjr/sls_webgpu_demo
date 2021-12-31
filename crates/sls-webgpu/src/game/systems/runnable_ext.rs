use crate::ecs::{
  storage::ComponentTypeId,
  systems::{CommandBuffer, ResourceTypeId, Runnable, SystemId, UnsafeResources},
  world::{ArchetypeAccess, WorldId},
  World,
};
use legion::systems::ParallelRunnable;
use shrinkwraprs::Shrinkwrap;

///
/// A newtype wrapper for Boxed Parallel Runnables, which allow them
/// to properly implement Runnable and ParallelRunnable
#[derive(Shrinkwrap, Debug)]
#[shrinkwrap(mutable)]
pub struct DynParallelRunnable(pub Box<dyn ParallelRunnable>);
impl DynParallelRunnable {
  pub fn new(boxed: Box<dyn ParallelRunnable>) -> Self {
    Self(boxed)
  }
}

impl Runnable for DynParallelRunnable {
  fn name(&self) -> Option<&SystemId> {
    self.0.name()
  }

  fn reads(&self) -> (&[ResourceTypeId], &[ComponentTypeId]) {
    self.0.reads()
  }

  fn writes(&self) -> (&[ResourceTypeId], &[ComponentTypeId]) {
    self.0.writes()
  }

  fn prepare(&mut self, world: &World) {
    self.0.prepare(world)
  }

  fn accesses_archetypes(&self) -> &ArchetypeAccess {
    self.0.accesses_archetypes()
  }

  unsafe fn run_unsafe(&mut self, world: &World, resources: &UnsafeResources) {
    self.0.run_unsafe(world, resources)
  }

  fn command_buffer_mut(&mut self, world: WorldId) -> Option<&mut CommandBuffer> {
    self.0.command_buffer_mut(world)
  }
}
