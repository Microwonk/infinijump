#![allow(clippy::type_complexity)]

mod audio;
mod character_controller;
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
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_xpbd_2d::math::{Scalar, Vector};
use bevy_xpbd_2d::prelude::*;
use character_controller::{
    CharacterController, CharacterControllerBundle, CharacterControllerPlugin,
};
use level_generator::perlin_generator::SimplePerlinLevelGenerator;
use level_generator::{LevelGeneratorPlugin, Seed};
use rand::{thread_rng, Rng};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // TODO: Add a pause state
}

pub struct InfiniJumpPlugin;

impl Plugin for InfiniJumpPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            PhysicsPlugins::default(),
            LevelGeneratorPlugin::<SimplePerlinLevelGenerator, CharacterController>::seeded(
                thread_rng().gen(),
            ),
            CharacterControllerPlugin,
            TempPlugin,
            LoadingPlugin,
            MenuPlugin,
            InternalAudioPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins(WorldInspectorPlugin::new())
                .register_type::<Seed>()
                .add_plugins((
                    FrameTimeDiagnosticsPlugin,
                    LogDiagnosticsPlugin::default(),
                    PhysicsDebugPlugin::default(),
                ))
                .add_systems(Update, close_on_esc);
        }
    }
}

// TODO: Temporary Plugin
pub struct TempPlugin;

impl Plugin for TempPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin)
            .add_systems(Startup, camera_setup)
            .add_systems(OnEnter(GameState::Playing), spawn_player);
    }
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
}

#[allow(dead_code)]
fn spawn_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 40.0)),
                color: Color::rgb(0.0, 0.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            100., 1000., 10.,
        )))
        .insert(
            CharacterControllerBundle::new(Collider::capsule(20.0, 12.5), Vector::NEG_Y * 1000.0)
                .with_movement(3050.0, 0.92, 400.0, (30.0 as Scalar).to_radians()),
        );
}
