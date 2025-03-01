use rand::Rng;

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
        // .add_systems(Update, update)
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
const DIM: usize = 5;
const SPRITE_SIZE: f32 = 50.;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // commands.spawn((
    //     Sprite::from_image(asset_server.load("down.png")),
    //     Transform::from_xyz(0., 0., 0.),
    // ));

    let mut sprites: Vec<Sprite> = vec![];

    let mut sprite_0: Sprite = Sprite::from_image(asset_server.load("blank.png"));
    sprite_0.custom_size = Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE));
    sprites.push(sprite_0);

    let mut sprite_1: Sprite = Sprite::from_image(asset_server.load("down.png"));
    sprite_1.custom_size = Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE));
    sprites.push(sprite_1);

    let mut sprite_2: Sprite = Sprite::from_image(asset_server.load("left.png"));
    sprite_2.custom_size = Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE));
    sprites.push(sprite_2);

    let mut sprite_3: Sprite = Sprite::from_image(asset_server.load("right.png"));
    sprite_3.custom_size = Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE));
    sprites.push(sprite_3);

    // println!("sprite: {:?}", sprite);
    // let x = sprite.custom_size.unwrap().x;

    // commands.spawn((sprite, Transform::from_xyz(x, 0., 0.)));

    let mut grid: Vec<Tile> = vec![];
    for _ in 0..DIM * DIM {
        grid.push(Tile {
            collapsed: false,
            options: vec![Option::Blank, Option::Down, Option::Left, Option::Right],
        });
    }

    println!("grid is {:?}", grid);

    let mut y_shift = 0.;
    for i in 0..DIM {
        let mut x_shift = 0.;
        for j in 0..DIM {
            let grid_ind: usize = i * DIM + j;
            let r_index = rand::rng().random_range(0..=sprites.len() - 1);
            let option: &Option = &grid[grid_ind].options[r_index];
            let sprite = match option {
                Option::Blank => &sprites[r_index],
                Option::Down => &sprites[r_index],
                Option::Left => &sprites[r_index],
                Option::Right => &sprites[r_index],
            };
            commands.spawn((sprite.clone(), Transform::from_xyz(x_shift, y_shift, 0.)));
            x_shift += SPRITE_SIZE;
        }
        y_shift += SPRITE_SIZE;
    }
}

fn update(_time: Res<Time>, mut sprite_position: Query<&mut Sprite>) {
    for transform in &mut sprite_position {
        println!("{:?}", transform);
    }
}
