use std::env;
use std::time::Duration;
use std::thread;
use std::fs;

struct GameState {
    current_state: Vec<Vec<char>>,
    tmp_state: Vec<Vec<char>>,

    iter: u32,
}

impl GameState {

    fn new(x: usize, y: usize, init: Vec<&str>) -> GameState {
        let mut state = GameState {
            current_state : vec![vec!['X'; x]; y], 
            tmp_state : vec![vec!['X'; x]; y], 
            iter : 0
        };

        let mut c = 0;
        for s in init {
            state.current_state[c] = s.chars().collect();
            state.tmp_state[c] = s.chars().collect();
            c += 1;
        }

        return state;
    }

    fn step(&mut self) {
        self.copy_state();

        for x in 0..self.current_state.len() {
            for y in 0..self.current_state[x].len() {
                let neighbors = self.count_neighbours(x, y);
                //println!("x{}y{}n{}",x,y,neighbors);
                match neighbors {
                    2 => {

                    },
                    3 => {
                        self.tmp_state[x][y] = 'X';
                    },
                    _ => {
                        self.tmp_state[x][y] = ' ';
                    }
                }
            }
        }

        std::mem::swap(&mut self.current_state, &mut self.tmp_state);
        self.iter += 1;
    }

    fn count_neighbours(&self, x: usize, y: usize) -> u32 {
        let prev_x = if x == 0 { self.current_state.len() - 1 } else { x - 1 };
        let next_x = if x == self.current_state.len() - 1 { 0 } else { x + 1 };
        
        let prev_y = if y == 0 { self.current_state[x].len() - 1 } else { y - 1 };
        let next_y = if y == self.current_state[x].len() - 1 { 0 } else { y + 1 };

        let mut count = 0;

        if self.current_state[prev_x][prev_y] == 'X' { count += 1; }
        if self.current_state[x][prev_y] == 'X' { count += 1; }
        if self.current_state[next_x][prev_y] == 'X' { count += 1; }
        if self.current_state[prev_x][y] == 'X' { count += 1; }
        if self.current_state[next_x][y] == 'X' { count += 1; }
        if self.current_state[prev_x][next_y] == 'X' { count += 1; }
        if self.current_state[x][next_y] == 'X' { count += 1; }
        if self.current_state[next_x][next_y] == 'X' { count += 1; }
        
        return count;
    }

    fn copy_state(&mut self) {
        for x in 0..self.current_state.len() {
            for y in 0..self.current_state[x].len() {
                self.tmp_state[x][y] = self.current_state[x][y];
            }
        }
    }

    fn pp(&mut self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Iteration {}", self.iter);

        for x in 0..self.current_state.len() {
            println!("{:?}", self.current_state[x]);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2
    {
        print_args();
        return;
    }

    let contents = fs::read_to_string(args[1].clone())
    .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\r\n").collect();
    let initial_state: Vec<&str> = lines[1..].to_vec();
    let gol_args: Vec<&str>  = lines[0].split(' ').collect();

    match gol_args.len() {
        3 => {
            let width: u32 = match gol_args[0].parse() {
                Ok(n) => {n},
                Err(_) => {
                    print_file_structure();
                    return;
                },
            };

            let height: u32 = match gol_args[1].parse() {
                Ok(n) => {n},
                Err(_) => {
                    print_file_structure();
                    return;
                },
            };

            let len: u32 = match gol_args[2].parse() {
                Ok(n) => {n},
                Err(_) => {
                    print_file_structure();
                    return;
                },
            };

            game_loop(width as usize, height as usize, len, initial_state);

        },
        _ => {
            print_file_structure();
        }
    }
}

fn print_file_structure() {
    println!("Expected file with the following structure: \n <width> <height> <len> ");
    println!("<height # of lines with width size containing either ' ' or 'X'>\n");
}

fn print_args() {
    println!("Expected: gameoflife.exe <inputfile>");
}

fn game_loop(width: usize, height: usize, len: u32, initial_state: Vec<&str>) {
    let mut state = GameState::new(width, height, initial_state);

    state.pp();

    for _ in 0..len {
        state.step();
        state.pp();
        
        thread::sleep(Duration::from_millis(250))
    }
}
