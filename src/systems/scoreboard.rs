use bevy::color::palettes::css::YELLOW;
use bevy::prelude::*;
use crate::components::*;

pub fn spawn_scoreboard(
    mut commands: Commands,
) {
    commands.spawn((
        Text::new("0"),
            TextFont{
                font_size: 64.0,
                ..Default::default()
            },
            TextColor(YELLOW.into()),
            TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(45.0),
            ..default()
        },
        Player1Score
));

    commands.spawn((
        Text::new("0"),
        TextFont{
            font_size: 64.0,
            ..Default::default()
        },
        TextColor(YELLOW.into()),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(45.0),
            ..default()
        },
        Player2Score
    ));
}

pub fn update_scoreboard(
    mut player1_score: Query<&mut Text, With<Player1Score>>,
    mut player2_score: Query<&mut Text, (With<Player2Score>, Without<Player1Score>)>,
    score: Res<Score>,
){
    if score.is_changed(){
        if let Ok(mut player1_score) = player1_score.get_single_mut() {
            player1_score.0 = score.player1.to_string();
        }
        if let Ok(mut player2_score) = player2_score.get_single_mut() {
            player2_score.0 = score.player2.to_string();
        }
    }
}

pub fn update_score(
    mut score: ResMut<Score>,
    mut events: EventReader<Scored>,
){
    for event in events.read(){
        match event.0{
            Scorer::Player1 => score.player1 += 1,
            Scorer::Player2 => score.player2 += 1,
        }
    }
    println!("Score: {} - {}", score.player1, score.player2);
}