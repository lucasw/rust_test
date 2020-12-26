/*
 * Sum up every element in a grid for the rectangle from 0,0 to the current grid location
 */

use rand::Rng;

fn print(grid: &Vec<usize>, wd: usize, ht: usize) {
    println!("{} ", grid.len()); 
    for y in 0..ht {
        for x in 0..wd {
            let ind = y * wd + x;
            print!("{:04} ", grid[ind]);
        }
        println!("");
    }
}

fn main() {
    let wd : usize = 24;
    let ht : usize = 16;
    let mut grid = vec![0; wd * ht];
    let mut test_sum = 0;
    for cell in grid.iter_mut() {
        let val = rand::thread_rng().gen_range(0, 10);
        test_sum += val;
        *cell = val;
    }
    print(&grid, wd, ht);

    let mut sums = vec![0; wd * ht];

    for y in 0..ht {
        for x in 0..wd {
            let ind = y * wd + x;
            sums[ind] = grid[ind];
            if x > 0 {
                sums[ind] += sums[ind - 1];
            }
            if y > 0 {
                sums[ind] += sums[ind - wd];
            }
            if x > 0 && y > 0 {
                sums[ind] -= sums[ind - wd - 1];
            }
        }
    }

    print(&sums, wd, ht);

    println!("test sum {}", test_sum);
}
