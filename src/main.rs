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

#[derive(Debug, Clone)]
enum Option {
    Blank,
    Down,
    Left,
    Right,
}
const NUM_OF_OPTIONS: u8 = 4;

#[derive(Debug, Clone)]
struct Tile {
    collapsed: bool,
    options: Vec<Option>,
}
const DIM: usize = 2;
const SPRITE_SIZE: f32 = 50.;

fn check_possible_i_and_j(i: i32, j: i32) -> bool {
    if i >= 0 && j >= 0 && i < DIM as i32 && j < DIM as i32 {
        return true;
    }
    false
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
                    let new_left_cell_opt: Vec<Option> = match collapsed_cell.options[0] {
                        Option::Blank => vec![Option::Blank, Option::Left],
                        Option::Down => vec![Option::Down, Option::Right],
                        Option::Left => vec![Option::Down, Option::Right],
                        Option::Right => vec![Option::Blank, Option::Left],
                    };
                    let left_cell: &mut Tile = &mut grid[possible_left_ind as usize];
                    if !left_cell.collapsed {
                        left_cell.options = new_left_cell_opt;
                    }
                }

                let poss_right_i: i32 = i as i32;
                let poss_right_j: i32 = j as i32 + 1;
                if check_possible_i_and_j(poss_right_i, poss_right_j) {
                    let possible_right_ind: i32 = poss_right_i * (DIM as i32) + poss_right_j;
                    let new_right_cell_opt: Vec<Option> = match collapsed_cell.options[0] {
                        Option::Blank => vec![Option::Blank, Option::Right],
                        Option::Down => vec![Option::Down, Option::Left],
                        Option::Left => vec![Option::Blank, Option::Right],
                        Option::Right => vec![Option::Down, Option::Left],
                    };
                    let right_cell: &mut Tile = &mut grid[possible_right_ind as usize];
                    if !right_cell.collapsed {
                        right_cell.options = new_right_cell_opt;
                    }
                }

                let poss_top_i: i32 = i as i32 - 1;
                let poss_top_j: i32 = j as i32;
                if check_possible_i_and_j(poss_top_i, poss_top_j) {
                    let possible_top_ind: i32 = poss_top_i * (DIM as i32) + poss_top_j;
                    let new_top_cell_opt: Vec<Option> = match collapsed_cell.options[0] {
                        Option::Blank => vec![Option::Blank],
                        Option::Down => vec![Option::Blank],
                        Option::Left => vec![Option::Down, Option::Left, Option::Right],
                        Option::Right => vec![Option::Down, Option::Left, Option::Right],
                    };
                    let top_cell: &mut Tile = &mut grid[possible_top_ind as usize];
                    if !top_cell.collapsed {
                        top_cell.options = new_top_cell_opt;
                    }
                }

                let poss_btm_i: i32 = i as i32 + 1;
                let poss_btm_j: i32 = j as i32;
                if check_possible_i_and_j(poss_btm_i, poss_btm_j) {
                    let possible_bottom_ind: i32 = poss_btm_i * (DIM as i32) + poss_btm_j;
                    let new_bottom_cell_opt: Vec<Option> = match collapsed_cell.options[0] {
                        Option::Blank => vec![Option::Blank, Option::Down],
                        Option::Down => vec![Option::Left, Option::Right],
                        Option::Left => vec![Option::Left, Option::Right],
                        Option::Right => vec![Option::Left, Option::Right],
                    };
                    let bottom_cell: &mut Tile = &mut grid[possible_bottom_ind as usize];
                    if !bottom_cell.collapsed {
                        bottom_cell.options = new_bottom_cell_opt;
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
    let mut indexes_2_collapse: Vec<usize> = vec![];
    let mut low_entropy_of_first: usize = 0;
    let mut i_j_idexes: Vec<(usize, usize)> = vec![];
    'outer_loop: for i in 0..DIM {
        for j in 0..DIM {
            let grid_ind: usize = i * DIM + j;
            let grid_cell: &Tile = &grid[grid_ind];
            if !grid_cell.collapsed {
                if low_entropy_of_first == 0 {
                    low_entropy_of_first = grid_cell.options.len();
                } else if grid_cell.options.len() > low_entropy_of_first {
                    break 'outer_loop;
                }
                indexes_2_collapse.push(grid_ind);
                i_j_idexes.push((i, j));
            }
        }
    }

    let r_index = rand::rng().random_range(0..=indexes_2_collapse.len() - 1);
    let ind_2_collapase: usize = indexes_2_collapse[r_index];
    let tile: &mut Tile = &mut grid[ind_2_collapase];
    tile.collapsed = true;

    let r_opt_index = rand::rng().random_range(0..=tile.options.len() - 1);
    let opt: &Option = &tile.options[r_opt_index];
    tile.options = vec![opt.clone()];

    i_j_idexes[r_index]
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
                let opt: Option = match r_option_index {
                    0 => Option::Blank,
                    1 => Option::Down,
                    2 => Option::Left,
                    3 => Option::Right,
                    _ => panic!(),
                };
                grid.push(Tile {
                    collapsed: true,
                    options: vec![opt],
                });
            } else {
                grid.push(Tile {
                    collapsed: false,
                    options: vec![Option::Blank, Option::Down, Option::Left, Option::Right],
                });
            }
        }
    }

    println!("Collapsed cell's i={}, j={}", random_cell_i, random_cell_j);

    // second stage, here we need to update entropy of near cells:
    update_near_cells_options(&mut grid, random_cell_i, random_cell_j);
    println!("grid after update_near_cells_options {:?}", grid);

    // third stage, main loop
    while !all_cell_collapsed(&grid) {
        grid.sort_by(|a, b| a.options.len().partial_cmp(&b.options.len()).unwrap());
        let i_j_tuple: (usize, usize) = do_collapse_random_with_low_entropy(&mut grid);
        update_near_cells_options(&mut grid, i_j_tuple.0, i_j_tuple.1);
    }

    // last stage, just display results
    let mut y_shift = 0.;
    for i in 0..DIM {
        let mut x_shift = 0.;
        for j in 0..DIM {
            let grid_ind: usize = i * DIM + j;
            let grid_cell: &Tile = &grid[grid_ind];
            if grid_cell.collapsed && grid_cell.options.len() == 1 {
                let r_index = rand::rng().random_range(0..=grid_cell.options.len() - 1);
                let option: &Option = &grid_cell.options[r_index];
                let sprite: &Sprite = match option {
                    Option::Blank => &sprites[0],
                    Option::Down => &sprites[1],
                    Option::Left => &sprites[2],
                    Option::Right => &sprites[3],
                };
                commands.spawn((sprite.clone(), Transform::from_xyz(x_shift, y_shift, 0.)));
            } else {
                panic!(
                    "ERROR: {}, {}, i={i}, j={j}, options={:?}",
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
