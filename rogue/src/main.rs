/*
 * Copyright (c) 2020 by Lucas Walter
 * September 2020
 * Learn rust with a weakly roguelike
 *
 */

use device_query::{DeviceQuery, Keycode};

struct Player {
    symbol: char,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            // symbol: '@',
            symbol: '♘',
        }
    }
}

#[derive(Copy, Clone)]
struct Cell {
    floor: char,
    large_object: *mut Player,
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            floor: '.',
            large_object: std::ptr::null_mut(),
        }
    }
}

impl Cell {
    fn print(&self) {
        if self.large_object.is_null() {
            print!("{}", self.floor);
        } else {
            unsafe {
                print!("{}", (*self.large_object).symbol);
            }
        }
    }
}

struct Map {
    _width: usize,
    _height: usize,
    cells: Vec<Cell>,
    // Box<[Cell]>
}

impl Map {
    // fn default(width: usize, height: usize) -> Map {
    fn new(width: usize, height: usize) -> Map {
        Map {
            _width: width,
            _height: height,
            cells: vec![Cell::default(); width * height],
        }
    }

    fn get(&self, x: usize, y: usize) -> Cell {
        return self.cells[y * self._width + x];
    }

    fn put(&mut self, _player: *mut Player, x: usize, y: usize) {
        self.cells[y * self._width + x].large_object = _player;
    }

    fn move_object(&mut self, old_x: usize, old_y: usize, new_x: usize, new_y: usize) {
        if old_x == new_x && old_y == new_y {
            return;
        }
        let old_ind : usize = old_y * self._width + old_x;
        let new_ind : usize = new_y * self._width + new_x;
        self.cells[new_ind].large_object = self.cells[old_ind].large_object;
        self.cells[old_ind].large_object = std::ptr::null_mut();
    }

    fn print(&self) {
        print!("\n");
        for y in 0..self._height {
            for x in 0..self._width {
                // let element = self.get(x, y);
                self.get(x, y).print();
            }
            print!("\n");
        }
        print!("\n");
    }
}

fn main() {
    println!("foo foo");

    /*
    let xs: &[i32] = &[1, 2, 3];
    print!("F");
    println!("{:?}", xs);
    */
    let _player: *mut Player;
    _player = &mut Player::default();

    const WIDTH: usize = 80;
    const HEIGHT: usize = 30;
    let mut _map = Map::new(WIDTH, HEIGHT);

    let mut px : usize = 40;
    let mut py : usize = 25;
    _map.put(_player, px, py);

    let device_state = device_query::DeviceState::new();

    let mut count: i32 = 0;
    loop {
        // Clear the screen, move to 0,0  "\033[2J"
        // Move up N lines "\033[<N>A"
        println!("\x1b[2J");
        println!("{}", count);
        _map.print();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let keys: Vec<Keycode> = device_state.get_keys();

        let old_px = px;
        let old_py = py;
        if keys.contains(&Keycode::H) {
            if px > 0 {
                px -= 1;
            }
        }
        if keys.contains(&Keycode::L) {
            if px < _map._width - 1 {
                px += 1;
            }
        }
        if keys.contains(&Keycode::K) {
            if py > 0 {
                py -= 1;
            }
        }
        if keys.contains(&Keycode::J) {
            if py < _map._height - 1 {
                py += 1;
            }
        }
        println!("");
        _map.move_object(old_px, old_py, px, py);
        count += 1;
    }
    // println!("{:#?}", row);
}
