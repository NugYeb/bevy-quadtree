mod setup;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
    window::{WindowResolution, WindowTheme},
};
use quadtree::QuadTreePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "QuadTree".to_string(),
                        window_theme: Some(WindowTheme::Dark),
                        resolution: WindowResolution::new(1200.0, 820.0),
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 16.0,
                    font: default(),
                    font_smoothing: FontSmoothing::default(),
                },
                text_color: Color::srgb(0.0, 1.0, 0.0),
                enabled: true,
            },
        })
        .add_plugins(QuadTreePlugin)
        .add_systems(Startup, setup::setup)
        // .add_systems(Update, startup::update)
        .run();
}
