use std::i32;

use bracket_lib::{prelude::*, random};

// Window Size 
const WIDTH: usize = 150; 
const HEIGHT: usize = 100;

struct State {
    map : [[bool; HEIGHT] ; WIDTH],
}

/// print the map in the terminal
fn print_map(map : [[bool; HEIGHT] ; WIDTH], ctx : &mut BTerm){
    for x in 0..WIDTH{
        for y in 0..HEIGHT{
            if map[x][y]{
                ctx.set(x, y, (0, 0, 0), (0, 0, 255), 0);
            }
            else{
                ctx.set(x, y, (0, 0, 0), (255, 0, 0), 0);
            }
        }
    }
}  

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        print_map(self.map, ctx);
    }
}

/// Return 1 if frue
/// Return 0 if false
fn bool_to_int(b: bool)-> i32{
    if b{
        return 1;
    }
    return 0;
}

/// Count the number of neighbours with the value true of the point (x, y) and return true if 
/// there is more than comparaison_level neighbours with this value
fn enough_neighbours(x: usize, y: usize, map : [[bool; HEIGHT] ; WIDTH], comparaison_level : i32) -> bool{
    let mut count_true_neighbours = 0;
    
    if x != 0 {
        if y != 0 {
            count_true_neighbours += bool_to_int(map[x-1][y-1]);
        }

        count_true_neighbours += bool_to_int(map[x-1][y]);

        if y != HEIGHT-1{
            count_true_neighbours += bool_to_int(map[x-1][y+1]);
        }
    }

    if y != 0 {
        count_true_neighbours += bool_to_int(map[x][y-1]);
    }

    if y != HEIGHT-1{
        count_true_neighbours += bool_to_int(map[x][y+1]);
    }

    if x != WIDTH-1{
        if y != 0 {
            count_true_neighbours += bool_to_int(map[x+1][y-1]);
        }

        count_true_neighbours += bool_to_int(map[x+1][y]);

        if y != HEIGHT-1{
            count_true_neighbours += bool_to_int(map[x+1][y+1]);
        }
    }

    return count_true_neighbours >= comparaison_level;
}

/// Generate a tab that represent the map of our game with randomize generation 
/// and cellular automata algorithm to make a map more realistic
fn generate_map()-> [[bool; HEIGHT] ; WIDTH] {
    let mut seed = random::RandomNumberGenerator::new();
    let mut map_rand = [[false; HEIGHT]; WIDTH];

    // generate our first random map 
    for x in 0..WIDTH{
        for y in 0..HEIGHT{
            map_rand[x][y] = seed.rand::<i32>() % 2 == 0; 
        }
    }

    //We increase the realism of our generation with a check 
    //of the neighbours of a pixel to determine if their is enough neighbours 
    //with the state true around to stay in this state
    let mut map = [[false; HEIGHT]; WIDTH];
    for x in 0..WIDTH{
        for y in 0..HEIGHT{
            map[x][y] = enough_neighbours(x, y, map_rand, 5); //true if their are enough neighbours that are true
        }
    }

    // And to increase the effect of our generation, 
    // we repeat the last operation 
    for x in 0..WIDTH{
        for y in 0..HEIGHT{
            map[x][y] = enough_neighbours(x, y, map, 4);
        }
    }

    return map;
}

fn main() -> BError {
    let context = BTermBuilder::simple(WIDTH, HEIGHT).unwrap()
        .with_title("First Map")
        .build()?;

    let gs: State = State {map: generate_map()};
    main_loop(context, gs)
}