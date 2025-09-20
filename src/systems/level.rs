use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Size of each tile in pixels
pub const TILE_SIZE: f32 = 25.0;

pub fn spawn_level(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let level = vec![
        "##################################################",
        "#                       ##                       #",
        "#                       ##                       #",
        "#                       ##                       #",
        "#    #####    ######    ##    ######    #####    #",
        "#    #####    ######    ##    ######    #####    #",
        "#                                                #",
        "#                                                #",
        "#                                                #",
        "#                                                #",
        "#                                                #",
        "#         ####     ####    ####     ####         #",
        "#         ####     #          #     ####         #",
        "#         ####     #          #     ####         #",
        "#         ####     #          #     ####         #",
        "#         ####     ############     ####         #",
        "#                                                #",
        "#                                                #",
        "#                                                #",
        "#                                                #",
        "#         ##############################         #",
        "#         ##############################         #",
        "#                                                #",
        "#                                                #",
        "#                                                #",
        "#                                                #",
        "##################################################",
    ];

    let height = level.len();
    let width = level[0].chars().count();

    let origin_offset = Vec2::new(
        -(width as f32) * TILE_SIZE / 2.0 + TILE_SIZE / 2.0,
        (height as f32) * TILE_SIZE / 2.0 - TILE_SIZE / 2.0,
    );

    let wall_texture = asset_server.load("textures/wall.png");

    for (y, row) in level.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                let world_pos = Vec3::new(
                    origin_offset.x + x as f32 * TILE_SIZE,
                    origin_offset.y - y as f32 * TILE_SIZE,
                    0.0,
                );

                commands.spawn((
                    SpriteBundle {
                        texture: wall_texture.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_translation(world_pos),
                        ..default()
                    },
                    Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
                    Name::new("Wall"),
                ));
            }
        }
    }
}
