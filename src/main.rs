use std::env;
use std::io;
use std::vec;
use std::fs;
use rand;

// chose the right state of the hanged man
fn hanging(ref remaining: i32) -> String {
    match remaining {
        8 => String::from(
            "
      
              
              
              
              
      
 =========================       
",
        ),

        7 => String::from(
            "
      
              
              
              
      |        
      |
 =========================       
",
        ),

        6 => String::from(
            "
      
      |        
      |        
      |        
      |        
      |
 =========================       
",
        ),

        5 => String::from(
            "
      ___________
      |        
      |        
      |        
      |        
      |
 =========================       
",
        ),
        4 => String::from(
            "
      ___________
      |        |
      |        
      |        
      |        
      |
 =========================       
",
        ),
        3 => String::from(
            "
      ___________
      |        |
      |        o
      |        
      |        
      |
 =========================       
",
        ),
        2 => String::from(
            "
      ___________
      |        |
      |        o
      |        |
      |        
      |
 =========================       
",
        ),
        1 => String::from(
            "
      ___________
      |        |
      |        o
      |       /|\\
      |        
      |
 =========================       
",
        ),
        0 => String::from(
            "
      ___________
      |        |
      |        o
      |       /|\\
      |       / \\
      |
 =========================       
",
        ),
        _ => String::from(""),
    }
}

// get the next player input
fn get_input() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("failed to read input");
    buf[0..buf.len()-1].to_string()
}

// check if a user inut is valid
fn is_valid_input(input: &String) -> bool {
    if input.len() == 0 {
        println!("Invalid input, too short");
        return false;
    }
    for char in input.chars(){
        if !char.is_ascii_alphabetic() {
            println!("Invalid input, only use alphabetic characters");
            return false;
        }
    }
    return true;
}

// choose a random word for te game
fn get_new_word(filename : String) -> String{
    let file = fs::read_to_string(filename)
                        .expect("could not read from file");
    let mut words = file.split('\n');
    let rand = rand::random::<usize>() % &words.clone().count();
    words.nth(rand).unwrap().to_owned()
}



// the game main loop
// start and end it when needed
fn game(filename : String) {
    let to_guess = get_new_word(filename);
    // create a string of the same size as to_guess filled with '_'
    let mut curr_word = String::new();
    for _ in 0..to_guess.len()-1 {
        curr_word.push('_');
    }

    // println!("{}", to_guess);
    // println!("{}", curr_word);

    let mut used_letter: Vec<char> = vec![];
    let mut remaining_guess = 8;
    print!("{}", hanging(remaining_guess));

    while remaining_guess > 0 {
        println!("{}", curr_word);
        println!("You have {} remaining guess", remaining_guess);
        if used_letter.len()>0 {
            let mut used_iter = used_letter.iter();
            print!("currently used letter are : {}", used_iter.next().unwrap());
            for char in used_iter {
                print!(", {char}");
            }
            println!(".");
        }
        loop {
            println!("Please enter your next guess");
            let mut input = get_input();
            while !is_valid_input(&input) {
                println!("Please enter your next guess");
                input = get_input();
            }
            match input.len() {
                1 => {
                    if used_letter.contains(&input.chars().next().unwrap()) {
                        println!("You already used this letter!");
                    } else if to_guess.contains(&input) {
                        println!("Correct Guess!");
                        for (i, char) in to_guess.chars().enumerate() {
                            if char == input.chars().next().unwrap() {
                                curr_word.replace_range(i..i + 1, &input);
                            }
                        }
                        used_letter.push(input.chars().next().unwrap());
                        remaining_guess += 1;
                        break;
                    } else {
                        println!("Wrong Guess!");
                        used_letter.push(input.chars().next().unwrap());
                        break;
                    }
                }
                _ => {
                    if input == to_guess {
                        println!("Congratulation you won the game :)");
                        return;
                    }
                    println!("Wrong Guess!");
                    break;
                }
            }
        }
        if curr_word == to_guess {
            println!("You correctly guessed {to_guess}!");
            println!("Congratulation you won the game! :)");
            return;
        }
        remaining_guess -= 1;
        print!("{}", hanging(remaining_guess));
    }

    println!("YOU LOSE! :(");
    println!("The word was {to_guess}");

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: String;
    match args.len() {
        1 => filename = String::from("data/word_list.txt"),
        2 => filename = args[1].clone(),
        _ => {println!("Incorrect argument pleas use : [filename]");
                return;
            }
    };


    println!("Welcome to the Hangman game!");
    game(filename);
}
