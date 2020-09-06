struct Player {
    symbol: char,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            symbol: '@',
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
}

fn main() {
    println!("foo foo");

    /*
    let xs: &[i32] = &[1, 2, 3];
    print!("F");
    println!("{:?}", xs);
    */
    /*
    let _player: *mut Player;  // = &Player::default();
    _player = &mut Player::default();
    */

    const WIDTH: usize = 80;
    const HEIGHT: usize = 30;
    let mut _map = Map::new(WIDTH, HEIGHT);
    // let mut row: [Cell; WIDTH] = [Cell::default(); WIDTH];

    // row[37].large_object = _player;

    // println!("{:#?}", row);
    print!("\n");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let element = _map.get(x, y);
            if element.large_object.is_null() {
                print!("{}", element.floor);
            } else {
                unsafe {
                    print!("{}", (*element.large_object).symbol);
                }
            }
        }
        print!("\n");
    }
    print!("\n");
}
