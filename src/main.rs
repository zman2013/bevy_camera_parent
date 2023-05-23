use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    window::{PresentMode, WindowResolution},
};

use bevy_inspector_egui::{quick::WorldInspectorPlugin};

pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

#[derive(Resource)]
pub struct PlaceHolderGraphics {
    texture_atlas: Handle<TextureAtlas>,
    player_index: usize,
    box_index: usize,
}

fn main() {
    App::new()
        .add_startup_system(load_graphics.in_base_set(StartupSet::PreStartup))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                resolution: WindowResolution::new(1600.0, 900.0),
                title: "DST clone".to_string(),
                resizable: false,
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(spawn_camera.in_base_set(StartupSet::PreStartup))
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_inventory_ui)
        .add_system(player_movement)
        .add_system(camera_follow)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Player)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let (mut player_transform, player) = player_query.single_mut();
    let _camera_transform = camera_query.single_mut();

    if keyboard.pressed(KeyCode::A) {
        player_transform.translation.x -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        player_transform.translation.x += player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::W) {
        player_transform.translation.y += player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        player_transform.translation.y -= player.speed * time.delta_seconds();
    }
}

/**
 * This function creates 5 Sprites and adds them as children of the camera. 
 * I want them to have a fixed position relative to the camera, 
 * so that they stay in the same position of the viewport even when the camera moves.
 */
fn spawn_inventory_ui(
    mut commands: Commands,
    graphics: Res<PlaceHolderGraphics>,
    camera_query: Query<Entity, With<Camera>>,
) {
    let camera = camera_query.single();
    let mut boxes = Vec::new();

    let sprite = TextureAtlasSprite::new(graphics.box_index);
    for i in 0..5 {
        let sprint_bundle = SpriteSheetBundle {
            sprite: sprite.clone(),
            texture_atlas: graphics.texture_atlas.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 40.0 * i as f32,
                    y: 40.0,
                    z: -1.0,
                },
                ..default()
            },
            ..default()
        };
        boxes.push(commands.spawn(sprint_bundle).id());
    }
    commands.entity(camera).push_children(&boxes);
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.camera_2d.clear_color = ClearColorConfig::Custom(Color::GREEN);

    commands.spawn(camera)
        .insert(VisibilityBundle::default());
}

fn spawn_player(mut commands: Commands, graphics: Res<PlaceHolderGraphics>) {
    let sprite = TextureAtlasSprite::new(graphics.player_index);
    commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: graphics.texture_atlas.clone(),
            ..default()
        })
        .insert(Player { speed: 300.0 })
        .insert(Name::new("Player"));
}

fn load_graphics(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_assets: ResMut<Assets<TextureAtlas>>,
) {
    let image_handle = assets.load("placeholder.png");
    let mut atlas = TextureAtlas::new_empty(image_handle, Vec2::splat(256.0));
    let player_index = atlas.add_texture(Rect {
        min: Vec2::splat(0.0),
        max: Vec2::splat(32.0),
    });

    let box_index = atlas.add_texture(Rect {
        min: Vec2::new(32.0, 32.0),
        max: Vec2::new(64.0, 64.0),
    });

    let atlas_handle = texture_assets.add(atlas);
    commands.insert_resource(PlaceHolderGraphics {
        texture_atlas: atlas_handle,
        player_index,
        box_index,
    });
}
