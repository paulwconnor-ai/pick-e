use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Size of each tile in pixels
pub const TILE_SIZE: f32 = 28.0;

pub fn spawn_level(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let level = vec![
        "1=======================================2",
        "|                                       |",
        "|                                       |",
        "|                                       |",
        "|                                       |",
        "|     =============================     |",
        "|                                       |",
        "|                                       |",
        "|                                       |",
        "|                                       |",
        "|     |   |    1==     ==2    1===2     |",
        "|     |   |    |         |    |   |     |",
        "|     |   |    |         |    |   |     |",
        "|     3===4    3=========4    |   |     |",
        "|                                       |",
        "|                                       |",
        "|                                       |",
        "|                                       |",
        "|     =============================     |",
        "|                                       |",
        "|                                       |",
        "|                                       |",
        "|                                       |",
        "3=======================================4",
    ];

    let height = level.len();
    let width = level[0].chars().count();

    let origin_offset = Vec2::new(
        -(width as f32) * TILE_SIZE / 2.0 + TILE_SIZE / 2.0,
        (height as f32) * TILE_SIZE / 2.0 - TILE_SIZE / 2.0,
    );

    // Load textures once
    let texture_wall_vert = asset_server.load("textures/wall_vert.png");
    let texture_wall_horiz = asset_server.load("textures/wall_horiz.png");
    let texture_corner_tl = asset_server.load("textures/wall_corner_tl.png");
    let texture_corner_tr = asset_server.load("textures/wall_corner_tr.png");
    let texture_corner_bl = asset_server.load("textures/wall_corner_bl.png");
    let texture_corner_br = asset_server.load("textures/wall_corner_br.png");

    for (y, row) in level.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let texture = match ch {
                '=' => Some(texture_wall_horiz.clone()),
                '|' => Some(texture_wall_vert.clone()),
                '1' => Some(texture_corner_tl.clone()),
                '2' => Some(texture_corner_tr.clone()),
                '3' => Some(texture_corner_bl.clone()),
                '4' => Some(texture_corner_br.clone()),
                _ => None,
            };

            if let Some(texture) = texture {
                let world_pos = Vec3::new(
                    origin_offset.x + x as f32 * TILE_SIZE,
                    origin_offset.y - y as f32 * TILE_SIZE,
                    0.0,
                );
                spawn_wall(commands, texture, world_pos);
            }
        }
    }
}

fn spawn_wall(commands: &mut Commands, texture: Handle<Image>, position: Vec3) {
    commands.spawn((
        SpriteBundle {
            texture,
            sprite: Sprite {
                color: Color::WHITE,
                anchor: bevy::sprite::Anchor::Center,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
        Name::new("Wall"),
    ));
}
