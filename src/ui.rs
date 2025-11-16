use bevy::{
    color::Color,
    ecs::{component::Component, system::ResMut},
    prelude::*,
};

use crate::collisions::Stats;

const WHITE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

#[derive(Component)]
pub struct StatsText;

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
