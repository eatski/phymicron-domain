use std::collections::HashMap;
use std::io::{stdout, Error, Write};
use std::{thread, time};

use rand::{thread_rng, Rng};

const SIZE: usize = 32;

#[derive(Clone)]
pub struct Micron;

pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn move_simple(&self, dir: Direction, num: usize) -> Self {
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
    let dirs : Vec<_> = Direction::all().collect();
    let mut rng = thread_rng();
    let object = choose(&world.objects,&mut rng);
    
    let objects = world.objects.iter().map(|(micron, point)| {
        let dir = dirs.get(rng.gen_range(0..dirs.len())).unwrap();
    });
    todo!()
} 

fn choose<'a,T,R: Rng>(vec: &'a Vec<T>,rng: &mut R) -> &'a T {
    vec.get(rng.gen_range(0..vec.len())).expect("NEVER")
}

fn choose_mut<'a,T,R: Rng>(vec: &'a mut Vec<T>,rng: &mut R) -> &'a mut T {
    let len = vec.len();
    vec.get_mut(rng.gen_range(0..len)).expect("NEVER")
}


pub struct World {
    pub objects: Vec<(Micron, Point)>,
}

fn main() -> Result<(), Error> {
    let mut stdout = stdout();
    for i in 0..100 {
        thread::sleep(time::Duration::from_millis(100));
        write!(
            stdout,
            "{}{}res: {}\n{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            i,
            " ".repeat(i) + "●"
        )?;
        stdout.flush()?;
    }
    Ok(())
}
