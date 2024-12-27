use bevy::prelude::*;
use crate::{
    components::class::ClassType,
    resources::GameState,
    systems::class::spawn_player_with_class,
};

pub fn class_selection_ui(mut commands: Commands) {
    // Root node
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }).with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Choose Your Class"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE)
            ));

            
        });

            // Button container
        //     parent
        //         .spawn(NodeBundle {
        //             style: Style {
        //                 margin: UiRect::all(Val::Px(20.0)),
        //                 flex_direction: FlexDirection::Column,
        //                 align_items: AlignItems::Center,
        //                 gap: Val::Px(10.0),
        //                 ..default()
        //             },
        //             ..default()
        //         })
        //         .with_children(|parent| {
        //             // Warrior Button
        //             spawn_class_button(parent, "Warrior", ClassType::Warrior);
        //             // Archer Button
        //             spawn_class_button(parent, "Archer", ClassType::Archer);
        //             // Mage Button
        //             spawn_class_button(parent, "Mage", ClassType::Mage);
        //         });
        // });
    
}

// fn spawn_class_button(parent: &mut ChildBuilder, text: &str, class_type: ClassType) {
//     parent
//         .spawn((
//             ButtonBundle {
//                 style: Style {
//                     width: Val::Px(200.0),
//                     height: Val::Px(50.0),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     ..default()
//                 },
//                 background_color: Color::rgb(0.2, 0.2, 0.2).into(),
//                 ..default()
//             },
//             class_type,
//         ))
//         .with_children(|parent| {
//             parent.spawn(TextBundle::from_section(
//                 text,
//                 TextStyle {
//                     font_size: 20.0,
//                     color: Color::WHITE,
//                     ..default()
//                 },
//             ));
//         });
// }

pub fn handle_class_selection(
    commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<
        (&Interaction, &ClassType),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, class_type) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            info!("Selected {:?} class", class_type);
            spawn_player_with_class(commands, *class_type);
            next_state.set(GameState::Playing);
            break;
        }
    }
}