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

fn main() {
    println!("foo foo");

    let xs: &[i32] = &[1, 2, 3];

    let _player: *mut Player;  // = &Player::default();
    _player = &mut Player::default();

    print!("F");
    println!("{:?}", xs);

    const WIDTH: usize = 80;
    const HEIGHT: usize = 30;
    let mut row: [Cell; WIDTH] = [Cell::default(); WIDTH];

    row[37].large_object = _player;

    // println!("{:#?}", row);
    print!("\n");
    for _ in 0..HEIGHT {
        for element in row.iter() {
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
