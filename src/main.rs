use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!();
  println!(r#" _____                              "#);
  println!(r#"|   __|_ _ ___ ___ ___              "#);
  println!(r#"|  |  | | | -_|_ -|_ -|             "#);
  println!(r#"|_____|___|___|___|___|             "#);
  println!(r#"           _   _                    "#);
  println!(r#"          | |_| |_ ___              "#);
  println!(r#"          |  _|   | -_|             "#);
  println!(r#"          |_| |_|_|___|          __ "#);
  println!(r#"     _____           _          |  |"#);
  println!(r#"    |   | |_ _ _____| |_ ___ ___|  |"#);
  println!(r#"    | | | | | |     | . | -_|  _|__|"#);
  println!(r#"    |_|___|___|_|_|_|___|___|_| |__|"#);
  println!();
  println!();
  println!("Welcome to the guessing game!");
  println!("First choose whether you want to guess or have me to guess the number... ");
  println!();

  loop {
    println!(">>> Do you want to guess?");
    println!("( Y - I want to guess / N - I don't want to guess / Q - Quit the game )");

    let mut choice = String::new();

    io::stdin()
      .read_line(&mut choice)
      .expect("Failed to read line");

    match choice.to_ascii_lowercase().trim() {
      "y" => guess_you(),
      "n" => guess_me(),
      "q" => {
        println!();
        println!("See you later! ;)");
        println!();
        break;
      }
      _ => println!("Invalid choice!"),
    }
  }
}

fn guess_you() {
  println!();
  println!("The rule for this game mode is simple:");
  println!("I will choose a number between 1 and 100 and YOU will try to guess it!");
  println!();

  let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
  let mut nr_attempts: u8 = 0;

  println!("Ok, I thought about my number already.");
  println!();
  loop {
    println!("Please input your guess. ( Q - quit )");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    match guess.to_ascii_lowercase().trim() {
      "q" => {
        println!("Did you give up? (ಡ_ಡ)");
        println!("By the way, the number was {secret_number}.");
        println!();
        break;
      }
      _ => {}
    }

    let guess: u8 = match guess.trim().parse::<u8>() {
      Ok(num) => {
        if num > 100 || num < 1 {
          println!("The number must be between 1 and 100!");
          continue;
        }
        num
      }
      Err(_) => {
        println!("Please type a valid number!");
        println!();
        continue;
      }
    };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
      Ordering::Less => {
        println!("You gotta try guessing a bigger number to get it right!");
        nr_attempts += 1;
      }
      Ordering::Greater => {
        println!("What can I say... It's smaller than that!");
        nr_attempts += 1;
      }
      Ordering::Equal => {
        println!("You got it right friend! Congratulations! ヾ( ˃ᴗ˂ )◞ • *✰");
        println!();
        println!("Number of attempts: {nr_attempts}");
        println!();
        break;
      }
    }
  }
}

fn guess_me() {
  println!();
  println!("The rule for this game mode is simple:");
  println!("YOU will choose a number between 1 and 100 and I will try to guess it!");
  println!();

  let mut min: u8 = 1;
  let mut max: u8 = 100;

  let mut random_range: i8 = rand::thread_rng().gen_range(0..=5);
  let mut negative: bool = rand::thread_rng().gen_range(0..=1) != 0;

  let mut guess: u8 = ((((min + max) / 2) as i8 + (random_range * if negative { -1 } else { 1 }))
    .unsigned_abs())
    .clamp(min, max);
  let mut last_guess: bool = false;
  let mut last_options: bool = false;
  let mut nr_attempts: u8 = 0;

  println!("Press ENTER when you are ready to start.");
  io::stdin().read_line(&mut String::new()).unwrap();

  println!();
  println!("My first guess is: {guess}.");
  loop {
    println!();
    if !last_guess {
      println!("Now, tell me, is my guess correct? Is it bigger or smaller than this?");
    }
    println!("( C - Correct / B - Bigger / S - Smaller / Q - Quit )");

    let mut choice = String::new();

    io::stdin()
      .read_line(&mut choice)
      .expect("Failed to read line");

    match choice.to_ascii_lowercase().trim() {
      "b" => {
        println!();
        if last_guess {
          println!("Are you trying to trick me? You said there are no numbers bigger or smaller than {guess}... (ಥ﹏ಥ)");
          println!();
          continue;
        }
        if guess == 100 {
          println!("Are you trying to trick me? There are no numbers bigger than 100... (ಥ﹏ಥ)");
          println!();
          continue;
        }
        min = guess + 1;
        nr_attempts += 1;
        println!("Ok, I will try to guess a bigger number.");
        println!();
      }
      "s" => {
        println!();
        if last_guess {
          println!("Are you trying to trick me? You said there are no numbers bigger or smaller than {guess}... (ಥ﹏ಥ)");
          println!();
          continue;
        }
        if guess == 1 {
          println!("Are you trying to trick me? There are no numbers smaller than 1... (ಥ﹏ಥ)");
          println!();
          continue;
        }
        max = guess - 1;
        nr_attempts += 1;
        println!("I'll try guessing a smaller number then.");
        println!();
      }
      "c" => {
        println!();
        println!("I knew it! I'm a genius! ( • ̀ω•́ )✧");
        println!();
        println!("I got it in {nr_attempts} attempts.");
        println!();
        break;
      }
      "q" => {
        println!();
        println!("WHAT? Are you gonna leave me here curious about the number? (ಠ益ಠ)");
        println!();
        break;
      }
      _ => {
        println!("Invalid choice!");
        continue;
      }
    }

    random_range = rand::thread_rng().gen_range(0..=5);
    negative = rand::thread_rng().gen_range(0..=1) != 0;

    if max - min <= 25 {
      guess = (min + max) / 2;
    } else {
      guess = ((((min + max) / 2) as i8 + (random_range * if negative { -1 } else { 1 }))
        .unsigned_abs())
        .clamp(min, max);
    }
    if min >= 95 && !last_options {
      guess = 100;
      last_options = true;
    }
    if max <= 5 && !last_options {
      guess = 1;
      last_options = true;
    }
    if max == min {
      last_guess = true;
      println!("I think I got it! Is it {guess}?");
      continue;
    }
    println!("Well then... My next guess is: {guess}.");
  }
}
