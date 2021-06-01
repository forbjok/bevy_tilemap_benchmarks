use std::time::Instant;

use bevy::prelude::*;
use bevy_ecs_tilemap::{
    prelude::{LayerBuilder, MapQuery, TileBundle},
    LayerSettings, Map, Tile, TilemapPlugin,
};

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
        .add_plugin(TilemapPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn update_tiles_system(mut commands: Commands, mut count: Local<u32>, mut map_query: MapQuery) {
    *count += 1;

    let upd_tiles = Instant::now();

    let mut i = *count % 4;

    for y in 0..TILEMAP_HEIGHT {
        let sprite_index = i % 4;

        for x in 0..TILEMAP_WIDTH {
            let tile_pos = UVec2::new(x, y);
            let tile = Tile {
                texture_index: sprite_index as u16,
                ..Default::default()
            };

            map_query.set_tile(&mut commands, tile_pos, tile, 0u16, 0u16).unwrap();
        }

        i += 1;
    }

    dbg!(upd_tiles.elapsed());
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    let texture_handle = asset_server.load("textures/tilesheet.png");
    let color_material_handle = color_materials.add(ColorMaterial::texture(texture_handle));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Create tilemap (bevy_ecs_tilemap)

    let tile_size = Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32);
    let texture_size = Vec2::new(64.0, 16.0);

    let mut layer_settings = LayerSettings::new(
        UVec2::new(TILEMAP_WIDTH / CHUNK_WIDTH, TILEMAP_HEIGHT / CHUNK_HEIGHT),
        UVec2::new(CHUNK_WIDTH, CHUNK_HEIGHT),
        tile_size,
        texture_size,
    );

    // Disable culling
    layer_settings.cull = false;

    let (layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 0u16);

    map_query.build_layer(&mut commands, layer_builder, color_material_handle);

    // Create map entity and component:
    let map_entity = commands.spawn().id();

    let mut map = Map::new(0u16, map_entity);
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
