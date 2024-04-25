use std::io;
use std::io::{prelude::*, BufReader};
use std::fs::OpenOptions;
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

mod guess;
mod game_data;
mod score;

use crate::score::Score;
use crate::game_data::GameDataHandler;
use crate::guess::Guess;

static SCORES: &str = "scores/scores.txt";

fn main() {
    menu();
}

fn clear(){
    print!("{}[2J", 27 as char);
}

fn raw_input() -> String {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("La bagarre");

    input
}

fn menu() {
    clear();
    println!("Guess the number!");
    println!("1. Play\n2. Scores\n3. Quit");

    let input = raw_input();
    let input: u32 = input.trim().parse().expect("BAGAR");

    match input {
        1 => init_game(),
        2 => display_scores(2),
        _ => quit(), 
    }
}

fn quit(){
}

fn display_scores(rank: u32) {
    clear();
    let scores_file = File::open(SCORES).expect("oops");
    let reader = BufReader::new(scores_file);

    let mut scores = vec![];
    for line in reader.lines() {
        let line_content = line.expect("Error reading line");
        let columns = line_content.split(":").collect::<Vec<&str>>();
        
        let player = String::from(*columns.get(0).expect(""));

        let tries: u32 = columns.get(1).expect("").trim().parse().expect("Tries is not an u32");
        
        let time = columns.get(2).expect("").trim();
        let time: f64 = time[..(time.len()-1)].parse().expect("Time is not a f64");

        scores.push(Score { player, tries, time});
    }

    match rank {
        1 => scores.sort_by_key(|k| k.tries),
        _ => scores.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap()),
    }

    println!("player\ttries\ttime\n______\t_____\t____\n");
    for score in scores {
        println!("{}\t{}\t{}", score.player, score.tries, score.time);
    }

    println!("_____________________\nTrier par :\n1. Tries\n2. Time\n________\n3. Menu");

    let input = raw_input();
    let input = input.trim().parse().expect("Invalid input");

    match input {
        1 => display_scores(1),
        2 => display_scores(2),
        _ => menu(),
    }
}

fn init_game() {
    clear();
    println!("Enter player name :");

    let player_name = raw_input();

    let mut player_data = GameDataHandler::new(String::from(player_name.trim()));

    game(&mut player_data);
    menu();
}

fn game(player_data: &mut GameDataHandler){
    clear();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Game started !");

    loop {
        println!("Please input your guess.");
        
        let guess = raw_input();
        clear();
        
        println!("You guessed: {guess}");
        

        let guess: i32 = guess.trim().parse().expect("Not an integer number");
        let guess = Guess::new(guess);
        

        player_data.make_guess();
        
        match guess.value().cmp(&secret_number){
            Ordering::Less => println!("Toupoti"),
            Ordering::Equal => {
                register_score(&player_data);

                break;
            },
            Ordering::Greater => println!("tougro"),
        }
    }
}

fn register_score(player_data: &GameDataHandler){
    let tries = player_data.tries();
    let time = player_data.time();
    let player_name = player_data.player_name();

    println!("Olala bravo très intelligent");              
    println!("tries : {tries}");
    println!("time : {time:?}");

    let score_line = format!("{player_name}:{tries}:{time:?}");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("scores/scores.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", score_line) {
        eprintln!("Couldn't write to file: {}", e);
    }

    println!("Enter to continue..");
    let _ = raw_input();
}