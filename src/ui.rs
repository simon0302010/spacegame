use bevy::{
    color::Color,
    ecs::{component::Component, system::ResMut},
    prelude::*,
};

use crate::collisions::Stats;

const WHITE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

#[derive(Component)]
pub struct StatsText;

#[derive(Component)]
pub struct GameOverText;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    InGame,
    GameOver,
}

pub fn spawn_stats(mut commands: Commands, stats: ResMut<Stats>) {
    commands.spawn((
        Text::new(format!("Score: {}, Health: {}", stats.score, stats.health)),
        TextFont {
            font_size: 20.0,
            ..Default::default()
        },
        TextColor(WHITE_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            top: Val::Px(15.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        StatsText,
    ));
}

pub fn update_stats(mut stats_query: Query<&mut Text, With<StatsText>>, stats: Res<Stats>) {
    if let Ok(mut stats_text) = stats_query.single_mut() {
        stats_text.0 = format!("Score: {}, Health: {}", stats.score, stats.health);
    }
}

pub fn spawn_game_over_ui(mut commands: Commands, stats: Res<Stats>) {
    commands.spawn((
        Text::new(format!(
            "You died.\nPress Space to restart\nScore: {}",
            stats.score
        )),
        TextFont {
            font_size: 50.0,
            ..Default::default()
        },
        TextColor(Color::srgb(1.5, 0.0, 0.0)),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            top: Val::Percent(30.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        GameOverText,
    ));
}

pub fn despawn_game_over_ui(
    mut commands: Commands,
    query: Query<Entity, With<GameOverText>>,
    mut stats: ResMut<Stats>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    stats.score = 0;
    stats.health = 3.0;
}

pub fn handle_game_over_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::InGame);
    }
}
