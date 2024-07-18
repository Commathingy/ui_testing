use bevy::{app::{Plugin, Update}, prelude::{IntoSystemConfigs, SystemSet}};
use bevy_lunex::{UiDebugPlugin, UiGenericPlugin, UiSystems};
use skill_button::{build_skill_button, SkillButtonUi};
pub use skill_button::SkillButton;

mod skill_button;

#[derive(SystemSet, Hash, Debug, Clone, PartialEq, Eq)]
pub struct ComponentBuildSet;

pub struct ComponentPlugin;

impl Plugin for ComponentPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(UiGenericPlugin::<SkillButtonUi>::new())
            .add_plugins(UiDebugPlugin::<SkillButtonUi>::new())
            .add_systems(Update, build_skill_button.before(UiSystems::Compute).in_set(ComponentBuildSet));
    }
}