// use std::{thread, time};
use rand::prelude::*;

use bevy::prelude::*;
use bevy::render::settings::*;
use bevy::render::RenderPlugin;
use bevy::window::PrimaryWindow;

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
        .add_systems(Update, mouse_click_system)
        // .add_systems(Update, update)
        .run();
}

// This system prints messages when you press or release the left mouse button:
fn mouse_click_system(mouse_button_input: Res<ButtonInput<MouseButton>>, q_windows: Query<&Window, With<PrimaryWindow>>,) {
    // if mouse_button_input.pressed(MouseButton::Left) {
    //     info!("left mouse currently pressed");
    // }
    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
        // Games typically only have one window (the primary window)
        if let Some(position) = q_windows.single().cursor_position() {
            println!("Cursor is inside the primary window, at {:?}", position);
        } else {
            println!("Cursor is not in the game window.");
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TileOption {
    Blank,
    Down,
    Left,
    Right,
    Up,
}
const NUM_OF_OPTIONS: u8 = 5;

#[derive(Debug, Clone)]
struct Tile {
    collapsed: bool,
    options: Vec<TileOption>,
    i: usize,
    j: usize,
}
const DIM: usize = 10;
const SPRITE_SIZE: f32 = 50.;

fn find_intesection(a: Vec<TileOption>, b: Vec<TileOption>) -> Vec<TileOption> {
    let mut res: Vec<TileOption> = vec![];
    for a_opt in a {
        if b.contains(&a_opt) {
            res.push(a_opt);
        }
    }
    res
}

fn get_possible_options(tile_opt: &TileOption, side: &str) -> Vec<TileOption> {
    if side == "left" {
        match tile_opt {
            TileOption::Blank => vec![TileOption::Blank, TileOption::Left],
            TileOption::Down => vec![TileOption::Down, TileOption::Right, TileOption::Up],
            TileOption::Left => vec![TileOption::Down, TileOption::Right, TileOption::Up],
            TileOption::Right => vec![TileOption::Blank, TileOption::Left],
            TileOption::Up => vec![TileOption::Right, TileOption::Down, TileOption::Up],
        }
    } else if side == "right" {
        match tile_opt {
            TileOption::Blank => vec![TileOption::Blank, TileOption::Right],
            TileOption::Down => vec![TileOption::Down, TileOption::Left, TileOption::Up],
            TileOption::Left => vec![TileOption::Blank, TileOption::Right],
            TileOption::Right => vec![TileOption::Down, TileOption::Left, TileOption::Up],
            TileOption::Up => vec![TileOption::Left, TileOption::Down, TileOption::Up],
        }
    } else if side == "top" {
        match tile_opt {
            TileOption::Blank => vec![TileOption::Blank, TileOption::Up],
            TileOption::Down => vec![TileOption::Blank, TileOption::Up],
            TileOption::Left => {
                vec![TileOption::Down, TileOption::Left, TileOption::Right]
            }
            TileOption::Right => {
                vec![TileOption::Down, TileOption::Left, TileOption::Right]
            }
            TileOption::Up => vec![TileOption::Left, TileOption::Right, TileOption::Down],
        }
    } else if side == "btm" {
        match tile_opt {
            TileOption::Blank => vec![TileOption::Blank, TileOption::Down],
            TileOption::Down => vec![TileOption::Left, TileOption::Right, TileOption::Up],
            TileOption::Left => vec![TileOption::Left, TileOption::Right, TileOption::Up],
            TileOption::Right => vec![TileOption::Left, TileOption::Right, TileOption::Up],
            TileOption::Up => vec![TileOption::Blank, TileOption::Down],
        }
    } else {
        panic!("ERROR: no such side {side}");
    }
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
                if poss_left_j >= 0 {
                    let left_ind: i32 = poss_left_i * (DIM as i32) + poss_left_j;
                    let left_cell: &mut Tile = &mut grid[left_ind as usize];
                    let new_left_cell_opt: Vec<TileOption> =
                        get_possible_options(&collapsed_cell.options[0], "left");
                    if !left_cell.collapsed {
                        left_cell.options =
                            find_intesection(left_cell.options.clone(), new_left_cell_opt);
                        if left_cell.options.len() == 0 {
                            panic!("No options for left cell");
                        }
                    }
                }

                let poss_right_i: i32 = i as i32;
                let poss_right_j: i32 = j as i32 + 1;
                if poss_right_j < DIM as i32 {
                    let right_ind: i32 = poss_right_i * (DIM as i32) + poss_right_j;
                    let right_cell: &mut Tile = &mut grid[right_ind as usize];
                    let new_right_cell_opt: Vec<TileOption> =
                        get_possible_options(&collapsed_cell.options[0], "right");
                    if !right_cell.collapsed {
                        right_cell.options =
                            find_intesection(right_cell.options.clone(), new_right_cell_opt);
                        if right_cell.options.len() == 0 {
                            panic!("No options for right cell");
                        }
                    }
                }

                let poss_top_i: i32 = i as i32 - 1;
                let poss_top_j: i32 = j as i32;
                if poss_top_i >= 0 {
                    let top_ind: i32 = poss_top_i * (DIM as i32) + poss_top_j;
                    let new_top_cell_opt: Vec<TileOption> =
                        get_possible_options(&collapsed_cell.options[0], "top");
                    let top_cell: &mut Tile = &mut grid[top_ind as usize];
                    if !top_cell.collapsed {
                        top_cell.options =
                            find_intesection(top_cell.options.clone(), new_top_cell_opt);
                        if top_cell.options.len() == 0 {
                            panic!("No options for top cell");
                        }
                    }
                }

                let poss_btm_i: i32 = i as i32 + 1;
                let poss_btm_j: i32 = j as i32;
                if poss_btm_i < DIM as i32 {
                    let bottom_ind: i32 = poss_btm_i * (DIM as i32) + poss_btm_j;
                    let new_bottom_cell_opt: Vec<TileOption> =
                        get_possible_options(&collapsed_cell.options[0], "btm");
                    let bottom_cell: &mut Tile = &mut grid[bottom_ind as usize];
                    if !bottom_cell.collapsed {
                        bottom_cell.options =
                            find_intesection(bottom_cell.options.clone(), new_bottom_cell_opt);
                        if bottom_cell.options.len() == 0 {
                            panic!("No options for bottom cell");
                        }
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

fn find_random_tile_with_low_entropy(grid: &Vec<Tile>) -> usize {
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
    let r_index: usize = rand::rng().random_range(0..=indexes_2_collapse.len() - 1);
    let ind_2_collapase: usize = indexes_2_collapse[r_index];

    return ind_2_collapase;
}

fn shuffle_tile_options(grid: &mut Vec<Tile>, tile_ind: usize) {
    let tile: &mut Tile = &mut grid[tile_ind];

    if tile.options.len() <= 0 {
        panic!("ERROR: tile {:?} has zero TileOptions", tile);
    }
    // here we need not just take random, but also check if this
    // TileOptions is indeed possible!
    let mut rng: ThreadRng = rand::rng();
    tile.options.shuffle(&mut rng);
}

fn check_side(
    grid: &Vec<Tile>,
    tile_to_collapse_i: usize,
    tile_to_collapse_j: usize,
    tile_to_collapse_opt: &TileOption,
    side: &str,
) -> bool {
    let mut poss_side_i: i32 = 0;
    let mut poss_side_j: i32 = 0;
    let mut ind_to_change: i32 = 0;
    let mut how_to_change: i32 = 0;

    let mut how_to_change_i: i32 = 0;
    let mut how_to_change_j: i32 = 0;

    match side {
        "left" => {
            poss_side_i = tile_to_collapse_i as i32;
            poss_side_j = tile_to_collapse_j as i32 - 1;
            ind_to_change = poss_side_j;
            how_to_change = -1;

            how_to_change_i = 0;
            how_to_change_j = -1;
        }
        "right" => {
            poss_side_i = tile_to_collapse_i as i32;
            poss_side_j = tile_to_collapse_j as i32 + 1;
            ind_to_change = poss_side_j;
            how_to_change = 1;

            how_to_change_i = 0;
            how_to_change_j = 1;
        }
        "top" => {
            poss_side_i = tile_to_collapse_i as i32 - 1;
            poss_side_j = tile_to_collapse_j as i32;
            ind_to_change = poss_side_i;
            how_to_change = -1;

            how_to_change_i = -1;
            how_to_change_j = 0;
        }
        "btm" => {
            poss_side_i = tile_to_collapse_i as i32 + 1;
            poss_side_j = tile_to_collapse_j as i32;
            ind_to_change = poss_side_i;
            how_to_change = 1;

            how_to_change_i = 1;
            how_to_change_j = 0;
        }
        _ => {
            panic!("Unknown side: {side}")
        }
    }

    let mut side_is_ok: bool = false;

    if ind_to_change >= 0 && ind_to_change < DIM as i32 {
        let mut current_tile_opt: &TileOption = tile_to_collapse_opt;
        let mut intersect: Vec<TileOption> = vec![];
        while ind_to_change >= 0 && ind_to_change < DIM as i32 {
            let side_ind: i32 = poss_side_i * (DIM as i32) + poss_side_j;
            let side_tile: &Tile = &grid[side_ind as usize];
            let side_possible_opts: Vec<TileOption> = get_possible_options(current_tile_opt, side);
            intersect = find_intesection(side_tile.options.clone(), side_possible_opts);
            if intersect.len() == 0 {
                side_is_ok = false;
                break;
            } else if intersect.len() == 1 {
                current_tile_opt = &intersect[0];
                side_is_ok = true;
            } else {
                side_is_ok = true;
                break;
            }
            ind_to_change += how_to_change;

            poss_side_i += how_to_change_i;
            poss_side_j += how_to_change_j;
        }
    } else {
        side_is_ok = true;
    }
    side_is_ok
}

fn find_proper_tile_option(grid: &Vec<Tile>, tile_ind: usize) -> TileOption {
    let mut opt_to_collapse: Option<TileOption> = None;

    let tile_to_collapse: &Tile = &grid[tile_ind];
    // check opt for possible collapse
    for i in 0..DIM {
        for j in 0..DIM {
            if i == tile_to_collapse.i && j == tile_to_collapse.j {
                for tile_to_collapse_opt in &tile_to_collapse.options {
                    let mut left_side_is_ok: bool = false;
                    let mut right_side_is_ok: bool = false;
                    let mut top_side_is_ok: bool = false;
                    let mut btm_side_is_ok: bool = false;

                    // go to the left and check all left cells:
                    left_side_is_ok = check_side(
                        &grid,
                        tile_to_collapse.i,
                        tile_to_collapse.j,
                        tile_to_collapse_opt,
                        "left",
                    );
                    if !left_side_is_ok {
                        continue;
                    }

                    // go to the right and check all right cells:
                    right_side_is_ok = check_side(
                        &grid,
                        tile_to_collapse.i,
                        tile_to_collapse.j,
                        tile_to_collapse_opt,
                        "right",
                    );
                    if !right_side_is_ok {
                        continue;
                    }

                    // go to the top and check all top cells:
                    top_side_is_ok = check_side(
                        &grid,
                        tile_to_collapse.i,
                        tile_to_collapse.j,
                        tile_to_collapse_opt,
                        "top",
                    );
                    if !top_side_is_ok {
                        continue;
                    }

                    // go to the btm and check all btm cells:
                    btm_side_is_ok = check_side(
                        &grid,
                        tile_to_collapse.i,
                        tile_to_collapse.j,
                        tile_to_collapse_opt,
                        "btm",
                    );
                    if !btm_side_is_ok {
                        continue;
                    }

                    if left_side_is_ok && right_side_is_ok && top_side_is_ok && btm_side_is_ok {
                        opt_to_collapse = Some(tile_to_collapse_opt.clone());
                        return opt_to_collapse.unwrap();
                    }
                }
            }
        }
    }
    if opt_to_collapse == None {
        panic!(
            "ERROR: cant find TileOption to collapse tile {:?}",
            tile_to_collapse
        );
    } else {
        panic!(
            "ERROR: found opt_to_collapse {:?}, but not return? WTF?",
            tile_to_collapse
        );
    }
}

fn do_collapse_tile(
    grid: &mut Vec<Tile>,
    tile_ind: usize,
    opt_to_collapse: TileOption,
) -> (usize, usize) {
    let tile: &mut Tile = &mut grid[tile_ind];
    tile.options = vec![opt_to_collapse];
    tile.collapsed = true;
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

    let mut sprite_4: Sprite = Sprite::from_image(asset_server.load("up.png"));
    sprite_4.custom_size = Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE));
    sprites.push(sprite_4);

    loop {
        println!("begin new loop iteration...");
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
                        4 => TileOption::Up,
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
                            TileOption::Up,
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
            let tile_ind: usize = find_random_tile_with_low_entropy(&grid);
            shuffle_tile_options(&mut grid, tile_ind);
            let tile_opt: TileOption = find_proper_tile_option(&grid, tile_ind);
            let i_j_tuple: (usize, usize) = do_collapse_tile(&mut grid, tile_ind, tile_opt);
            println!("grid after do_collapse_tile {:?}", grid);
            update_near_cells_options(&mut grid, i_j_tuple.0, i_j_tuple.1);
            println!("grid after update_near_cells_options {:?}", grid);
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
                        TileOption::Up => &sprites[4],
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

        break;
        use std::{thread, time};

        let sec = time::Duration::from_millis(1000);

        thread::sleep(sec);
    }
}

// fn main() {

// }

fn update(_time: Res<Time>, mut sprite_position: Query<&mut Sprite>) {
    for transform in &mut sprite_position {
        println!("{:?}", transform);
    }
}
