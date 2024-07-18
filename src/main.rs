use bevy::prelude::*;
use bevy_lunex::prelude::*;
use components::ComponentPlugin;
use routes::{RoutePlugin, SkillTreeRoute};


mod routes;
mod components;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin)
        .add_plugins(ComponentPlugin)
        .add_plugins(RoutePlugin)

        .add_plugins(UiDebugPlugin::<MainUi>::new())

        .add_systems(Startup, spawn_camera)
        .add_systems(Update, test_spawn_window)
        .run();
}


fn test_spawn_window(
    press: Res<ButtonInput<MouseButton>>,
    ent: Query<Entity, With<SkillTreeRoute>>,
    mut commands: Commands
) {
    if press.just_pressed(MouseButton::Left){
        commands.spawn(SkillTreeRoute);
    }
    if press.just_pressed(MouseButton::Right){
        commands.entity(ent.single()).despawn_recursive();
    }
}

fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn((
        MainUi,
        Camera2dBundle::default()
    )).with_children(|cam|{
        cam.spawn((
            // This is the main component
            Cursor2d::new(),
            // This is required so that the sprite doesn't block our picking raycaster
            Pickable::IGNORE,
            // This is required so that the sprite doesn't block our picking raycaster
            PointerBundle::new(PointerId::Custom(pointer::Uuid::new_v4())),
        ));
    });

}

/*

    commands.spawn(
        UiTreeBundle::<MainUi>::from(UiTree::new("Hello UI!"))
    ).with_children(|ui|{
        ui.spawn((

            // Link the entity
            UiLink::<MainUi>::path("Root"),
        
            // Specify UI layout
            UiLayout::window_full().pos(Ab(20.0)).size(Rl(100.0) - Ab(40.0)).pack::<Base>(),
        ));
        
        ui.spawn((
        
            // Link the entity
            UiLink::<MainUi>::path("Root/Rectangle"),
        
            // Specify UI layout
            UiLayout::solid().size(Ab((1920.0, 1080.0))).pack::<Base>(),
        
            // Add image to the entity
            UiImage2dBundle::from(asset_server.load("background.png")),
        ));
    });
     */
