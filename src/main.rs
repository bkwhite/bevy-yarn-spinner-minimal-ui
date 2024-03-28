use bevy::prelude::*;
use bevy_yarnspinner::{
    events::{NodeCompleteEvent, PresentLineEvent},
    prelude::{DialogueRunner, YarnProject, YarnSpinnerPlugin},
};

#[derive(Component, Default)]
pub struct DialogLine;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(YarnSpinnerPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, present_line.run_if(on_event::<PresentLineEvent>()))
        .add_systems(Update, handle_click.run_if(resource_exists::<YarnProject>))
        .add_systems(
            Update,
            complete_dialogue.run_if(on_event::<NodeCompleteEvent>()),
        )
        .run();
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Waiting for dialog...",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 32.0,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
                DialogLine,
            ));

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::DARK_GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn handle_click(
    project: Res<YarnProject>,
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match interaction {
            Interaction::Pressed => {
                if text.sections[0].value == "Start".to_string() {
                    text.sections[0].value = "Next".to_string();

                    let mut dialogue_runner = project.create_dialogue_runner();
                    dialogue_runner.start_node("Hello");
                    commands.spawn(dialogue_runner);
                } else if text.sections[0].value == "Next".to_string() {
                    for mut dialogue_runner in dialogue_runners.iter_mut() {
                        if dialogue_runner.is_running() {
                            dialogue_runner.continue_in_next_update();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn present_line(
    mut reader: EventReader<PresentLineEvent>,
    mut line: Query<&mut Text, With<DialogLine>>,
) {
    for evt in reader.read() {
        let event_text = evt.line.text.clone();
        for mut text in line.iter_mut() {
            text.sections[0].value = event_text.clone();
        }
    }
}

fn complete_dialogue(
    button: Query<&Children, With<Button>>,
    mut line: Query<&mut Text, With<DialogLine>>,
    mut text: Query<&mut Text, Without<DialogLine>>,
) {
    for mut text in line.iter_mut() {
        text.sections[0].value = "Waiting for dialog...".to_string();
    }

    for children in button.iter() {
        let mut text = text.get_mut(children[0]).unwrap();
        text.sections[0].value = "Start".to_string();
    }
}
