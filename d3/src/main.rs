fn number_of_steps(i: i32) -> i32 {
        let mut n = i-1;
        let r = if n == 0 {
            0
        }
        else {
            let mut p = (n as f64).sqrt() as i32;
            if p % 2 == 0 && p > 0 {
                p -= 1;
            }
            
            n = (n-p*p) % (p+1);
            let q = p/2;

            1 + q + (n-q).abs()
        };
        return r;
    }

const N: i32 = 13; // memory square dimensions
const P: i32 = (N-1)/2; // position of the center of the square

macro_rules! v { // a macro to find the index of a element at (x, y) if items are stored in row order
    [$v:expr, $x:expr, $y:expr] => {$v[(N*$x+$y) as usize]}
}

// return the sum of the surrounding items assuming that the item at the center is 0
fn fill_memory(data: &mut Vec<i32>, x: i32, y: i32) -> i32 {   
    assert_eq!(v![data, x, y], 0);
    let mut s = 0;
    for i in 0..3 {
        for j in 0..3 {
            s += v![data, x+i-1, y+j-1]
        }
    }
    v![data, x, y] = s;
    return s;
}

// traverse the square in a spiral
fn traverse_memory(input: i32) -> Option<i32> {
    let mut data = vec![0; (N*N) as usize];
    v![data, P, P] = 1;
    let mut x: i32 = P; // the starting location
    let mut y: i32 = P;
    let mut dx = 0; // the initial movement direction along the edge
    let mut dy = 1;

    let mut side = 2; // current number of steps along a side
    for _i in 0..(P-1) { // because we keep a margin of 1 row/column otherwise the fill_memory function would read out of bounds
        for sd in 0..4 { // 4 is the number of sides in the square
            for j in 0..side { // for each side we are to do 'side' steps before changing direction
                if sd == 0 && j == 0 { // the first step is always (+1, 0), this is how we step into the new 'layer'
                    x += 1;
                }
                else {
                    x += dx;
                    y += dy;
                }
                // println!("{} {}", x, y);
                let s = fill_memory(&mut data, x, y);
                if s > input {
                    return Some(s);
                }
            }
            dy = -dy; // change walking direction
            std::mem::swap(&mut dx, &mut dy);
        }
        side += 2; // the length of the side of the spiral increases by 2
    }
    return None;
}

fn main() {
    println!("{}", number_of_steps(289326)); // 419
    println!("{}", traverse_memory(289326).unwrap()); // 295229
}
