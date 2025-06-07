//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

mod animation;
pub(crate) mod crosshair;
pub(crate) mod explosion;
pub(crate) mod gore_settings;
pub(crate) mod health;
pub(crate) mod hud;
pub(crate) mod level;
pub(crate) mod npc;
pub(crate) mod player;
pub(crate) mod upgrades;
pub(crate) mod waves;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        crosshair::plugin,
        explosion::plugin,
        gore_settings::plugin,
        npc::plugin,
        player::plugin,
        health::plugin,
        hud::plugin,
        waves::plugin,
        upgrades::plugin,
        // This plugin preloads the level,
        // so make sure to add it last.
        level::plugin,
    ));
}
