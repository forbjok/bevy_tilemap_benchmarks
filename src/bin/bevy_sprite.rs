use std::time::Instant;

use bevy::{math::vec3, prelude::*};

const TILEMAP_WIDTH: u32 = 1024;
const TILEMAP_HEIGHT: u32 = 1024;
const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(update_tiles_system)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct Tile {
    origin_sprite: u32,
}

fn update_tiles_system(mut count: Local<u32>, mut query: Query<(&Tile, &mut TextureAtlasSprite), With<Tile>>) {
    *count += 1;

    let upd_tiles = Instant::now();

    for (tile, mut sprite) in query.iter_mut() {
        sprite.index = ((tile.origin_sprite + *count) % 4) as usize;
    }

    dbg!(upd_tiles.elapsed());
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    // Load tilesheet texture and make a texture atlas from it
    let texture_handle = asset_server.load("textures/tilesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let offset = vec3(
        -(((TILEMAP_WIDTH * TILE_WIDTH) / 2) as f32),
        -(((TILEMAP_HEIGHT * TILE_HEIGHT) / 2) as f32),
        0.0,
    );

    // Create tile sprites
    for y in 0..TILEMAP_HEIGHT {
        let origin_sprite = y % 4;

        let ty = (y * TILE_HEIGHT) as f32;

        for x in 0..TILEMAP_WIDTH {
            let tx = (x * TILE_WIDTH) as f32;

            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform {
                        translation: vec3(tx, ty, 1.0) + offset,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Tile { origin_sprite });
        }
    }
}
