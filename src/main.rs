use bevy::prelude::*;
use bevy::render::settings::*;
use bevy::render::RenderPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }
                .into(),
                ..default()
            }),
        )
        .add_systems(Startup, setup)
        // .add_systems(Update, lol)
        .run();
}

#[derive(Debug)]
enum Option {
    Blank,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Tile {
    collapsed: bool,
    options: Vec<Option>,
}
const DIM: usize = 2;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("down.png")),
        Transform::from_xyz(100., 0., 0.),
    ));
    let sprite: Sprite = Sprite::from_image(asset_server.load("blank.png"));
    println!("sprite: {:?}", sprite);
    commands.spawn((sprite, Transform::from_xyz(30., 0., 0.)));

    let mut grid: Vec<Tile> = vec![];
    for _ in 0..DIM * DIM {
        grid.push(Tile {
            collapsed: false,
            options: vec![Option::Blank, Option::Down, Option::Left, Option::Right],
        });
    }
    println!("grid is {:?}", grid);
}

fn lol(_time: Res<Time>, mut sprite_position: Query<&mut Sprite>) {
    for transform in &mut sprite_position {
        println!("{:?}", transform);
    }
}
