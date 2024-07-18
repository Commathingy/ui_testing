use bevy::prelude::*;
use bevy_lunex::prelude::*;


#[derive(Component)]
pub struct SkillButton {
    // Any fields we want to interact with should be here.
    pub position: UiValue<Vec2>,
    pub text: String,
}

#[derive(Component, Clone)]
pub struct SkillButtonUi;

pub fn build_skill_button(
    button_added: Query<(Entity, &SkillButton), Added<SkillButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    for (button_ent, button) in button_added.iter(){
        let root = UiLink::<SkillButtonUi>::path("SkillButtonRoot");
        commands.entity(button_ent).insert(
            UiTreeBundle::<SkillButtonUi>::from(UiTree::new("SkillButtonRoute"))
        ).with_children(|ui|{
            ui.spawn((
                root.clone(),
                UiLayout::window_full().pack::<Base>()
            ));
            ui.spawn((
                root.add("Image"),
                UiLayout::window().pos(button.position).size(Ab((128.0, 36.0))).pack::<Base>(),
                UiImage2dBundle::from(asset_server.load("background.png"))
            ));

            let current_y = button.position.get_y();
            ui.spawn((
                root.add("Text"),
                UiLayout::window().pos(button.position.with_y(current_y - Ab(15.0))).pack::<Base>(),
                UiText2dBundle {
                    text: Text::from_section(&button.text,
                        TextStyle {
                            font_size: 60.0, // By default hardcoded as Relative height (Rh) - so 60% of the node height
                            color: Color::BLACK,
                            ..Default::default()
                        }),
                    ..default()
                },
            ));
        });
    }
    
}