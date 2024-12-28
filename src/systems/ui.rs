use crate::{
    components::{class::{ClassType, SelectedClass}, ui::MenuData}, resources::GameState, systems::class::spawn_player_with_class,
};
use bevy::prelude::*;

pub fn class_selection_ui(mut commands: Commands, mut menu_data: ResMut<MenuData>) {
    // Root node
    let root = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Choose Your Class"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent
                .spawn(Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Warrior Button
                    spawn_class_button(parent, "Warrior", ClassType::Warrior);
                    // Archer Button
                    spawn_class_button(parent, "Archer", ClassType::Archer);
                    // Mage Button
                    spawn_class_button(parent, "Mage", ClassType::Mage);
                });
        })
        .id();

    menu_data.root_entity = Some(root);

    // Button container
}

fn spawn_class_button(parent: &mut ChildBuilder, text: &str, class_type: ClassType) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor::from(Color::srgb(0.2, 0.2, 0.2)),
            class_type,
        ))
        .with_child((
            Text::new(text),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
}

pub fn handle_class_selection(
    mut selected_class: ResMut<SelectedClass>,
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<(&Interaction, &ClassType), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, class_type) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            info!("Selected {:?} class", class_type);
            selected_class.class_type = Some(*class_type);
            next_state.set(GameState::Playing);
            break;
        }
    }
}

pub fn cleanup_menu(
    mut commands: Commands,
    menu_data: Res<MenuData>,
) {
    if let Some(root_entity) = menu_data.root_entity {
        commands.entity(root_entity).despawn_recursive();
    }
}
