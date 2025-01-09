use bevy::{color::palettes::css::{BLUE, GOLD, GREEN, MEDIUM_PURPLE}, prelude::*};
use rand::Rng;
use crate::components::{
    item::{Item, ItemType, ItemRarity},
    weapon::{WeaponItem, WeaponType, WeaponProperties, MeleeProperties, RangedProperties},
    armor::{ArmorItem, ArmorType, ArmorSlot},
    consumable::{ConsumableItem, EffectType},
};

#[derive(Component)]
pub struct ItemWorldSpawn {
    pub hover_text_entity: Option<Entity>,
}

pub fn spawn_random_item(
    commands: &mut Commands,
    position: Vec3,
    rarity: Option<ItemRarity>,
) -> Entity {
    let mut rng = rand::thread_rng();
    
    // If no rarity specified, randomly select one
    let rarity = rarity.unwrap_or_else(|| {
        match rng.gen_range(0..100) {
            0..=60 => ItemRarity::Common,
            61..=85 => ItemRarity::Uncommon,
            86..=95 => ItemRarity::Rare,
            96..=98 => ItemRarity::Epic,
            _ => ItemRarity::Legendary,
        }
    });

    let item_type = match rng.gen_range(0..3) {
        0 => {
            let is_melee = rng.gen_bool(0.5);
            let damage = rng.gen_range(5.0..15.0) * get_rarity_multiplier(&rarity);
            
            ItemType::Weapon(WeaponItem {
                weapon_type: if is_melee { WeaponType::Melee } else { WeaponType::Ranged },
                damage,
                attack_speed: rng.gen_range(0.8..2.0),
                last_attack: 0.0,
                properties: if is_melee {
                    WeaponProperties::Melee(MeleeProperties {
                        swing_width: rng.gen_range(32.0..64.0),
                        swing_height: rng.gen_range(16.0..32.0),
                        swing_time: rng.gen_range(0.1..0.3),
                        swing_arc: rng.gen_range(std::f32::consts::PI * 0.5..std::f32::consts::PI),
                    })
                } else {
                    WeaponProperties::Ranged(RangedProperties {
                        projectile_speed: rng.gen_range(200.0..400.0),
                        projectile_size: rng.gen_range(4.0..12.0),
                        max_range: rng.gen_range(300.0..500.0),
                    })
                },
            })
        },
        1 => ItemType::Armor(ArmorItem {
            armor_type: match rng.gen_range(0..3) {
                0 => ArmorType::Light,
                1 => ArmorType::Medium,
                _ => ArmorType::Heavy,
            },
            defense: rng.gen_range(2.0..8.0) * get_rarity_multiplier(&rarity),
            slot: match rng.gen_range(0..4) {
                0 => ArmorSlot::Head,
                1 => ArmorSlot::Chest,
                2 => ArmorSlot::Legs,
                _ => ArmorSlot::Feet,
            },
            requirements: Default::default(),
        }),
        _ => ItemType::Consumable(ConsumableItem {
            effect_type: EffectType::Health,
            potency: rng.gen_range(20.0..50.0) * get_rarity_multiplier(&rarity),
            duration: None,
        }),
    };

    let (color, shape_size) = get_item_visual_properties(&item_type);

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(shape_size),
            ..default()
        },
        Transform::from_translation(position),
        Item {
            id: "random_item".to_string(),
            name: generate_item_name(&item_type, &rarity),
            description: generate_item_description(&item_type),
            item_type,
            rarity,
            value: rng.gen_range(10..100),
        },
        ItemWorldSpawn {
            hover_text_entity: None
        },
    )).id()
}

pub fn update_item_hover_text(
    mut commands: Commands,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut item_query: Query<(Entity, &Transform, &Item, &mut ItemWorldSpawn)>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        
        if let Ok(world_position) = camera.viewport_to_world_2d(
            camera_transform,
            cursor_position,
        ) {
            for (entity, transform, item, mut world_spawn) in item_query.iter_mut() {
                let distance = world_position.distance(transform.translation.truncate());
                
                if distance < 32.0 {
                    if world_spawn.hover_text_entity.is_none() {
                        let text_entity = commands.spawn((
                            Text2d::new(format!("{}\n{}", item.name, item.description)),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(get_rarity_color(&item.rarity)),
                            Transform::from_translation(transform.translation + Vec3::new(0.0, 32.0, 0.0)),
                        )).id();
                        world_spawn.hover_text_entity = Some(text_entity);
                    }
                } else if let Some(text_entity) = world_spawn.hover_text_entity.take() {
                    commands.entity(text_entity).despawn();
                }
            }
        }
    }
}

fn get_rarity_multiplier(rarity: &ItemRarity) -> f32 {
    match rarity {
        ItemRarity::Common => 1.0,
        ItemRarity::Uncommon => 1.5,
        ItemRarity::Rare => 2.0,
        ItemRarity::Epic => 3.0,
        ItemRarity::Legendary => 5.0,
    }
}

fn get_rarity_color(rarity: &ItemRarity) -> Color {
    match rarity {
        ItemRarity::Common => Color::WHITE,
        ItemRarity::Uncommon => Color::from(GREEN),
        ItemRarity::Rare => Color::from(BLUE),
        ItemRarity::Epic => Color::from(MEDIUM_PURPLE),
        ItemRarity::Legendary => Color::from(GOLD),
    }
}

fn get_item_visual_properties(item_type: &ItemType) -> (Color, Vec2) {
    match item_type {
        ItemType::Weapon(_) => (Color::srgb(0.7, 0.7, 0.7), Vec2::new(16.0, 24.0)),
        ItemType::Armor(_) => (Color::srgb(0.4, 0.4, 0.4), Vec2::new(20.0, 20.0)),
        ItemType::Consumable(_) => (Color::srgb(0.8, 0.2, 0.2), Vec2::new(12.0, 12.0)),
    }
}

fn generate_item_name(item_type: &ItemType, rarity: &ItemRarity) -> String {
    match item_type {
        ItemType::Weapon(w) => format!("{:?} {:?} Weapon", rarity, w.weapon_type),
        ItemType::Armor(a) => format!("{:?} {:?} {:?}", rarity, a.armor_type, a.slot),
        ItemType::Consumable(_) => format!("{:?} Health Potion", rarity),
    }
}

fn generate_item_description(item_type: &ItemType) -> String {
    match item_type {
        ItemType::Weapon(w) => {
            let base_desc = format!("Damage: {:.1}, Speed: {:.1}", w.damage, w.attack_speed);
            match &w.properties {
                WeaponProperties::Melee(m) => format!("{} (Melee, Width: {:.1})", base_desc, m.swing_width),
                WeaponProperties::Ranged(r) => format!("{} (Ranged, Range: {:.1})", base_desc, r.max_range),
            }
        },
        ItemType::Armor(a) => format!("Defense: {:.1}", a.defense),
        ItemType::Consumable(c) => format!("Heals for {:.1} health", c.potency),
    }
}