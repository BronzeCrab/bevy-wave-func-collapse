use rand::prelude::*;

use bevy::prelude::*;

use bevy::render::settings::*;
use bevy::render::RenderPlugin;

#[derive(Component)]
struct Sprites(Vec<Sprite>);

#[derive(Debug, Clone, PartialEq)]
enum TileOption {
    Blank,
    Down,
    Left,
    Right,
    Up,
}

#[derive(Debug, Clone)]
struct Tile {
    collapsed: bool,
    options: Vec<TileOption>,
    i: usize,
    j: usize,
    can_be_collapsed: bool,
}
const DIM: usize = 6;
const SPRITE_SIZE: f32 = 50.;
const GREEN: Color = Color::srgb(0., 0.2, 0.);

#[derive(Component)]
struct Grid(Vec<Tile>);

#[derive(Component)]
struct RectangleIndexes {
    grid_ind: usize,
    i: usize,
    j: usize,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }
                .into(),
                ..default()
            }),
            MeshPickingPlugin,
        ))
        .add_systems(Startup, setup)
        // .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // Text with one section
    commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("hello bevy!"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 32.0,
            ..default()
        },
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Relative,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            // width: Val::Percent(10.),
            // height: Val::Px(100.),
            top: Val::Px(10.0),
            width: Val::Percent(100.),
            height: Val::Px(32.0),
            // right: Val::Px(5.0),
            ..default()
        },
    ));

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

    commands.spawn(Sprites(sprites));

    let mut x_start: f32 = 0.0;
    let mut y_start: f32 = 0.0;
    for i in 0..DIM {
        for j in 0..DIM {
            let grid_ind: usize = i * DIM + j;
            commands
                .spawn((
                    Mesh2d(meshes.add(Rectangle::new(SPRITE_SIZE, SPRITE_SIZE))),
                    MeshMaterial2d(materials.add(GREEN)),
                    Transform::from_xyz(x_start, y_start, 0.0),
                    RectangleIndexes {
                        grid_ind: grid_ind,
                        i: i,
                        j: j,
                    },
                ))
                .observe(on_rect_click);
            x_start += SPRITE_SIZE + 10.0;
        }
        x_start = 0.0;
        y_start -= SPRITE_SIZE + 10.0;
    }

    // first stage - fill grid:
    let mut grid: Vec<Tile> = vec![];
    for i in 0..DIM {
        for j in 0..DIM {
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
                can_be_collapsed: true,
            });
        }
    }

    commands.spawn(Grid(grid));
}

fn on_rect_click(
    click: Trigger<Pointer<Click>>,
    mut transforms: Query<&mut Transform>,
    mut rect_indexes_q: Query<&RectangleIndexes>,
    mut commands: Commands,
    spites_q: Query<&Sprites>,
    mut grid_q: Query<&mut Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("click on rect happened");

    let rect_indexes: &RectangleIndexes = rect_indexes_q.get_mut(click.target).unwrap();
    let mut grid: &mut Vec<Tile> = &mut grid_q.single_mut().0;

    if grid[rect_indexes.grid_ind].can_be_collapsed {
        if let Ok(transform) = transforms.get_mut(click.target) {
            shuffle_tile_options(&mut grid[rect_indexes.grid_ind]);
            let tile_opt: TileOption = find_proper_tile_option(grid, &grid[rect_indexes.grid_ind]);

            commands.entity(click.target).remove::<Mesh2d>();
            let spr_struct = spites_q.single();
            let tile_option_index = match tile_opt {
                TileOption::Blank => 0,
                TileOption::Down => 1,
                TileOption::Left => 2,
                TileOption::Right => 3,
                TileOption::Up => 4,
            };
            let sprite: &Sprite = &spr_struct.0[tile_option_index];
            commands.spawn((
                sprite.clone(),
                Transform::from_xyz(transform.translation.x, transform.translation.y, 0.),
            ));

            println!("rect_index, {:?}", rect_indexes.grid_ind);

            grid[rect_indexes.grid_ind].options = vec![tile_opt];
            grid[rect_indexes.grid_ind].collapsed = true;

            if !all_cell_collapsed(grid) {
                update_near_cells_options(&mut grid, rect_indexes.i, rect_indexes.j);
                println!("grid after update_near_cells_TileOptions {:?}", grid);

                let indexes_2_collapse: Vec<usize> =
                    find_and_mark_random_tile_with_low_entropy(&mut grid);
                for (ind, material) in materials.iter_mut().enumerate() {
                    if indexes_2_collapse.contains(&ind) {
                        material.1.color = GREEN;
                    } else {
                        material.1.color = Color::BLACK;
                    }
                }
            } else {
                println!("All cells are collapsed!");
            }

            println!("grid after click {:?}", grid);
        } else {
            println!("Can't get transform");
        }
    } else {
        println!("Can't collide this cell yet");
    }
}

// fn update(_time: Res<Time>, mut sprite_position: Query<&mut Sprite>) {
//     for transform in &mut sprite_position {
//         println!("{:?}", transform);
//     }
// }

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

fn find_and_mark_random_tile_with_low_entropy(grid: &mut Vec<Tile>) -> Vec<usize> {
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
            let grid_cell: &mut Tile = &mut grid[grid_ind];
            if !grid_cell.collapsed && grid_cell.options.len() == lowest_entropy {
                indexes_2_collapse.push(grid_ind);
                grid_cell.can_be_collapsed = true;
            } else if !grid_cell.collapsed {
                grid_cell.can_be_collapsed = false;
            }
        }
    }

    if indexes_2_collapse.len() <= 0 {
        panic!("ERROR: indexes_2_collapse is empty, grid is {:?}", grid);
    }
    indexes_2_collapse
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

fn find_proper_tile_option(grid: &Vec<Tile>, tile_to_collapse: &Tile) -> TileOption {
    let mut opt_to_collapse: Option<TileOption> = None;

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

fn shuffle_tile_options(tile: &mut Tile) {
    if tile.options.len() <= 0 {
        panic!("ERROR: tile {:?} has zero TileOptions", tile);
    }
    let mut rng: ThreadRng = rand::rng();
    tile.options.shuffle(&mut rng);
}
