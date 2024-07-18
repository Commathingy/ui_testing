use bevy::prelude::*;

mod skill_tree_route;
use skill_tree_route::build_skill_tree_route;
pub use skill_tree_route::SkillTreeRoute;

use crate::components::ComponentBuildSet;

pub struct RoutePlugin;

impl Plugin for RoutePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, build_skill_tree_route.before(ComponentBuildSet));
    }
}