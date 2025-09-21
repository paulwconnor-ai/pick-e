use bevy::prelude::*;
use bevy::render::texture::Image;
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
pub struct LevelAssets {
    pub background: Handle<Image>,
    pub mask: Handle<Image>,
    pub spawned: bool,
}

pub fn setup_level_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background = asset_server.load("textures/map-beauty.png");
    let mask = asset_server.load("textures/map-mask_lo.png");

    commands.insert_resource(LevelAssets {
        background,
        mask,
        spawned: false,
    });

    info!("Level textures requested...");
}

pub fn spawn_level(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    mut level_assets: ResMut<LevelAssets>,
) {
    if level_assets.spawned {
        return;
    }

    let Some(mask_texture) = images.get(&level_assets.mask) else {
        debug!("Waiting for mask texture to load...");
        return;
    };
    let Some(beauty_texture) = images.get(&level_assets.background) else {
        debug!("Waiting for beauty texture to load...");
        return;
    };

    level_assets.spawned = true;
    info!("Level textures loaded! Attempting to spawn...");

    commands.spawn((
        SpriteBundle {
            texture: level_assets.background.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                scale: Vec3::ONE,
                ..default()
            },
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
            ..default()
        },
        Name::new("LevelBackground"),
    ));

    let cache_path = "assets/collision-cache.txt";
    if let Some(text) = try_load_collision_cache(cache_path) {
        if try_spawn_from_cache(&mut commands, &text, beauty_texture, mask_texture).is_some() {
            info!("Spawned level from collision cache");
            return;
        } else {
            warn!("Cache file found but invalid — falling back to mask");
        }
    } else {
        info!("No collision cache found — will generate from mask");
    }

    let merged_rects = generate_colliders_from_mask(mask_texture, beauty_texture, &mut commands);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let lines: Vec<String> = merged_rects
            .iter()
            .map(|(x, y, w, h)| format!("{x},{y},{w},{h}"))
            .collect();
        let result = std::fs::write(cache_path, lines.join("\n"));
        if let Err(e) = result {
            warn!("Failed to write collision cache: {}", e);
        }
    }

    info!(
        "Spawned {} colliders from mask (tile = {:.2})",
        merged_rects.len(),
        compute_tile_size(mask_texture, beauty_texture)
    );
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn try_load_collision_cache(path: &str) -> Option<String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::fs::read_to_string(path).ok()
    }
    #[cfg(target_arch = "wasm32")]
    {
        None
    }
}

fn try_spawn_from_cache(
    commands: &mut Commands,
    text: &str,
    beauty: &Image,
    mask: &Image,
) -> Option<()> {
    let tile_size = compute_tile_size(mask, beauty);
    let origin_offset = compute_origin_offset(mask, tile_size);

    let mut num_colliders = 0;

    for line in text.lines() {
        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() != 4 {
            warn!("Bad line in cache: {line}");
            return None;
        }
        let x = parts[0].parse::<usize>().ok()?;
        let y = parts[1].parse::<usize>().ok()?;
        let w = parts[2].parse::<usize>().ok()?;
        let h = parts[3].parse::<usize>().ok()?;

        spawn_collider(commands, x, y, w, h, tile_size, origin_offset);
        num_colliders += 1;
    }

    info!("Spawned {} colliders from cache", num_colliders);

    Some(())
}

fn compute_tile_size(mask: &Image, beauty: &Image) -> f32 {
    let mask_w = mask.size().x as f32;
    let mask_h = mask.size().y as f32;
    let beauty_w = beauty.size().x;
    let beauty_h = beauty.size().y;
    let tile_size_x = beauty_w as f32 / mask_w;
    let tile_size_y = beauty_h as f32 / mask_h;
    tile_size_x.min(tile_size_y)
}

fn compute_origin_offset(mask: &Image, tile_size: f32) -> Vec2 {
    let mask_w = mask.size().x as f32;
    let mask_h = mask.size().y as f32;
    Vec2::new(
        -mask_w * tile_size / 2.0 + tile_size / 2.0,
        mask_h * tile_size / 2.0 - tile_size / 2.0,
    )
}

fn generate_colliders_from_mask(
    mask: &Image,
    beauty: &Image,
    commands: &mut Commands,
) -> Vec<(usize, usize, usize, usize)> {
    let w = mask.size().x as usize;
    let h = mask.size().y as usize;
    let data = &mask.data;
    let pixel_stride = 4;

    let mut solid = vec![vec![false; w]; h];
    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) * pixel_stride;
            solid[y][x] = data[idx] < 128;
        }
    }

    let mut visited = vec![vec![false; w]; h];
    let mut merged = Vec::new();

    for y in 0..h {
        for x in 0..w {
            if !solid[y][x] || visited[y][x] {
                continue;
            }

            let mut rect_w = 1;
            while x + rect_w < w && solid[y][x + rect_w] && !visited[y][x + rect_w] {
                rect_w += 1;
            }

            let mut rect_h = 1;
            'check_rows: while y + rect_h < h {
                for dx in 0..rect_w {
                    if !solid[y + rect_h][x + dx] || visited[y + rect_h][x + dx] {
                        break 'check_rows;
                    }
                }
                rect_h += 1;
            }

            for dy in 0..rect_h {
                for dx in 0..rect_w {
                    visited[y + dy][x + dx] = true;
                }
            }

            spawn_collider(
                commands,
                x,
                y,
                rect_w,
                rect_h,
                compute_tile_size(mask, beauty),
                compute_origin_offset(mask, compute_tile_size(mask, beauty)),
            );

            merged.push((x, y, rect_w, rect_h));
        }
    }

    merged
}

fn spawn_collider(
    commands: &mut Commands,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    tile_size: f32,
    origin_offset: Vec2,
) {
    let center_x = x as f32 + w as f32 / 2.0 - 0.5;
    let center_y = y as f32 + h as f32 / 2.0 - 0.5;

    let world_pos = Vec3::new(
        origin_offset.x + center_x * tile_size,
        origin_offset.y - center_y * tile_size,
        0.5,
    );

    let half_width = w as f32 * tile_size / 2.0;
    let half_height = h as f32 * tile_size / 2.0;

    commands.spawn((
        Collider::cuboid(half_width, half_height),
        Transform::from_translation(world_pos),
        GlobalTransform::default(),
        Name::new("MergedWall"),
    ));
}
