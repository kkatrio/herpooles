use crate::geometry;
use crate::render;
use crate::PressedKeys;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct Herpooles {
    pub x: f32,
    pub y: f32,
    pub alive: bool,
}

impl Herpooles {
    pub fn new() -> Herpooles {
        Herpooles {
            x: 500.0,
            y: 500.0,
            alive: true,
        }
    }

    pub fn fire_poo(&self) {}
}

pub struct Zombie {
    pub x: f32,
    pub y: f32,
}

impl Zombie {
    pub fn new() -> Zombie {
        Zombie { x: 500.0, y: 400.0 }
    }
}

pub struct Poo {
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
}

impl Poo {
    pub fn new() -> Poo {
        Poo {
            x: 100.0,
            y: 100.0,
            direction: Direction::South,
        }
    }
}

#[derive(PartialEq)]
enum HState {
    Alive,
    Dead,
}

// or maybe use associated constants or constans in another mod:
// https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust
impl HState {
    fn color(&self) -> &str {
        match *self {
            HState::Dead => "red",
            HState::Alive => "green",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

fn is_herpooles_alive(h: &Herpooles, z: &Zombie) -> bool {
    let d = (h.x - z.x) * (h.x - z.x) + (h.y - z.y) * (h.y - z.y);
    //log!("d: {}", d);
    d > 400.0 // TODO: calculate based on herpooles and zombie area
}

fn move_zombies(z: &mut Zombie, h: &Herpooles) {
    // find vector z -> h
    let zp = geometry::Point { x: z.x, y: z.y };
    let hp = geometry::Point { x: h.x, y: h.y };
    let zh_vec = geometry::Vector::new(zp, hp);

    // apply A + d n
    let zombie_speed = 0.5;
    let mv_vec: geometry::Vector = zh_vec.unit_vec() * zombie_speed;
    let pos: geometry::Point = zp + mv_vec;
    z.x = pos.x;
    z.y = pos.y;
}

fn move_poo(p: &mut Poo) {
    // find unit vector of orbit
    // TODO: maybe store the unit vector in the poo instead of the direction.
    let (p_next_x, p_next_y) = match p.direction {
        Direction::North => (p.x, p.y - 1.0),
        Direction::East => (p.x + 1.0, p.y),
        Direction::South => (p.x, p.y + 1.0),
        Direction::West => (p.x - 1.0, p.y),
    };
    let direction_vec = geometry::Vector::new(
        geometry::Point { x: p.x, y: p.y },
        geometry::Point {
            x: p_next_x,
            y: p_next_y,
        },
    );
    let poo_speed = 0.5;
    let mv_vec = direction_vec.unit_vec() * poo_speed;
    p.x = p.x + mv_vec.x;
    p.y = p.y + mv_vec.y;
}

// h is a Rc clone
pub fn step(
    ctx: &web_sys::CanvasRenderingContext2d,
    h: &Rc<RefCell<Herpooles>>,
    z: &mut Zombie,
    p: &mut Poo,
    pressed_keys: &Rc<Cell<PressedKeys>>,
) {
    ctx.clear_rect(1.0, 1.0, 998.0, 798.0);

    // herpooles
    let mut hcv = h.borrow_mut();
    let pressed_keys = pressed_keys.get();
    move_herpooles(&mut hcv, &pressed_keys);
    //h.set(hcv); // set the value back to the cell, so that it is updated for the next step
    let herpooles_state = if is_herpooles_alive(&hcv, z) {
        HState::Alive
    } else {
        HState::Dead
    };
    if herpooles_state == HState::Dead {
        log!("herpooles state dead!");
        //let mut in_h = h.take();
        hcv.alive = false;
        //h.set(in_h);
    }
    render::draw_herpooles(ctx, &hcv, herpooles_state.color());

    // poo
    move_poo(p);
    render::draw_poo(ctx, p);

    // zombies
    move_zombies(z, &hcv);
    render::draw_zombie(ctx, &z, "grey");
}

pub fn move_herpooles(herpooles: &mut Herpooles, pressed_keys: &PressedKeys) {
    if pressed_keys.right && herpooles.x < 1000.0 {
        herpooles.x += 10.0;
    }
    if pressed_keys.left && herpooles.x > 0.0 {
        herpooles.x -= 10.0;
    }
    if pressed_keys.up && herpooles.y > 0.0 {
        herpooles.y -= 10.0;
    }
    if pressed_keys.down && herpooles.y < 800.0 {
        herpooles.y += 10.0;
    }
}
