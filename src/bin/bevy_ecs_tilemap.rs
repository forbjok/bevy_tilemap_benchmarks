use std::time::Instant;

use bevy::prelude::*;
use bevy_ecs_tilemap::{Map, Tile, TilemapPlugin};

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

fn update_tiles_system(mut commands: Commands, mut query: Query<&mut Map>, mut count: Local<u32>) {
    *count += 1;

    let upd_tiles = Instant::now();

    for mut tilemap in query.iter_mut() {
        let mut i = *count % 4;

        for y in 0..TILEMAP_HEIGHT {
            let sprite_index = i % 4;

            for x in 0..TILEMAP_WIDTH {
                let tile = Tile {
                    texture_index: sprite_index,
                    ..Default::default()
                };

                tilemap.add_tile(&mut commands, UVec2::new(x, y), tile, true).unwrap();
            }

            i += 1;
        }
    }

    dbg!(upd_tiles.elapsed());
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("textures/tilesheet.png");
    //let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1);
    //let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let color_material_handle = color_materials.add(ColorMaterial::texture(texture_handle));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Create tilemap (bevy_ecs_tilemap)
    {
        let tile_size = Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32);
        let texture_size = Vec2::new(64.0, 16.0);

        let mut map_settings = bevy_ecs_tilemap::MapSettings::new(
            UVec2::new(TILEMAP_WIDTH / CHUNK_WIDTH, TILEMAP_HEIGHT / CHUNK_HEIGHT),
            UVec2::new(CHUNK_WIDTH, CHUNK_HEIGHT),
            tile_size,
            texture_size,
            0,
        );

        // Disable culling as a workaround for bug caused by not using bevy_ecs_tilemap not using GlobalTransform
        map_settings.cull = false;

        let mut map = bevy_ecs_tilemap::Map::new(map_settings);

        // Map
        let map_entity = commands.spawn().id();

        map.build(&mut commands, &mut meshes, color_material_handle, map_entity, false);

        commands.entity(map_entity).insert_bundle(bevy_ecs_tilemap::MapBundle {
            map,
            transform: Transform {
                scale: Vec3::splat(1.0),
                translation: Vec3::new(-640.0, -360.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
    };
}
