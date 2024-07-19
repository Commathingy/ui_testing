use bevy::prelude::*;
use bevy_lunex::prelude::*;

use crate::components::SkillButton;


#[derive(Component)]
pub struct SkillTreeRoute;

pub fn build_skill_tree_route(
    route_added: Query<Entity, Added<SkillTreeRoute>>,
    mut commands: Commands,
) {
    for route_ent in route_added.iter(){

        let root = UiLink::<MainUi>::path("SkillTreeRoot");

        commands.entity(route_ent).insert((
            UiTreeBundle::<MainUi>::from(UiTree::new("SkillTreeRoute")),
        ))
        .with_children(|route|{
            route.spawn((
                root.clone(),
                UiLayout::window().pos(Rl((50.0, 100.0))).size(Rl((50.0, 100.0))).pack::<Base>()
            ));

            route.spawn((
                root.add("test_img"),
                UiLayout::window_full().pack::<Base>(),
                SkillButton{position: Ab((50.0, 100.0)).into(), text: "testButton".into()}
            ));

        });
    }
}