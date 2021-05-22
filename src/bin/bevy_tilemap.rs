use std::time::Instant;

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

const TILEMAP_WIDTH: u32 = 1024;
const TILEMAP_HEIGHT: u32 = 1024;
const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;
const CHUNK_WIDTH: u32 = 64;
const CHUNK_HEIGHT: u32 = 64;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system(update_tiles_system.system())
        .add_plugins(TilemapDefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn update_tiles_system(mut query: Query<&mut Tilemap>, mut count: Local<usize>) {
    *count += 1;

    let upd_tiles = Instant::now();

    for mut tilemap in query.iter_mut() {
        let mut new_tiles: Vec<Tile<(u32, u32)>> = Vec::new();

        let mut i = *count % 4;

        for y in 0..TILEMAP_HEIGHT {
            let sprite_index = i % 4;

            for x in 0..TILEMAP_WIDTH {
                let point = (x, y);

                new_tiles.push(Tile {
                    point,
                    sprite_index,
                    ..Default::default()
                });
            }

            i += 1;
        }

        tilemap.insert_tiles(new_tiles).unwrap();
    }

    dbg!(upd_tiles.elapsed());
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle = asset_server.load("textures/tilesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Create tilemap (bevy_tilemap)
    let mut tilemap = Tilemap::builder()
        .auto_chunk()
        .topology(GridTopology::Square)
        .dimensions(TILEMAP_WIDTH, TILEMAP_HEIGHT)
        .chunk_dimensions(CHUNK_WIDTH, CHUNK_HEIGHT, 1)
        .texture_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .z_layers(1)
        .texture_atlas(texture_atlas_handle)
        .finish()
        .unwrap();

    for x in 0..(TILEMAP_WIDTH / CHUNK_WIDTH) {
        for y in 0..(TILEMAP_HEIGHT / CHUNK_HEIGHT) {
            tilemap.spawn_chunk((x, y)).unwrap();
        }
    }

    let tilemap_components = TilemapBundle {
        tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Transform {
            scale: Vec3::splat(1.0),
            translation: Vec3::new(-640.0, -360.0, 0.0),
            ..Default::default()
        },
        global_transform: Default::default(),
    };

    commands.spawn_bundle(tilemap_components);
}
