use std::collections::HashMap;
use std::io::{stdout, Error, Write};
use std::{thread, time};

use rand::{thread_rng, Rng};
use termion::color;

const SIZE: i32 = 32;

#[derive(Clone)]
pub struct Micron {
    pub color: usize,
}

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn move_simple(&self, dir: &Direction, num: i32) -> Self {
        match dir {
            Direction::Up => Self {
                x: self.x,
                y: self.y - num,
            },
            Direction::Ri => Self {
                x: self.x + num,
                y: self.y,
            },
            Direction::Le => Self {
                x: self.x - num,
                y: self.y,
            },
            Direction::Do => Self {
                x: self.x,
                y: self.y + num,
            },
        }
    }
}

pub enum Direction {
    Up,
    Ri,
    Le,
    Do,
}

impl Direction {
    pub fn all() -> impl Iterator<Item = Direction> {
        [Direction::Up, Direction::Ri, Direction::Le, Direction::Do].into_iter()
    }
}

fn next(world: &mut World) {
    let dirs: Vec<_> = Direction::all().collect();
    let mut rng = thread_rng();

    let moves: Vec<_> = world
        .objects
        .keys()
        .map(|point| {
            let dir = choose(&dirs, &mut rng);
            let next_point = point.move_simple(dir, 1);
            (point.clone(), next_point)
        })
        .collect();
    for (cur, next) in moves.iter() {
        let micron = world.objects.remove(cur);
        if moves.iter().filter(|(_, point)| point == next).count() == 1 && world.valid(next) {
            world.objects.insert(next.clone(), micron.unwrap());
        }
    }
    for color in [0,1] {
        let (point, micron) = pick(&mut world.objects.iter().filter(|(_,p)| p.color == color).collect(), &mut rng);
        let dir = choose(&dirs, &mut rng);
        let new_point = point.move_simple(dir, 1);
        if world.valid(&new_point) {
            let micron = micron.clone();
            world.objects.insert(new_point, micron);
        }
    }
    
}

fn choose<'a, T, R: Rng>(vec: &'a Vec<T>, rng: &mut R) -> &'a T {
    vec.get(rng.gen_range(0..vec.len())).expect("NEVER")
}

fn pick<T, R: Rng>(vec: &mut Vec<T>, rng: &mut R) -> T {
    let index = rng.gen_range(0..vec.len());
    vec.remove(index)
}
pub struct World {
    pub objects: HashMap<Point, Micron>,
}

impl World {
    pub fn valid(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < SIZE && point.y >= 0 && point.y < SIZE
    }
}

fn visualize(mic: &Micron) -> String {
    match mic.color {
        0 => format!("{}●{} ", color::Fg(color::Red), color::Fg(color::Reset)),
        1 => format!("{}●{} ", color::Fg(color::Blue), color::Fg(color::Reset)),
        2 => format!(
            "{}●{} ",
            color::Fg(color::LightGreen),
            color::Fg(color::Reset)
        ),
        _ => panic!(),
    }
}

fn to_string(world: &World) -> String {
    (0..SIZE)
        .map(|y| {
            (0..SIZE)
                .map(|x| {
                    let found = world.objects.get(&Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    });
                    found
                        .and_then(|mic| Some(visualize(mic)))
                        .unwrap_or_else(|| "  ".to_string())
                })
                .chain(["\n".to_string()])
                .collect::<String>()
        })
        .collect()
}

fn main() -> Result<(), Error> {
    let mut world = World {
        objects: [
            (Point { x: 10, y: 10 }, Micron { color: 0 }),
            (Point { x: 22, y: 22 }, Micron { color: 1 }),
        ]
        .into(),
    };
    let mut stdout = stdout();

    for _ in 0..100 {
        thread::sleep(time::Duration::from_millis(1000));
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            to_string(&world)
        )?;
        next(&mut world);
    }
    Ok(())
}
