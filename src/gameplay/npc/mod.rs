//! NPC handling. In the demo, the NPC is a fox that moves towards the player. We can interact with the NPC to trigger dialogue.

use ai_state::AiState;
use animation::{NpcAnimationState, setup_npc_animations};
use avian3d::prelude::*;
use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use bevy_tnua::{TnuaAnimatingState, prelude::*};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use bevy_trenchbroom::prelude::*;

use crate::{
    gameplay::npc::stats::NpcStats,
    third_party::{avian3d::CollisionLayer, bevy_trenchbroom::LoadTrenchbroomModel as _},
};

use super::{animation::AnimationPlayerAncestor, health::Health};
mod ai_state;
mod animation;
mod assets;
mod attack;
mod lifecycle;
pub(crate) mod navigation;
mod sound;
pub(crate) mod stats;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        navigation::plugin,
        animation::plugin,
        assets::plugin,
        sound::plugin,
        ai_state::plugin,
        attack::plugin,
        lifecycle::plugin,
        stats::plugin,
    ));
    app.register_type::<Npc>();
    app.add_observer(on_add);
}

#[derive(PointClass, Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/zombie_3/zombie_3.gltf")]
#[require(NpcStats)]
// In Wasm, TrenchBroom classes are not automatically registered.
// So, we need to manually register the class in `src/third_party/bevy_trenchbroom/mod.rs`.
pub(crate) struct Npc;

pub(crate) const NPC_RADIUS: f32 = 0.4;
const NPC_CAPSULE_LENGTH: f32 = 0.6;
pub(crate) const NPC_HEIGHT: f32 = NPC_CAPSULE_LENGTH + 2.0 * NPC_RADIUS;
const NPC_HALF_HEIGHT: f32 = NPC_HEIGHT / 2.0;
const NPC_FLOAT_HEIGHT: f32 = NPC_HALF_HEIGHT + 0.5;

#[cfg_attr(feature = "hot_patch", hot)]
fn on_add(trigger: Trigger<OnAdd, Npc>, mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .entity(trigger.target())
        .insert((
            Npc,
            Collider::capsule(NPC_RADIUS, NPC_CAPSULE_LENGTH),
            TnuaController::default(),
            TnuaAvian3dSensorShape(Collider::cylinder(NPC_RADIUS - 0.01, 0.0)),
            ColliderDensity(2_000.0),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
            TnuaAnimatingState::<NpcAnimationState>::default(),
            AnimationPlayerAncestor,
            CollisionLayers::new(
                [CollisionLayer::Character, CollisionLayer::Npc],
                LayerMask::ALL,
            ),
            Health::new(100.0),
            AiState::default(),
        ))
        .with_child((
            Name::new("Npc Model"),
            SceneRoot(assets.load_trenchbroom_model::<Npc>()),
            Transform::from_xyz(0.0, -NPC_FLOAT_HEIGHT, 0.0),
        ))
        .observe(setup_npc_animations);
}
