// use bevy::prelude::*;
// use bevy::text::FontStyle;
// use crate::components::class::*;
// use crate::GameState;

// pub fn class_selection_setup(mut commands: Commands) {
//     // Setup UI for class selection
//     commands
//         .spawn(NodeBundle {
//             style: Style {
//                 width: Val::Percent(100.0),
//                 height: Val::Percent(100.0),
//                 flex_direction: FlexDirection::Column,
//                 align_items: AlignItems::Center,
//                 justify_content: JustifyContent::Center,
//                 ..default()
//             },
//             ..default()
//         })
//         .with_children(|parent| {
//             // Add class selection buttons
//             for class_type in [ClassType::Warrior, ClassType::Archer, ClassType::Mage] {
//                 parent.spawn((
//                     ButtonBundle {
//                         style: Style {
//                             width: Val::Px(150.0),
//                             height: Val::Px(50.0),
//                             margin: UiRect::all(Val::Px(10.0)),
//                             justify_content: JustifyContent::Center,
//                             align_items: AlignItems::Center,
//                             ..default()
//                         },
//                         background_color: Color::rgb(0.15, 0.15, 0.15).into(),
//                         ..default()
//                     },
//                     class_type,
//                 ));
//             }
//         });
// }

// pub fn handle_class_selection(
//     mut commands: Commands,
//     interaction_query: Query<(&Interaction, &ClassType), Changed<Interaction>>,
//     mut next_state: ResMut<NextState<GameState>>,
// ) {
//     for (interaction, class_type) in interaction_query.iter() {
//         if *interaction == Interaction::Pressed {
//             let player_class = match class_type {
//                 ClassType::Warrior => PlayerClass::warrior(),
//                 ClassType::Archer => PlayerClass::archer(),
//                 ClassType::Mage => PlayerClass::mage(),
//             };
            
//             // Spawn player with selected class
//             commands.spawn((
//                 player_class,
//                 // Add other player components...
//             ));

//             // Transition to gameplay state
//             next_state.set(GameState::Playing);
//         }
//     }
// }