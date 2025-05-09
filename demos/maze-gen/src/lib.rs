use wasm_bindgen::prelude::*;
use std::{cell::RefCell, rc::Rc};
use web_sys::HtmlCanvasElement;
use std::collections::HashSet;

const CELL_SIZE: u32 = 5;
const PATH_COLOR: &'static str = "#00AA00";

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
        let (w, h) = (80, 60);
        let mut maze = MazeBuilder::new(w, h);
        let canvas = canvas();
        canvas.set_attribute("width", format!("{}", (CELL_SIZE + 1) * (2 * w - 1) + 2).as_str())?;
        canvas.set_attribute("height", format!("{}", (CELL_SIZE + 1) * (2 * h - 1) + 2).as_str())?;
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            draw_maze(&canvas, &maze);
            maze.step();
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
    }

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

// set of cells in maze - just the start
// set of cells not in maze
// pick a random cell not in the maze
// walk randomly until hitting the maze
// if hit path, backtrack

type Point = (i64, i64);
type Edge = (Point, Point);
const DIRS: [Point; 4] = [
    (-1, 0), (1, 0), (0, -1), (0, 1),
];

struct MazeBuilder {
    untouched: HashSet<Point>,
    path_cells: HashSet<Point>,
    edges: HashSet<Edge>,
    path_lens: Vec<usize>,
    width: u32,
    height: u32,
}

impl MazeBuilder {
    fn new(width: u32, height: u32) -> Self {
        let untouched = (0..width)
            .map(|x| {
                (0..height)
                    .filter_map(move |y| {
                        if x != 0 || y != 0 {
                            Some((x as i64, y as i64))
                        } else {
                            None
                        }
                    })
            })
            .flatten()
            .collect::<HashSet<Point>>();
        let start = (0_i64, 0_i64);
        let path_cells = vec![start].into_iter().collect::<HashSet<Point>>();
        Self {
            untouched,
            path_cells,
            edges: HashSet::new(),
            path_lens: vec![],
            width,
            height,
        }

    }

    fn step(&mut self) {
        // remove element from untouched
        let first = match self.untouched.clone().into_iter().next() {
            Some(f) => f,
            None => {
                return;
            },
        };
        self.untouched.remove(&first);

        let mut stack = vec![vec![first]];
        let mut visited = HashSet::<Point>::new();
        while let Some(curr_path) = stack.pop() {

            let curr = curr_path[curr_path.len()-1];

            if self.path_cells.contains(&curr) {
                for i in 0..(curr_path.len()-1) {
                    let j = i + 1;
                    self.edges.insert((curr_path[i], curr_path[j]));
                    self.edges.insert((curr_path[j], curr_path[i]));
                }
                self.path_lens.push(curr_path.len());
                curr_path.iter().for_each(|p| {
                    self.untouched.remove(p);
                    self.path_cells.insert(*p);
                });
                return;
            }

            if visited.contains(&curr) {
                continue;
            }
            visited.insert(curr);

            let nbors = self.get_nbors(&curr);
            let curr_path_nbors = nbors.iter().fold(0, |acc, nbor| {
                if curr_path.contains(nbor) {
                    acc + 1
                } else {
                    acc
                }
            });

            if curr_path_nbors > 1 {
                continue;
            }

            nbors.iter()
                .for_each(|nbor| {
                    if !visited.contains(nbor) {
                        let mut new_path = curr_path.clone();
                        new_path.push(*nbor);
                        stack.push(new_path);
                    }
                });
        }
    }

    fn get_nbors(&self, curr: &Point) -> Vec<Point> {
        let mut nbors = DIRS
            .iter()
            .filter_map(|dir| {
                let nx = curr.0 + dir.0;
                let ny = curr.1 + dir.1;
                if nx >= 0 && nx < self.width as i64 && ny >= 0 && ny < self.height as i64 {
                    Some((nx, ny))
                } else {
                    None
                }
            })
            .collect::<Vec<Point>>();
        // TODO: This should really use a seed
        use rand::seq::SliceRandom;
        nbors.shuffle(&mut rand::rng());
        nbors
    }
}


fn draw_maze(canvas: &HtmlCanvasElement, maze: &MazeBuilder) {
    let ctx = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    ctx.begin_path();

    ctx.set_fill_style_str(PATH_COLOR);
    for ((x1, y1), (x2, y2)) in maze.edges.iter() {
        let x = std::cmp::min(x1, x2);
        let y = std::cmp::min(y1, y2);
        let w = if x1 == x2 {
            CELL_SIZE
        } else {
            (CELL_SIZE * 3) + 2
        };
        let h = if y1 == y2 {
            CELL_SIZE
        } else {
            (CELL_SIZE * 3) + 2
        };
        ctx.fill_rect(
            2_f64 * (*x as u32 * (CELL_SIZE + 1) + 1) as f64,
            2_f64 * (*y as u32 * (CELL_SIZE + 1) + 1) as f64,
            w as f64, h as f64
        );
    }
}

