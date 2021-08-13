use std::{env, fs};
use frhd;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the track file!");

    let instructions = contents.lines()
        .map(|s| s.trim().split(' ').map(String::from).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();


    let mut track = frhd::Track {
        trackdata: String::new(),
        physical: Vec::new(),
        scenery: Vec::new(),
        powerups: String::new()
    };
 
    
    for i in 0..instructions.len() {
        for j in 0.. instructions[i as usize].len() {
            if instructions[i][j] == "p".to_string() {
                track.insert_line(
                    instructions[i][1].parse::<i32>().unwrap(), 
                    instructions[i][2].parse::<i32>().unwrap(), 
                    instructions[i][3].parse::<i32>().unwrap(), 
                    instructions[i][4].parse::<i32>().unwrap(), 
                    'p'
                );
            } else if instructions[i][j] == "s".to_string() {
                track.insert_line(
                    instructions[i][1].parse::<i32>().unwrap(), 
                    instructions[i][2].parse::<i32>().unwrap(), 
                    instructions[i][3].parse::<i32>().unwrap(), 
                    instructions[i][4].parse::<i32>().unwrap(), 
                    's'
                );
            } else if instructions[i][j] == "c" {
                track.insert_check(
                    instructions[i][1].parse::<i32>().unwrap(),
                    instructions[i][2].parse::<i32>().unwrap()
                );
            } else if instructions[i][j] == "t" {
                track.insert_star(
                    instructions[i][1].parse::<i32>().unwrap(),
                    instructions[i][2].parse::<i32>().unwrap()
                );
            } else if instructions[i][j] == "m" {
                track.insert_slow_mo(
                    instructions[i][1].parse::<i32>().unwrap(),
                    instructions[i][2].parse::<i32>().unwrap()
                );
            } else if instructions[i][j] == "o" {
                track.insert_bomb(
                    instructions[i][1].parse::<i32>().unwrap(),
                    instructions[i][2].parse::<i32>().unwrap()
                );
            } else if instructions[i][j] == "g" {
                track.insert_gravity(
                    instructions[i][1].parse::<i32>().unwrap(),
                    instructions[i][2].parse::<i32>().unwrap(),
                    instructions[i][3].parse::<i32>().unwrap()
                );
            } else if instructions[i][j] == "b".to_string() {
                track.insert_boost(
                    instructions[i][1].parse::<i32>().unwrap(),
                    instructions[i][2].parse::<i32>().unwrap(),
                    instructions[i][3].parse::<i32>().unwrap()
                );
            }
        }
    }

    println!("{}", track.generate_code());
}
