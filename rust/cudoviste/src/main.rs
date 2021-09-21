mod util;

type Grid = Vec<Vec<char>>;

fn get_grid(r: u8, c: u8) -> Grid {
    
    let mut grid = Vec::with_capacity(r as usize);
    for _ in 0..r {
        let mut row = Vec::with_capacity(c as usize);
        let line = util::input();
        let line = line.trim();
        for ch in line.chars() {
            row.push(ch);
        }
        grid.push(row);
    }
    grid

}

fn main() {
    
    let dimensions = util::parse_line_of_nums::<u8>();

    let r = dimensions[0];
    let c = dimensions[1];

    let grid = get_grid(r, c);

    let mut car_squashes = vec![0; 5];

    for i in 0..(r-1) {
        for j in 0..(c-1) {
            let square_one = grid[i as usize][j as usize];
            let square_two = grid[(i+1) as usize][j as usize];
            let square_three = grid[i as usize][(j+1) as usize];
            let square_four = grid[(i+1) as usize][(j+1) as usize];

            let squares: Vec<char> = vec![square_one, square_two, square_three, square_four];

            let on_building = squares.iter().any(|ch| *ch == '#');

            if on_building { continue; }

            let cars = squares.iter().filter(|ch| **ch == 'X').count();

            car_squashes[0] += if cars == 0 { 1 } else { 0 };
            car_squashes[1] += if cars == 1 { 1 } else { 0 };
            car_squashes[2] += if cars == 2 { 1 } else { 0 };
            car_squashes[3] += if cars == 3 { 1 } else { 0 };
            car_squashes[4] += if cars == 4 { 1 } else { 0 };

        }
    }

    for i in car_squashes {
        println!("{}", i);
    }
}
