// use std::{thread, time};
use rand::prelude::*;

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

#[derive(Debug, Clone, PartialEq)]
enum TileOption {
    Blank,
    Down,
    Left,
    Right,
}
const NUM_OF_OPTIONS: u8 = 4;

#[derive(Debug, Clone)]
struct Tile {
    collapsed: bool,
    options: Vec<TileOption>,
    i: usize,
    j: usize,
}
const DIM: usize = 2;
const SPRITE_SIZE: f32 = 50.;

fn check_possible_i_and_j(i: i32, j: i32) -> bool {
    if i >= 0 && j >= 0 && i < DIM as i32 && j < DIM as i32 {
        return true;
    }
    false
}

fn find_intesection(a: Vec<TileOption>, b: Vec<TileOption>) -> Vec<TileOption> {
    let mut res: Vec<TileOption> = vec![];
    for a_opt in a {
        if b.contains(&a_opt) {
            res.push(a_opt);
        }
    }
    res
}

fn update_near_cells_options(
    grid: &mut Vec<Tile>,
    collapsed_cell_i: usize,
    collapsed_cell_j: usize,
) {
    for i in 0..DIM {
        for j in 0..DIM {
            if i == collapsed_cell_i && j == collapsed_cell_j {
                let collapsed_cell: Tile = grid[i * DIM + j].clone();

                let poss_left_i: i32 = i as i32;
                let poss_left_j: i32 = j as i32 - 1;
                if check_possible_i_and_j(poss_left_i, poss_left_j) {
                    let possible_left_ind: i32 = poss_left_i * (DIM as i32) + poss_left_j;
                    let new_left_cell_opt: Vec<TileOption> = match collapsed_cell.options[0] {
                        TileOption::Blank => vec![TileOption::Blank, TileOption::Left],
                        TileOption::Down => vec![TileOption::Down, TileOption::Right],
                        TileOption::Left => vec![TileOption::Down, TileOption::Right],
                        TileOption::Right => vec![TileOption::Blank, TileOption::Left],
                    };
                    let left_cell: &mut Tile = &mut grid[possible_left_ind as usize];
                    if !left_cell.collapsed {
                        left_cell.options =
                            find_intesection(left_cell.options.clone(), new_left_cell_opt);
                    }
                }

                let poss_right_i: i32 = i as i32;
                let poss_right_j: i32 = j as i32 + 1;
                if check_possible_i_and_j(poss_right_i, poss_right_j) {
                    let possible_right_ind: i32 = poss_right_i * (DIM as i32) + poss_right_j;
                    let new_right_cell_opt: Vec<TileOption> = match collapsed_cell.options[0] {
                        TileOption::Blank => vec![TileOption::Blank, TileOption::Right],
                        TileOption::Down => vec![TileOption::Down, TileOption::Left],
                        TileOption::Left => vec![TileOption::Blank, TileOption::Right],
                        TileOption::Right => vec![TileOption::Down, TileOption::Left],
                    };
                    let right_cell: &mut Tile = &mut grid[possible_right_ind as usize];
                    if !right_cell.collapsed {
                        right_cell.options =
                            find_intesection(right_cell.options.clone(), new_right_cell_opt);
                    }
                }

                let poss_top_i: i32 = i as i32 - 1;
                let poss_top_j: i32 = j as i32;
                if check_possible_i_and_j(poss_top_i, poss_top_j) {
                    let possible_top_ind: i32 = poss_top_i * (DIM as i32) + poss_top_j;
                    let new_top_cell_opt: Vec<TileOption> = match collapsed_cell.options[0] {
                        TileOption::Blank => vec![TileOption::Blank],
                        TileOption::Down => vec![TileOption::Blank],
                        TileOption::Left => {
                            vec![TileOption::Down, TileOption::Left, TileOption::Right]
                        }
                        TileOption::Right => {
                            vec![TileOption::Down, TileOption::Left, TileOption::Right]
                        }
                    };
                    let top_cell: &mut Tile = &mut grid[possible_top_ind as usize];
                    if !top_cell.collapsed {
                        top_cell.options =
                            find_intesection(top_cell.options.clone(), new_top_cell_opt);
                    }
                }

                let poss_btm_i: i32 = i as i32 + 1;
                let poss_btm_j: i32 = j as i32;
                if check_possible_i_and_j(poss_btm_i, poss_btm_j) {
                    let possible_bottom_ind: i32 = poss_btm_i * (DIM as i32) + poss_btm_j;
                    let new_bottom_cell_opt: Vec<TileOption> = match collapsed_cell.options[0] {
                        TileOption::Blank => vec![TileOption::Blank, TileOption::Down],
                        TileOption::Down => vec![TileOption::Left, TileOption::Right],
                        TileOption::Left => vec![TileOption::Left, TileOption::Right],
                        TileOption::Right => vec![TileOption::Left, TileOption::Right],
                    };
                    let bottom_cell: &mut Tile = &mut grid[possible_bottom_ind as usize];
                    if !bottom_cell.collapsed {
                        bottom_cell.options =
                            find_intesection(bottom_cell.options.clone(), new_bottom_cell_opt);
                    }
                }
            }
        }
    }
}

fn all_cell_collapsed(grid: &Vec<Tile>) -> bool {
    for cell in grid {
        if !cell.collapsed {
            return false;
        }
    }
    return true;
}

fn do_collapse_random_with_low_entropy(grid: &mut Vec<Tile>) -> (usize, usize) {
    let mut lowest_entropy: usize = usize::MAX;
    for i in 0..DIM {
        for j in 0..DIM {
            let grid_ind: usize = i * DIM + j;
            let grid_cell: &Tile = &grid[grid_ind];
            if !grid_cell.collapsed {
                if grid_cell.options.len() < lowest_entropy {
                    lowest_entropy = grid_cell.options.len();
                }
            }
        }
    }

    let mut indexes_2_collapse: Vec<usize> = vec![];
    for i in 0..DIM {
        for j in 0..DIM {
            let grid_ind: usize = i * DIM + j;
            let grid_cell: &Tile = &grid[grid_ind];
            if !grid_cell.collapsed && grid_cell.options.len() == lowest_entropy {
                indexes_2_collapse.push(grid_ind);
            }
        }
    }

    if indexes_2_collapse.len() <= 0 {
        panic!("ERROR: indexes_2_collapse is empty");
    }
    let r_index = rand::rng().random_range(0..=indexes_2_collapse.len() - 1);
    let ind_2_collapase: usize = indexes_2_collapse[r_index];
    let tile: &mut Tile = &mut grid[ind_2_collapase];
    tile.collapsed = true;

    if tile.options.len() <= 0 {
        panic!("ERROR: tile {:?} has zero TileOptions", tile);
    }
    // TODO: here we need not just take random, but also check if this
    // TileOptions is ineed possible!
    let mut rng = rand::rng();
    tile.options.shuffle(&mut rng);
    let mut opt_to_collapse: Option<TileOption> = None;
    for opt in &tile.options {
        // TODO: check opt for possible collapse
        opt_to_collapse = Some(opt.clone());
        break;
    }
    if opt_to_collapse == None {
        panic!("ERROR: cant find TileOption to collapse tile {:?}", tile);
    }
    tile.options = vec![opt_to_collapse.unwrap()];

    (tile.i, tile.j)
}

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

    // first stage - fill grid and pick one random cell, collapse it:
    let mut grid: Vec<Tile> = vec![];
    let random_cell_i: usize = rand::rng().random_range(0..=DIM - 1);
    let random_cell_j: usize = rand::rng().random_range(0..=DIM - 1);
    for i in 0..DIM {
        for j in 0..DIM {
            if i == random_cell_i && j == random_cell_j {
                let r_option_index: u8 = rand::rng().random_range(0..=NUM_OF_OPTIONS - 1);
                let opt: TileOption = match r_option_index {
                    0 => TileOption::Blank,
                    1 => TileOption::Down,
                    2 => TileOption::Left,
                    3 => TileOption::Right,
                    _ => panic!(),
                };
                grid.push(Tile {
                    collapsed: true,
                    options: vec![opt],
                    i: i,
                    j: j,
                });
            } else {
                grid.push(Tile {
                    collapsed: false,
                    options: vec![
                        TileOption::Blank,
                        TileOption::Down,
                        TileOption::Left,
                        TileOption::Right,
                    ],
                    i: i,
                    j: j,
                });
            }
        }
    }

    println!("Collapsed cell's i={}, j={}", random_cell_i, random_cell_j);

    // second stage, here we need to update entropy of near cells:
    update_near_cells_options(&mut grid, random_cell_i, random_cell_j);
    println!("grid after update_near_cells_TileOptions {:?}", grid);

    println!("entering the loop...");
    // third stage, main loop
    while !all_cell_collapsed(&grid) {
        let i_j_tuple: (usize, usize) = do_collapse_random_with_low_entropy(&mut grid);
        println!("grid after do_collapse_random_with_low_entropy {:?}", grid);
        update_near_cells_options(&mut grid, i_j_tuple.0, i_j_tuple.1);
        println!("grid after update_near_cells_TileOptions {:?}", grid);
    }

    println!("grid after all cells collapsed {:?}", grid);

    // last stage, just display results
    let mut y_shift = 0.;
    for i in 0..DIM {
        let mut x_shift = 0.;
        for j in 0..DIM {
            let grid_ind: usize = i * DIM + j;
            let grid_cell: &Tile = &grid[grid_ind];
            if grid_cell.collapsed && grid_cell.options.len() == 1 {
                let tile_option: &TileOption = &grid_cell.options[0];
                let sprite: &Sprite = match tile_option {
                    TileOption::Blank => &sprites[0],
                    TileOption::Down => &sprites[1],
                    TileOption::Left => &sprites[2],
                    TileOption::Right => &sprites[3],
                };
                commands.spawn((sprite.clone(), Transform::from_xyz(x_shift, -y_shift, 0.)));
            } else {
                panic!(
                    "ERROR: {}, {}, i={i}, j={j}, TileOptions={:?}",
                    grid_cell.collapsed,
                    grid_cell.options.len(),
                    grid_cell.options
                );
            }
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
