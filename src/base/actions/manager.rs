use super::Actions;
use crate::{app::states::manager::StateManager, base::commands::Command};

#[derive(Clone)]
pub struct ActionsManager {
    // state_manager: &'a StateManager,
}
impl ActionsManager {
    pub fn get_command_of_action(&self, action: Actions, states: &StateManager) -> Option<Command> {
        Some(states.get_command_map().get(&action)?.clone())
    }
}
