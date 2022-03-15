use std::time::Instant;

use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::{
    prelude::{LayerBuilder, MapQuery, TileBundle},
    ChunkSize, LayerSettings, Map, MapSize, TextureSize, Tile, TilePos, TileSize, TilemapPlugin,
};

const TILEMAP_WIDTH: u32 = 1024;
const TILEMAP_HEIGHT: u32 = 1024;
const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;
const CHUNK_WIDTH: u32 = 64;
const CHUNK_HEIGHT: u32 = 64;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(set_texture_filters_to_nearest)
        .add_system(update_tiles_system)
        .add_plugin(TilemapPlugin)
        .add_startup_system(setup)
        .run();
}

/// Copy-pasted from the bevy_ecs_tilemap examples.
/// Without this, the tilemap will not even render.
pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        if let AssetEvent::Created { handle } = event {
            if let Some(mut texture) = textures.get_mut(handle) {
                texture.texture_descriptor.usage =
                    TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC | TextureUsages::COPY_DST;
            }
        }
    }
}

fn update_tiles_system(mut commands: Commands, mut count: Local<u32>, mut map_query: MapQuery) {
    *count += 1;

    let upd_tiles = Instant::now();

    let mut i = *count % 4;

    for y in 0..TILEMAP_HEIGHT {
        let sprite_index = i % 4;

        for x in 0..TILEMAP_WIDTH {
            let tile_pos = TilePos(x, y);
            let tile = Tile {
                texture_index: sprite_index as u16,
                ..Default::default()
            };

            if let Ok(tile_entity) = map_query.get_tile_entity(tile_pos, 0u16, 0u16) {
                commands.entity(tile_entity).insert(tile);
            } else {
                map_query.set_tile(&mut commands, tile_pos, tile, 0u16, 0u16).unwrap();
            }

            map_query.notify_chunk_for_tile(tile_pos, 0u16, 0u16);
        }

        i += 1;
    }

    dbg!(upd_tiles.elapsed());
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands, mut map_query: MapQuery) {
    let texture_handle = asset_server.load("textures/tilesheet.png");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Create tilemap (bevy_ecs_tilemap)

    let tile_size = TileSize(TILE_WIDTH as f32, TILE_HEIGHT as f32);
    let texture_size = TextureSize(64.0, 16.0);

    let mut layer_settings = LayerSettings::new(
        MapSize(TILEMAP_WIDTH / CHUNK_WIDTH, TILEMAP_HEIGHT / CHUNK_HEIGHT),
        ChunkSize(CHUNK_WIDTH, CHUNK_HEIGHT),
        tile_size,
        texture_size,
    );

    // Disable culling
    layer_settings.cull = false;

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 0u16);

    map_query.build_layer(&mut commands, layer_builder, texture_handle);

    map.add_layer(&mut commands, 0u16, layer_entity);

    // Insert Map component in map entity
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform {
            scale: Vec3::splat(1.0),
            translation: Vec3::new(-640.0, -360.0, 0.0),
            ..Default::default()
        })
        .insert(GlobalTransform::default());
}
