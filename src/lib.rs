#![allow(clippy::type_complexity)]

mod audio;
mod config;
mod level_generator;
mod loading;
mod menu;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::{app::App, window::close_on_esc};
use bevy_pancam::{PanCam, PanCamPlugin};
use level_generator::perlin::PerlinLevelGenerator;
use level_generator::LevelGeneratorPlugin;
use rand::{thread_rng, Rng};

// This example game uses States to separate logic
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct InfiniJumpPlugin;

impl Plugin for InfiniJumpPlugin {
    fn build(&self, app: &mut App) {
        // need to do this outside of the add_plugins call, because otherwise we would borrow more than once
        let mut rng = thread_rng();
        let seed = rng.gen();
        let lgp = LevelGeneratorPlugin::<PerlinLevelGenerator>::seeded(seed);

        app.add_state::<GameState>().add_plugins((
            lgp,
            CameraPlugin,
            LoadingPlugin,
            MenuPlugin,
            InternalAudioPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
            app.add_systems(Update, close_on_esc);
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin)
            .add_systems(Startup, camera_setup);
    }
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
}
