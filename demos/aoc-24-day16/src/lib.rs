mod input;
use input::INPUT;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use web_sys::HtmlCanvasElement;
use web_sys::console;

const GRID_COLOR: &'static str = "#CCCCCC";
const CELL_SIZE: u32 = 5;
const WALL_COLOR: &'static str  = "#FFFFFF";
const EMPTY_COLOR: &'static str = "#000000";
const VISITED_COLOR: &'static str = "#005500";
const NBOR_COLOR: &'static str = "#00AA00";
const START_COLOR: &'static str = "#FF0000";

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
        let mut maze = parse_input(INPUT);
        let (w, h) = (maze.mz.len() as u32, maze.mz[0].len() as u32);
        console::log_1(&format!("{:?}", (w, h)).into());
        let canvas = canvas();
        canvas.set_attribute("width", format!("{}", (CELL_SIZE + 1) * w + 1).as_str())?;
        canvas.set_attribute("height", format!("{}", (CELL_SIZE + 1) * h + 1).as_str())?;
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            // step for everything currently in the queue
            // that way we draw the maze only after all queued nodes
            // have been processed
            let mut steps = maze.queue.len();
            loop {
                if let Some(ans) = maze.step() {
                    console::log_1(&format!("Answer: {:?}", ans).into());
                    return;
                }
                steps -= 1;
                if steps == 0 {
                    break;
                }
            }

            draw_grid(&canvas, w, h);
            draw_maze(&canvas, &maze, w as usize, h as usize);
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
    }

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn draw_maze(canvas: &HtmlCanvasElement, maze: &Maze, width: usize, height: usize) {
    let ctx = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    ctx.begin_path();

    for row in 0..height {
        for col in 0..width {
            ctx.set_fill_style_str(EMPTY_COLOR);
            match maze.mz[row][col] {
                MazeTile::Path => {
                    if maze.visited.contains_key(&(col as i64, row as i64)) {
                        ctx.set_fill_style_str(VISITED_COLOR);
                    }
                    if maze.queue.iter().any(|(pos, _, _)| {
                        pos == &(col as i64, row as i64)
                    }) {
                        ctx.set_fill_style_str(NBOR_COLOR);
                    }
                }
                MazeTile::Wall => {
                    ctx.set_fill_style_str(WALL_COLOR);
                }
                MazeTile::Start | MazeTile::End => {
                    ctx.set_fill_style_str(START_COLOR);
                }
            }

            ctx.fill_rect(
                (col as u32 * (CELL_SIZE + 1) + 1) as f64,
                (row as u32 * (CELL_SIZE + 1) + 1) as f64,
                CELL_SIZE as f64, CELL_SIZE as f64
            );
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

type Point = (i64, i64);

#[derive(Debug, PartialEq)]
enum MazeTile {
    Path,
    Wall,
    Start,
    End,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
enum Dir {
    North,
    South,
    #[default]
    East,
    West,
}

#[derive(Debug, Default)]
struct Maze {
    mz: Vec<Vec<MazeTile>>,
    queue: Vec<(Point, Dir, i64)>,
    visited: HashMap<Point, i64>,
    min_end: i64,
}

impl Maze {
    fn step(&mut self) -> Option<i64> {
        if self.queue.is_empty() {
            return Some(self.min_end);
        }
        let curr = self.queue.remove(0);
        if curr.1 != Dir::East && curr.0.0 > 0 {
            // explore tile to left
            let nbor = &self.mz[curr.0.1 as usize][(curr.0.0 - 1) as usize];
            let nbor_pos = (curr.0.0 - 1, curr.0.1);
            let mut n_score = curr.2 + 1;
            if curr.1 != Dir::West {
                n_score += 1000;
            }
            match nbor {
                MazeTile::End => {
                    self.min_end = std::cmp::min(n_score, self.min_end);
                },
                MazeTile::Path => {
                    if !self.visited.contains_key(&nbor_pos) {
                        // if we got here faster than before, push to queue and replace in hm
                        self.queue.push((nbor_pos, Dir::West, n_score));
                        self.visited.insert(nbor_pos, n_score);
                    } else {
                        let prev_score = self.visited.get(&nbor_pos).unwrap();
                        if n_score < *prev_score {
                            self.queue.push((nbor_pos, Dir::West, n_score));
                            self.visited.insert(nbor_pos, n_score);
                        }
                    }
                },
                MazeTile::Start | MazeTile::Wall => {},
            }
        }
        if curr.1 != Dir::West && 
            (curr.0.0 as usize) < self.mz[curr.0.1 as usize].len() {
            // explore tile to right
            let nbor = &self.mz[curr.0.1 as usize][(curr.0.0 + 1) as usize];
            let nbor_pos = (curr.0.0 + 1, curr.0.1);
            let mut n_score = curr.2 + 1;
            if curr.1 != Dir::East {
                n_score += 1000;
            }
            match nbor {
                MazeTile::End => {
                    self.min_end = std::cmp::min(n_score, self.min_end);
                },
                MazeTile::Path => {
                    if !self.visited.contains_key(&nbor_pos) {
                        // if we got here faster than before, push to self.queue and replace in hm
                        self.queue.push((nbor_pos, Dir::East, n_score));
                        self.visited.insert(nbor_pos, n_score);
                    } else {
                        let prev_score = self.visited.get(&nbor_pos).unwrap();
                        if n_score < *prev_score {
                            self.queue.push((nbor_pos, Dir::East, n_score));
                            self.visited.insert(nbor_pos, n_score);
                        }
                    }
                },
                MazeTile::Start | MazeTile::Wall => {},
            }
        }
        if curr.1 != Dir::North && 
            (curr.0.1 as usize) < self.mz.len() {
            // explore tile below
            let nbor = &self.mz[(curr.0.1 + 1) as usize][curr.0.0 as usize];
            let nbor_pos = (curr.0.0, curr.0.1 + 1);
            let mut n_score = curr.2 + 1;
            if curr.1 != Dir::South {
                n_score += 1000;
            }
            match nbor {
                MazeTile::End => {
                    self.min_end = std::cmp::min(n_score, self.min_end);
                },
                MazeTile::Path => {
                    if !self.visited.contains_key(&nbor_pos) {
                        // if we got here faster than before, push to self.queue and replace in hm
                        self.queue.push((nbor_pos, Dir::South, n_score));
                        self.visited.insert(nbor_pos, n_score);
                    } else {
                        let prev_score = self.visited.get(&nbor_pos).unwrap();
                        if n_score < *prev_score {
                            self.queue.push((nbor_pos, Dir::South, n_score));
                            self.visited.insert(nbor_pos, n_score);
                        }
                    }
                },
                MazeTile::Start | MazeTile::Wall => {},
            }
        }
        if curr.1 != Dir::South && curr.0.1 > 0 {
            // explore tile above
            let nbor = &self.mz[(curr.0.1 - 1) as usize][curr.0.0 as usize];
            let nbor_pos = (curr.0.0, curr.0.1 - 1);
            let mut n_score = curr.2 + 1;
            if curr.1 != Dir::North {
                n_score += 1000;
            }
            match nbor {
                MazeTile::End => {
                    self.min_end = std::cmp::min(n_score, self.min_end);
                },
                MazeTile::Path => {
                    if !self.visited.contains_key(&nbor_pos) {
                        // if we got here faster than before, push to self.queue and replace in hm
                        self.queue.push((nbor_pos, Dir::North, n_score));
                        self.visited.insert(nbor_pos, n_score);
                    } else {
                        let prev_score = self.visited.get(&nbor_pos).unwrap();
                        if n_score < *prev_score {
                            self.queue.push((nbor_pos, Dir::North, n_score));
                            self.visited.insert(nbor_pos, n_score);
                        }
                    }
                },
                MazeTile::Start | MazeTile::Wall => {},
            }
        }
        None
    }
}

fn parse_input(contents: &str) -> Maze {
    let mut rd_pos = Point::default();
    let mut end = Point::default();
    let mz = contents.lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    match c {
                        'S' => {
                            rd_pos = (x as i64, y as i64);
                            Some(MazeTile::Start)
                        },
                        'E' => {
                            end = (x as i64, y as i64);
                            Some(MazeTile::End)
                        }
                        '#' => Some(MazeTile::Wall),
                        '.' => Some(MazeTile::Path),
                        _ => None,
                    }
                }).collect::<Vec<MazeTile>>()
        }).collect::<Vec<Vec<MazeTile>>>();
    Maze { 
        mz,
        queue: vec![(rd_pos, Dir::default(), 0)],
        visited: HashMap::new(),
        min_end: i64::MAX,
    }
}
