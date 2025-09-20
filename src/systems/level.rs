use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use image::GenericImageView;

pub fn spawn_level(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // -- Load background --
    let background_texture = asset_server.load("textures/map-beauty.png");
    commands.spawn(SpriteBundle {
        texture: background_texture,
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::Center,
            ..default()
        },
        ..default()
    });

    // -- Load mask --
    let beauty_image =
        image::open("assets/textures/map-beauty.png").expect("Failed to load map-beauty.png");
    let mask_image = image::open("assets/textures/map-mask_lo.png")
        .expect("Failed to load map-mask_lo.png")
        .to_luma8();

    let (beauty_w, beauty_h) = beauty_image.dimensions();
    let (mask_w, mask_h) = mask_image.dimensions();

    let tile_size_x = beauty_w as f32 / mask_w as f32;
    let tile_size_y = beauty_h as f32 / mask_h as f32;
    let tile_size = tile_size_x.min(tile_size_y);

    let origin_offset = Vec2::new(
        -(mask_w as f32) * tile_size / 2.0 + tile_size / 2.0,
        (mask_h as f32) * tile_size / 2.0 - tile_size / 2.0,
    );

    // Step 1: Build grid of solids
    let mut solid = vec![vec![false; mask_w as usize]; mask_h as usize];
    for y in 0..mask_h {
        for x in 0..mask_w {
            let pixel = mask_image.get_pixel(x, y)[0];
            solid[y as usize][x as usize] = pixel < 128;
        }
    }

    let mut merged_rects = Vec::new();
    let mut visited = vec![vec![false; mask_w as usize]; mask_h as usize];

    // Step 2: Greedy merge rectangles
    for y in 0..mask_h as usize {
        for x in 0..mask_w as usize {
            if !solid[y][x] || visited[y][x] {
                continue;
            }

            // Grow horizontally
            let mut w = 1;
            while x + w < mask_w as usize && solid[y][x + w] && !visited[y][x + w] {
                w += 1;
            }

            // Grow vertically
            let mut h = 1;
            'outer: while y + h < mask_h as usize {
                for dx in 0..w {
                    if !solid[y + h][x + dx] || visited[y + h][x + dx] {
                        break 'outer;
                    }
                }
                h += 1;
            }

            // Mark all as visited
            for dy in 0..h {
                for dx in 0..w {
                    visited[y + dy][x + dx] = true;
                }
            }

            merged_rects.push((x, y, w, h));
        }
    }

    // Step 3: Spawn merged colliders
    for (x, y, w, h) in &merged_rects {
        let center_x = *x as f32 + *w as f32 / 2.0 - 0.5;
        let center_y = *y as f32 + *h as f32 / 2.0 - 0.5;

        let world_pos = Vec3::new(
            origin_offset.x + center_x * tile_size,
            origin_offset.y - center_y * tile_size,
            0.5,
        );

        let half_width = *w as f32 * tile_size / 2.0;
        let half_height = *h as f32 * tile_size / 2.0;

        // Physics collider
        commands.spawn((
            Collider::cuboid(half_width, half_height),
            Transform::from_translation(world_pos),
            GlobalTransform::default(),
            Name::new("MergedWall"),
        ));

        // Optional debug sprite
        // commands.spawn(SpriteBundle {
        //     sprite: Sprite {
        //         color: Color::rgba(1.0, 0.0, 0.0, 0.8),
        //         custom_size: Some(Vec2::new(half_width * 2.0, half_height * 2.0)),
        //         ..default()
        //     },
        //     transform: Transform::from_translation(world_pos),
        //     ..default()
        // });
    }

    info!(
        "Merged from {} mask pixels to {} colliders (tile = {:.2})",
        solid
            .iter()
            .map(|row| row.iter().filter(|&&s| s).count())
            .sum::<usize>(),
        merged_rects.len(),
        tile_size
    );
}
