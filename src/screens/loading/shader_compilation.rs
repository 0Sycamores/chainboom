//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;
use bevy_hanabi::ParticleEffect;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use crate::{
    font::FontAssets,
    gameplay::explosion::assets::ExplosionAssets,
    shader_compilation::{
        LoadedPipelineCount, PipelinesReady, all_pipelines_loaded, spawn_shader_compilation_map,
    },
    theme::{palette::SCREEN_BACKGROUND, prelude::*},
};

use super::LoadingScreen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(LoadingScreen::Shaders),
        (
            spawn_or_skip_shader_compilation_loading_screen,
            (spawn_shader_compilation_map, setup_particle_effects).run_if(
                |loaded_pipeline_count: Res<LoadedPipelineCount>| !loaded_pipeline_count.is_done(),
            ),
        ),
    );

    app.add_systems(
        Update,
        (
            update_loading_shaders_label,
            enter_spawn_level_screen.run_if(all_pipelines_loaded),
        )
            .chain()
            .run_if(in_state(LoadingScreen::Shaders)),
    );

    app.register_type::<LoadingShadersLabel>();
}

#[cfg_attr(feature = "hot_patch", hot)]
fn spawn_or_skip_shader_compilation_loading_screen(
    mut commands: Commands,
    loaded_pipeline_count: Res<LoadedPipelineCount>,
    pipelines_ready: Res<PipelinesReady>,
    mut next_screen: ResMut<NextState<LoadingScreen>>,
    fonts: Res<FontAssets>,
) {
    if loaded_pipeline_count.is_done() && pipelines_ready.0 {
        next_screen.set(LoadingScreen::Level);
        return;
    }
    commands.spawn((
        widget::ui_root("Loading Screen"),
        BackgroundColor(SCREEN_BACKGROUND),
        StateScoped(LoadingScreen::Shaders),
        children![
            (
                widget::label("Compiling shaders", fonts.default.clone()),
                LoadingShadersLabel
            ),
            widget::label("This can take up to a minute", fonts.default.clone()),
        ],
    ));
}

fn setup_particle_effects(mut commands: Commands, explosion_assets: Res<ExplosionAssets>) {
    // Spawn the particle effects for shader compilation.
    commands.spawn((
        StateScoped(LoadingScreen::Shaders),
        ParticleEffect::new(explosion_assets.prop_explosion_vfx.clone()),
    ));
    commands.spawn((
        StateScoped(LoadingScreen::Shaders),
        ParticleEffect::new(explosion_assets.enemy_explosion_vfx.clone()),
    ));
}

#[cfg_attr(feature = "hot_patch", hot)]
fn enter_spawn_level_screen(mut next_screen: ResMut<NextState<LoadingScreen>>) {
    next_screen.set(LoadingScreen::Level);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct LoadingShadersLabel;

#[cfg_attr(feature = "hot_patch", hot)]
fn update_loading_shaders_label(
    mut query: Query<&mut Text, With<LoadingShadersLabel>>,
    loaded_pipeline_count: Res<LoadedPipelineCount>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!(
            "Compiling shaders: {} / {}",
            loaded_pipeline_count.0,
            LoadedPipelineCount::TOTAL_PIPELINES
        );
    }
}
