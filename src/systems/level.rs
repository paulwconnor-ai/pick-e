use bevy::prelude::*;
use bevy::render::texture::Image;
use bevy_rapier2d::prelude::*;

// Embed the cache file as a string on WASM
#[cfg(target_arch = "wasm32")]
const COLLISION_CACHE: &str = include_str!("../../assets/collision-cache.txt");
const DOWNSCALE_FACTOR: usize = 4;
const DEBUG_DRAW_COLLISIONS: bool = false;

#[derive(Resource)]
pub struct LevelAssets {
    pub background: Handle<Image>,
    pub spawned: bool,
}

pub fn setup_level_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background = asset_server.load("textures/map-beauty.png");

    commands.insert_resource(LevelAssets {
        background,
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

    // Try loading from cache first
    if let Some(text) = try_load_collision_cache("assets/collision-cache.txt") {
        if try_spawn_from_cache(&mut commands, &text, beauty_texture).is_some() {
            info!("Spawned level from collision cache");
            return;
        } else {
            warn!("Cache file found but invalid — falling back to mask");
        }
    } else {
        info!("No collision cache found — will generate from mask");
    }

    // Fallback: generate from mask
    let merged_rects = generate_colliders_from_beauty(beauty_texture, &mut commands);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let lines: Vec<String> = merged_rects
            .iter()
            .map(|(x, y, w, h)| format!("{x},{y},{w},{h}"))
            .collect();
        if let Err(e) = std::fs::write("assets/collision-cache.txt", lines.join("\n")) {
            warn!("Failed to write collision cache: {e}");
        }
    }

    info!(
        "Spawned {} colliders from mask (tile = {:.2})",
        merged_rects.len(),
        DOWNSCALE_FACTOR as f32
    );
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn try_load_collision_cache(_path: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        Some(COLLISION_CACHE.to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::fs::read_to_string(_path).ok()
    }
}

fn try_spawn_from_cache(commands: &mut Commands, text: &str, beauty: &Image) -> Option<()> {
    let tile_size = DOWNSCALE_FACTOR as f32;
    let origin_offset = compute_origin_offset(beauty, tile_size);

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

    info!("Spawned {num_colliders} colliders from cache");
    Some(())
}

pub fn compute_origin_offset(image: &Image, tile_size: f32) -> Vec2 {
    let scaled_w = image.size().x as f32 / DOWNSCALE_FACTOR as f32;
    let scaled_h = image.size().y as f32 / DOWNSCALE_FACTOR as f32;

    Vec2::new(
        -scaled_w * tile_size / 2.0 + tile_size / 2.0,
        scaled_h * tile_size / 2.0 - tile_size / 2.0,
    )
}

const BLUE_DOMINANCE_RATIO: f32 = 1.25;
const MIN_ALPHA: u8 = 8;

pub fn is_clearly_blue(r: u8, g: u8, b: u8, a: u8) -> bool {
    if a < MIN_ALPHA {
        return false;
    }
    let rf = r as f32;
    let gf = g as f32;
    let bf = b as f32;
    let avg_rg = (rf + gf) * 0.5;
    bf > rf && bf > gf && bf >= BLUE_DOMINANCE_RATIO * avg_rg
}

pub fn generate_colliders_from_beauty(
    beauty: &Image,
    commands: &mut Commands,
) -> Vec<(usize, usize, usize, usize)> {
    let full_w = beauty.size().x as usize;
    let full_h = beauty.size().y as usize;
    let stride = 4;
    let data = &beauty.data;

    let w = full_w / DOWNSCALE_FACTOR;
    let h = full_h / DOWNSCALE_FACTOR;

    let mut solid = vec![vec![false; w]; h];

    for y in 0..h {
        for x in 0..w {
            let orig_x = x * DOWNSCALE_FACTOR;
            let orig_y = y * DOWNSCALE_FACTOR;
            let idx = (orig_y * full_w + orig_x) * stride;

            let r = data[idx];
            let g = data[idx + 1];
            let b = data[idx + 2];
            let a = data[idx + 3];

            solid[y][x] = is_clearly_blue(r, g, b, a);
        }
    }

    let mut visited = vec![vec![false; w]; h];
    let mut merged = Vec::new();

    let tile_size = DOWNSCALE_FACTOR as f32;
    let origin_offset = compute_origin_offset(beauty, tile_size);

    for y in 0..h {
        for x in 0..w {
            if !solid[y][x] || visited[y][x] {
                continue;
            }

            // Grow horizontally
            let mut rect_w = 1;
            while x + rect_w < w && solid[y][x + rect_w] && !visited[y][x + rect_w] {
                rect_w += 1;
            }

            // Grow vertically
            let mut rect_h = 1;
            'grow: while y + rect_h < h {
                for dx in 0..rect_w {
                    if !solid[y + rect_h][x + dx] || visited[y + rect_h][x + dx] {
                        break 'grow;
                    }
                }
                rect_h += 1;
            }

            // Mark visited
            for dy in 0..rect_h {
                for dx in 0..rect_w {
                    visited[y + dy][x + dx] = true;
                }
            }

            // Skip tiny fragments
            if rect_w < 2 && rect_h < 2 {
                continue;
            }

            spawn_collider(commands, x, y, rect_w, rect_h, tile_size, origin_offset);

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

    let half_w = w as f32 * tile_size / 2.0;
    let half_h = h as f32 * tile_size / 2.0;

    commands.spawn((
        Collider::cuboid(half_w, half_h),
        Transform::from_translation(world_pos),
        GlobalTransform::default(),
        Name::new("MergedWall"),
    ));

    if DEBUG_DRAW_COLLISIONS {
        // Compute a color bucket (0 = red, 1 = green, 2 = blue)
        let bucket =
            (x.wrapping_mul(31) ^ y.wrapping_mul(67) ^ w.wrapping_mul(97) ^ h.wrapping_mul(137))
                % 3;

        let color = match bucket {
            0 => Color::rgba(1.0, 0.2, 0.2, 0.1), // red
            1 => Color::rgba(0.2, 1.0, 0.2, 0.1), // green
            _ => Color::rgba(0.2, 0.2, 1.0, 0.1), // blue
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(2.0 * half_w, 2.0 * half_h)),
                    ..default()
                },
                transform: Transform::from_translation(world_pos),
                ..default()
            },
            Name::new("ColliderDebugSprite"),
        ));
    }
}
