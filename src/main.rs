use std::collections::HashMap;
use std::io::{repeat, stdout, Error, Write};
use std::{thread, time};

use rand::{thread_rng, Rng};

const SIZE: usize = 32;

#[derive(Clone)]
pub struct Micron;

#[derive(Eq, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn move_simple(&self, dir: &Direction, num: usize) -> Self {
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
    let (micron, point) = pick(&mut world.objects, &mut rng);
    let dir = choose(&dirs, &mut rng);
    let new_one = (micron.clone(), point.move_simple(dir, 1));
    for (_, point) in world.objects.iter_mut() {
        let dir = choose(&dirs, &mut rng);
        *point = point.move_simple(dir, 1);
    }
    world.objects.push(new_one);
}

fn choose<'a, T, R: Rng>(vec: &'a Vec<T>, rng: &mut R) -> &'a T {
    vec.get(rng.gen_range(0..vec.len())).expect("NEVER")
}

fn pick<T, R: Rng>(vec: &mut Vec<T>, rng: &mut R) -> T {
    let index = rng.gen_range(0..vec.len());
    vec.remove(index)
}

fn choose_mut<'a, T, R: Rng>(vec: &'a mut Vec<T>, rng: &mut R) -> &'a mut T {
    let len = vec.len();
    vec.get_mut(rng.gen_range(0..len)).expect("NEVER")
}

pub struct World {
    pub objects: Vec<(Micron, Point)>,
}

fn main() -> Result<(), Error> {
    let mut world = World {
        objects: vec![(Micron, Point { x: 15, y: 15 })],
    };
    let mut stdout = stdout();

    for i in 0..100 {
        thread::sleep(time::Duration::from_millis(100));
        let str: String = (0..SIZE).map(|y| {
            (0..SIZE).map(|x| {
                let found = world
                    .objects
                    .iter()
                    .find(|(_, point)| point == &Point { x, y });
                found.and_then(|_| Some("●")).unwrap_or_else(|| " ")
            }).chain(["\n"]) .collect::<String>()
        }).collect();
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            str
        )?;
        stdout.flush()?;
        next(&mut world);
    }
    Ok(())
}
