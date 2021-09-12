// systems for pushing componenet data to the renderer state.

use legion::{systems::CommandBuffer, *};

fn set_or_add_component<T>(
  _entity: &Entity,
  _cmd: &mut CommandBuffer,
  _current_val: &mut Option<T>,
  _new_val: Option<T>,
) {
}
