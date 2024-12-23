use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin {
                // Customize the debug render settings
                mode: DebugRenderMode::COLLIDER_SHAPES | DebugRenderMode::RIGID_BODY_AXES,
                style: DebugRenderStyle {
                    rigid_body_axes_length: 20.0,
                    ..default()
                },
                ..default()
            })
            .add_systems(
                Update,
                (
                    toggle_debug,
                    (|mut debug_render_context: ResMut<DebugRenderContext>| {
                        debug_render_context.enabled = !debug_render_context.enabled;
                    })
                    .run_if(input_just_pressed(KeyCode::KeyV)),
                ),
            );
    }
}


#[derive(Component)]
pub struct DebugCooldown(pub Timer);

pub fn toggle_debug(time: Res<Time>, mut query: Query<(&mut ColliderDebug, &mut DebugCooldown)>) {
    for (mut debug, mut cooldown) in query.iter_mut() {
        cooldown.0.tick(time.delta());
        if cooldown.0.just_finished() {
            *debug = match *debug {
                ColliderDebug::AlwaysRender => ColliderDebug::NeverRender,
                ColliderDebug::NeverRender => ColliderDebug::AlwaysRender,
            }
        }
    }
}