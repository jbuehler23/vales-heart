use crate::{
    components::{
        class::{ClassType, SelectedClass}, combat::Enemy, inventory::{Inventory, Item, ItemRarity}, player::Player, ui::{ButtonType, InventoryUI, ItemSlot, MenuButton, MenuData}, weapon::Weapon
    },
    resources::GameState,
};
use bevy::{prelude::*, text::cosmic_text::ttf_parser::RgbaColor};

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

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    if let Some(root_entity) = menu_data.root_entity {
        commands.entity(root_entity).despawn_recursive();
    }
}

pub fn spawn_inventory_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        (Node {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            width: Val::Px(400.0),
            height: Val::Px(300.0),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
            padding: UiRect::all(Val::Px(10.0)),
            
            ..default()
        },
        BackgroundColor::from(Color::srgba(0.2, 0.2, 0.2, 0.8)),
        Visibility::Hidden,
        InventoryUI,
    )).with_children(|parent| {
        for i in 0..20 {
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(70.0),
                    height: Val::Px(70.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor::from(Color::srgba(0.1, 0.1, 0.1, 0.9)),
                ItemSlot { index: i },
            ));
        }
    });
}

pub fn pause_menu_ui(mut commands: Commands, mut menu_data: ResMut<MenuData>) {
    info!("Showing pause menu");
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
                Text::new("PAUSED"),
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
                    // Resume Button
                    spawn_menu_button(parent, ButtonType::Resume);
                    // Restart Button
                    spawn_menu_button(parent, ButtonType::Restart);
                });
        })
        .id();

    menu_data.root_entity = Some(root);
}

fn spawn_menu_button(parent: &mut ChildBuilder, button_type: ButtonType) {
    parent
        .spawn((
            Button,
            MenuButton { button_type },
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor::from(Color::srgb(0.2, 0.2, 0.2)),
        ))
        .with_child((
            Text::new(button_type.to_string()),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
}

pub fn handle_pause_menu(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    weapon_query: Query<Entity, With<Weapon>>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match button.button_type {
                ButtonType::Resume => {
                    next_state.set(GameState::Playing);
                }
                ButtonType::Restart => {
                    // Clean up game entities
                    for entity in player_query
                        .iter()
                        .chain(enemy_query.iter())
                        .chain(weapon_query.iter())
                    {
                        commands.entity(entity).despawn_recursive();
                    }
                    next_state.set(GameState::ClassSelection);
                }
            }
        }
    }
}

pub fn update_inventory_slots(
    mut commands: Commands,
    inventory_query: Query<(&Inventory, &Children)>,
    mut slot_query: Query<(&ItemSlot, &mut BackgroundColor)>,
    item_query: Query<&Item>,
) {
    for (inventory, children) in inventory_query.iter() {
        for (slot, mut bg_color) in slot_query.iter_mut() {
            if let Some(Some(item_stack)) = inventory.slots.get(&slot.index) {
                // Update slot appearance based on item rarity
                bg_color.0 = match item_stack.item.rarity {
                    ItemRarity::Common => Color::srgb(0.2, 0.2, 0.2),
                    ItemRarity::Uncommon => Color::srgb(0.2, 0.4, 0.2),
                    ItemRarity::Rare => Color::srgb(0.2, 0.2, 0.4),
                    ItemRarity::Epic => Color::srgb(0.4, 0.2, 0.4),
                    ItemRarity::Legendary => Color::srgb(0.4, 0.4, 0.2),
                };
            } else {
                bg_color.0 = Color::srgba(0.1, 0.1, 0.1, 0.9);
            }
        }
    }
}
