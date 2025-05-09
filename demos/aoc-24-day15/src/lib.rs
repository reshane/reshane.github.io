mod input;
use input::INPUT;

use wasm_bindgen::prelude::*;
use std::{cell::RefCell, rc::Rc};
use web_sys::HtmlCanvasElement;
use web_sys::console;

use std::collections::HashSet;

type Pos = (i64, i64);

#[derive(Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default)]
struct Warehouse {
    size: (u32, u32),
    walls: HashSet<Pos>,
    boxes: HashSet<Pos>,
    robot: Pos,
    moves: Vec<Dir>,
}

fn parse_input(contents: &str) -> Warehouse {
    let split = contents.split("\n\n").collect::<Vec<&str>>();
    let map_str = split[0];
    let dir_str = split[1];

    let (mut w, mut h) = (0, 0);

    let mut warehouse = Warehouse::default();

    for (y, line) in map_str.lines().filter(|line| { !line.is_empty() }).enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    // wall
                    warehouse.walls.insert((x as i64, y as i64));
                },
                'O' => {
                    // box
                    warehouse.boxes.insert((x as i64, y as i64));
                },
                '@' => {
                    warehouse.robot = (x as i64, y as i64);
                },
                '.' => {},
                _ => {},
            }
            w = x;
        }
        h = y;
    }
    warehouse.size = (w as u32 + 1, h as u32 + 1);

    dir_str.chars()
        .filter_map(|c| {
            match c {
                '^' => Some(Dir::Up),
                'v' => Some(Dir::Down),
                '<' => Some(Dir::Left),
                '>' => Some(Dir::Right),
                _ => None,
            }
        })
        .for_each(|dir| { warehouse.moves.push(dir) });

    warehouse
}

impl Warehouse {
    fn step(&mut self) {
        let dir = self.moves.remove(0);
        let dir_vec = match dir {
            Dir::Up => (0_i64, -1_i64),
            Dir::Down => (0_i64, 1_i64),
            Dir::Left => (-1_i64, 0_i64),
            Dir::Right => (1_i64, 0_i64),
        };

        let mut target = (self.robot.0 + dir_vec.0, self.robot.1 + dir_vec.1);
        let robot_next_pos = target.clone();
        let mut boxes_to_move = Vec::<Pos>::new();

        // update target until it is not in the boxes
        while self.boxes.contains(&target) {
            boxes_to_move.push(target);
            target.1 += dir_vec.1;
            target.0 += dir_vec.0;
        }

        if self.walls.contains(&target) {
            // there is a wall & we don't move
            return;
        }

        // the vector of boxes are a vertical line of boxes
        // we can just move the first encountered
        // to the empty position after last (aka target)
        if boxes_to_move.len() > 0 {
            self.boxes.remove(&robot_next_pos);
            self.boxes.insert(target);
        }
        self.robot = robot_next_pos;
    }

}

const GRID_COLOR: &'static str = "#CCCCCC";
const CELL_SIZE: u32 = 10;
const WALL_COLOR: &'static str  = "#FFFFFF";
const BOX_COLOR: &'static str = "#005500";
const EMPTY_COLOR: &'static str = "#000000";
const ROBOT_COLOR: &'static str = "#FF0000";

fn window() -> web_sys::Window {
    web_sys::window().expect("should have a window in this context")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
    .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should be able to request animation frame");
}

fn document() -> web_sys::Document {
    window().document().expect("window should have a document")
}

fn canvas() -> web_sys::HtmlCanvasElement {
    let canvas = document().get_element_by_id("canvas").unwrap();
    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));

    let g = f.clone();
    {
        let mut warehouse = w_parse_input(INPUT);
        let (w, h) = warehouse.size;
        let canvas = canvas();
        console::log_1(&w.into());
        console::log_1(&h.into());
        canvas.set_attribute("width", format!("{}", (CELL_SIZE + 1) * w + 1).as_str())?;
        canvas.set_attribute("height", format!("{}", (CELL_SIZE + 1) * h + 1).as_str())?;
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if warehouse.moves.is_empty() {
                let ans = warehouse.boxes
                    .iter()
                    .fold(0, |acc, b| {
                        acc + (b.1 * 100) + b.0
                    });
                console::log_1(&ans.into());
                return;
            }
            warehouse.step();
            draw_warehouse(&canvas, &warehouse, w, h);
            draw_grid(&canvas, w, h);
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
    }

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn draw_warehouse(canvas: &HtmlCanvasElement, warehouse: &WWarehouse, width: u32, height: u32) {
    let ctx = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    ctx.begin_path();

    for row in 0..=height {
        for col in 0..=width {
            ctx.set_fill_style_str(EMPTY_COLOR);
            if warehouse.walls.contains(&(col as i64, row as i64)) {
                ctx.set_fill_style_str(WALL_COLOR);
            }
            if warehouse.boxes.contains(&(col as i64, row as i64)) {
                ctx.set_fill_style_str(BOX_COLOR);
            }
            if warehouse.robot == (col as i64, row as i64) {
                ctx.set_fill_style_str(ROBOT_COLOR);
            }
            if warehouse.boxes.contains(&(col as i64, row as i64)) {
                ctx.set_fill_style_str(BOX_COLOR);
                ctx.fill_rect(
                    (col * (CELL_SIZE + 1) + 2 + CELL_SIZE) as f64,
                    (row * (CELL_SIZE + 1) + 1) as f64,
                    CELL_SIZE as f64, CELL_SIZE as f64
                );
            }

            if !warehouse.boxes.contains(&(col as i64 - 1, row as i64)) {
                ctx.fill_rect(
                    (col * (CELL_SIZE + 1) + 1) as f64,
                    (row * (CELL_SIZE + 1) + 1) as f64,
                    CELL_SIZE as f64, CELL_SIZE as f64
                );
            }
        }
    }
}

fn draw_grid(canvas: &HtmlCanvasElement, width: u32, height: u32) {
    let ctx = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    ctx.begin_path();
    ctx.set_stroke_style_str(GRID_COLOR);

    // Vertical lines
    for i in 0..=width {
        ctx.move_to((i * (CELL_SIZE + 1) + 1) as f64, 0_f64);
        ctx.line_to((i * (CELL_SIZE + 1) + 1) as f64, ((CELL_SIZE + 1) * height + 1) as f64);
    }

    // Horizontal lines
    for i in 0..=height {
        ctx.move_to(0_f64, (i * (CELL_SIZE + 1) + 1) as f64);
        ctx.line_to(((CELL_SIZE + 1) * width + 1) as f64, (i * (CELL_SIZE + 1) + 1) as f64);
    }

    ctx.stroke();
}

#[derive(Debug, Default)]
struct WWarehouse {
    dims: Pos,
    size: (u32, u32),
    walls: HashSet<Pos>,
    boxes: HashSet<Pos>,
    robot: Pos,
    moves: Vec<Dir>,
}

fn w_parse_input(contents: &str) -> WWarehouse {
    let split = contents.split("\n\n").collect::<Vec<&str>>();
    let map_str = split[0];
    let dir_str = split[1];

    let mut dims = (0, 0);
    let (mut rows, mut cols) = (0, 0);

    let mut warehouse = WWarehouse::default();

    for (y, line) in map_str.lines().filter(|line| { !line.is_empty() }).enumerate() {
        dims.1 = y as i64;
        rows = y as u32;
        for (x, c) in line.chars().enumerate() {
            cols = x as u32;
            let x = x * 2;
            dims.0 = x as i64 + 1;
            match c {
                '#' => {
                    // wall
                    warehouse.walls.insert((x as i64, y as i64));
                    warehouse.walls.insert((x as i64 + 1, y as i64));
                },
                'O' => {
                    // box
                    warehouse.boxes.insert((x as i64, y as i64));
                },
                '@' => {
                    warehouse.robot = (x as i64, y as i64);
                },
                '.' => {},
                _ => {},
            }
        }
    }

    warehouse.dims = dims;
    warehouse.size = ((cols + 1) * 2, rows + 1);

    dir_str.chars()
        .filter_map(|c| {
            match c {
                '^' => Some(Dir::Up),
                'v' => Some(Dir::Down),
                '<' => Some(Dir::Left),
                '>' => Some(Dir::Right),
                _ => None,
            }
        })
        .for_each(|dir| { warehouse.moves.push(dir) });

    warehouse
}

impl WWarehouse {
    fn step(&mut self) {
        let dir = self.moves.remove(0);
        let dir_vec = match dir {
            Dir::Up => (0_i64, -1_i64),
            Dir::Down => (0_i64, 1_i64),
            Dir::Left => (-1_i64, 0_i64),
            Dir::Right => (1_i64, 0_i64),
        };
        match dir {
            Dir::Up | Dir::Down => {
                // start from robot + dir_vec
                // if we have a box, put it in the vector (box & box + 1x)
                // then for each box in the vector
                // explore box + dir_vec
                // if we encounter a wall, return
                // if the vector is empty, we have to add dir_vec to all the boxes
                // and the robot
                let mut queue = Vec::<Pos>::new();
                let next = (self.robot.0, self.robot.1 + dir_vec.1);
                if self.walls.contains(&next) {
                    return;
                }
                if let Some(bx) = self.boxes.get(&next) {
                    // there is a box directly above
                    queue.push(*bx);
                }
                if let Some(bx) = self.boxes.get(&(next.0 - 1, next.1)) {
                    // there is a box to the left
                    queue.push(*bx);
                }

                let mut visited = HashSet::<Pos>::new();

                while !queue.is_empty() {
                    let current = queue.remove(0);
                    let next = (current.0, current.1 + dir_vec.1);

                    if self.walls.contains(&next) {
                        return;
                    }
                    if self.walls.contains(&(next.0 + 1, next.1)) {
                        return;
                    }

                    if let Some(bx) = self.boxes.get(&next) {
                        // there is a box directly above
                        queue.push(*bx);
                    }
                    if let Some(bx) = self.boxes.get(&(next.0 - 1, next.1)) {
                        // there is a box to the left
                        queue.push(*bx);
                    }
                    if let Some(bx) = self.boxes.get(&(next.0 + 1, next.1)) {
                        // there is a box to the right
                        queue.push(*bx);
                    }

                    visited.insert(current);
                }

                visited.iter().for_each(|b| {
                    self.boxes.remove(&b);
                });
                visited.iter().for_each(|b| {
                    self.boxes.insert((b.0, b.1 + dir_vec.1));
                });
                self.robot.1 += dir_vec.1;
            },
            Dir::Right | Dir::Left => {
                let mut target = (self.robot.0 + dir_vec.0, self.robot.1);
                let robot_next_pos = target.clone();

                if dir == Dir::Left {
                    target.0 += dir_vec.0;
                }

                let mut boxes_to_move = Vec::<Pos>::new();
                while self.boxes.contains(&target) {
                    boxes_to_move.push(target);
                    target.0 += 2 * dir_vec.0;
                }

                let mut wall_test = target;
                if dir == Dir::Left {
                    wall_test.0 += 1;
                }

                if self.walls.contains(&wall_test) {
                    // there is a wall & we don't move
                    return;
                }

                boxes_to_move.iter().for_each(|b| {
                    self.boxes.remove(&b);
                });
                boxes_to_move.iter().for_each(|b| {
                    self.boxes.insert((b.0 + dir_vec.0, b.1));
                });
                
                self.robot = robot_next_pos;
            },
        }
    }
}
