//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

use bevy::prelude::*;
use bevy_yarnspinner::events::PresentLineEvent;

#[derive(Resource, Default)]
pub struct SpinnerCurrentLine(pub String);

#[derive(Component, Default)]
pub struct DialogLine;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SpinnerCurrentLine>()
        .add_systems(Startup, setup)
        .add_systems(Update, present_line.run_if(on_event::<PresentLineEvent>()))
        .run();
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
   commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "waiting for dialog...",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 100.0,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
                DialogLine,
            ));
        });
}

fn present_line(mut reader: EventReader<PresentLineEvent>) {
    for evt in reader.read() {}
}
