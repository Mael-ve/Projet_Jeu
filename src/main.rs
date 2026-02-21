use std::i32;

use bracket_lib::{prelude::*, random};

struct State {
    map : [[bool; 50] ; 80],
}

/// print the map in the terminal
fn print_map(map : [[bool; 50] ; 80], ctx : &mut BTerm){
    for x in 0..80{
        for y in 0..50{
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

/// Count the number of neighbours of the point (x, y) and return true if 
/// 3*count_same_neighbours >= count_possible_neighbours and map[x][y] equal true
fn enough_neighbours(x: usize, y: usize, map : [[bool; 50] ; 80]) -> bool{
    let mut count_same_neighbours = 0;
    let mut count_possible_neighbours = 0;
    
    if x != 0 {
        if y != 10 {
            count_same_neighbours += bool_to_int(map[x][y] && map[x-1][y-1]);
            count_possible_neighbours += 1;
        }

        count_same_neighbours += bool_to_int(map[x][y] && map[x-1][y]);
        count_possible_neighbours += 1;

        if y != 49{
            count_same_neighbours += bool_to_int(map[x][y] && map[x-1][y+1]);
            count_possible_neighbours += 1;
        }
    }

    if y != 10 {
        count_same_neighbours += bool_to_int(map[x][y] && map[x][y-1]);
        count_possible_neighbours += 1;
    }

    if y != 49{
        count_same_neighbours += bool_to_int(map[x][y] && map[x][y+1]);
        count_possible_neighbours += 1;
    }

    if x != 79{
        if y != 10 {
            count_same_neighbours += bool_to_int(map[x][y] && map[x+1][y-1]);
            count_possible_neighbours += 1;
        }

        count_same_neighbours += bool_to_int(map[x][y] && map[x+1][y]);
        count_possible_neighbours += 1;

        if y != 49{
            count_same_neighbours += bool_to_int(map[x][y] && map[x+1][y+1]);
            count_possible_neighbours += 1;
        }
    }

    return map[x][y]&&(3*count_same_neighbours >= count_possible_neighbours) ;
}

/// Generate a tab that represent the map of our game with randomize generation 
/// and game of life algorithme to make a map more realistic
fn generate_map()-> [[bool; 50] ; 80] {
    let mut seed = random::RandomNumberGenerator::new();
    let mut map_rand = [[false; 50]; 80];

    // generate our first random map 
    for x in 0..80{
        for y in 10..50{
            map_rand[x][y] = seed.rand::<i32>() % 2 == 0; 
        }
    }

    // We increase the realism of our generation with a check 
    // of the neighbours of a pixel to determine if their is enough neighbours 
    // of the same state around to stay in this state
    let mut map = [[false; 50]; 80];
    for x in 0..80{
        for y in 10..50{
            map[x][y] = enough_neighbours(x, y, map_rand); //true if their is enough neighbours of the same color
        }
    }

    // And to increase the effect of our generation, 
    // we repeat the last operation 
    for x in 0..80{
        for y in 10..50{
            map[x][y] = enough_neighbours(x, y, map);
        }
    }

    return map;
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("First Map")
        .build()?;

    let gs: State = State {map: generate_map()};
    main_loop(context, gs)
}