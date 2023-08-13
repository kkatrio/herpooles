use crate::geometry;
use crate::render;
use crate::PressedKeys;
use rand;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Herpooles {
    pub x: f32, // pub needed to render
    pub y: f32,
    dead: bool,
    poo: Vec<Poo>,
    pub bearing: Direction, // for render
}

impl Herpooles {
    pub fn new() -> Herpooles {
        Herpooles {
            x: 500.0,
            y: 500.0,
            dead: true,
            poo: vec![],
            bearing: Direction::North,
        }
    }

    pub fn fire_poo(&mut self) {
        // TODO: clean some poo from the vec
        self.poo.push(Poo::new(&self.x, &self.y, self.bearing));
    }

    pub fn is_alive(&self) -> bool {
        !self.dead
    }

    fn color(&self) -> &str {
        match self.dead {
            true => "red",
            false => "green",
        }
    }
}

pub struct Zombie {
    pub x: f32,
    pub y: f32,
    walking: bool,
}

impl Zombie {
    pub fn new() -> Zombie {
        let xr = rand::random::<f32>();
        //let yr = rand::random::<f32>();
        let x_variance = 1000.0;
        // let y_variance = 10.0;
        let x_start = 0.0;
        let y_start = 200.0;
        Zombie {
            x: x_start + xr * x_variance,
            y: y_start,
            walking: true,
        }
    }

    // no effect since we clean dirty zombies in the same frame
    fn color(&self) -> &str {
        match self.walking {
            true => "grey",
            false => "yellow",
        }
    }
}

#[derive(Copy, Clone)]
pub struct Poo {
    pub x: f32,
    pub y: f32,
    direction: Direction,
    must_clean: bool,
}

impl Poo {
    pub fn new(x: &f32, y: &f32, direction: Direction) -> Poo {
        Poo {
            x: *x,
            y: *y,
            direction: direction,
            must_clean: false,
        }
    }
}

//#[derive(PartialEq)]
//enum HState {
//    Alive,
//    Dead,
//}
//
//// or maybe use associated constants or constans in another mod:
//// https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust
//impl HState {
//    fn color(&self) -> &str {
//        match *self {
//            HState::Dead => "red",
//            HState::Alive => "green",
//        }
//    }
//}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

fn zombies_reached(h: &Herpooles, z: &Zombie) -> bool {
    let d = (h.x - z.x) * (h.x - z.x) + (h.y - z.y) * (h.y - z.y);
    //log!("d: {}", d);
    if d < 400.0 {
        log!("herpooles dead!");
    }
    d < 400.0 // TODO: calculate based on herpooles and zombie area
}

fn hit_zombie(p: &Poo, z: &Zombie) -> bool {
    if p.must_clean {
        false
    } else {
        let d = (p.x - z.x) * (p.x - z.x) + (p.y - z.y) * (p.y - z.y);
        d < 400.0 // TODO: calculate based on area
    }
}

fn move_zombie(z: &mut Zombie, h: &Herpooles) {
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
    let poo_speed = 0.6;
    let mv_vec = direction_vec.unit_vec() * poo_speed;
    p.x = p.x + mv_vec.x;
    p.y = p.y + mv_vec.y;
}

pub fn move_herpooles(herpooles: &mut Herpooles, pressed_keys: &PressedKeys) {
    if pressed_keys.right && herpooles.x < 1000.0 {
        herpooles.bearing = Direction::East;
        herpooles.x += 2.0;
    }
    if pressed_keys.left && herpooles.x > 0.0 {
        herpooles.bearing = Direction::West;
        herpooles.x -= 2.0;
    }
    if pressed_keys.up && herpooles.y > 0.0 {
        herpooles.bearing = Direction::North;
        herpooles.y -= 2.0;
    }
    if pressed_keys.down && herpooles.y < 800.0 {
        herpooles.bearing = Direction::South;
        herpooles.y += 2.0;
    }
}

// h is a Rc clone
pub fn step(
    ctx: &web_sys::CanvasRenderingContext2d,
    h: &Rc<RefCell<Herpooles>>,
    zombies: &mut Vec<Zombie>,
    pressed_keys: &Rc<Cell<PressedKeys>>,
    zombie_kill_sound: &web_sys::HtmlAudioElement,
) {
    ctx.clear_rect(1.0, 1.0, 998.0, 798.0); //TODO: parameterize this

    let mut h_ref = h.borrow_mut();
    let pressed_keys = pressed_keys.get();

    // move herpooles
    move_herpooles(&mut h_ref, &pressed_keys);
    // move zombies
    zombies.iter_mut().for_each(|z| {
        move_zombie(z, &h_ref);
    });

    // An empty iterator returns false.
    if !zombies.is_empty() {
        h_ref.dead = zombies.iter().any(|z| zombies_reached(&h_ref, &z));
    }

    // draw zombies and herpooles
    render::draw_herpooles(ctx, &h_ref, &h_ref.color());
    zombies.iter().for_each(|z| {
        render::draw_zombie(ctx, z, z.color());
    });

    // move and draw poo
    h_ref.poo.iter_mut().for_each(|p| {
        move_poo(p);
        render::draw_poo(ctx, p);
    });

    // check collision and mark for cleaning
    // zombies is a &mut
    // https://stackoverflow.com/questions/68936034/mutable-reference-to-a-vector-was-moved-due-to-this-implicit-call-to-into-iter
    for z in zombies.iter_mut() {
        for p in &mut h_ref.poo {
            if hit_zombie(&p, &z) {
                p.must_clean = true;
                z.walking = false;
                let _promise = zombie_kill_sound.play().unwrap();
            }
        }
    }

    // clean poo
    // retain removes when predicate is false
    h_ref.poo.retain(|&p| !p.must_clean);

    // clean dirty zombies
    zombies.retain(|z| z.walking);
    if zombies.is_empty() {
        log!("no zombies");
    }
}

pub fn set(zombies: &mut Vec<Zombie>) {
    if zombies.len() < 10 {
        zombies.push(Zombie::new());
    }
}
