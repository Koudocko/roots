use colored::Colorize;
use rand::{Rng, rngs::ThreadRng};
use std::{time::Duration, thread, num::IntErrorKind};

#[derive(Clone)]
struct Stem{
    current_pos: i32,
    origin_pos: i32,
    colour: i32,
    dir: bool,
}

fn wiggle(handle: &mut ThreadRng, stem: &Stem, bredth: i32, depth: i32, completed: i32)-> (i32, bool){
    let (dir, weight) =
        if completed >= (depth/2){
            if stem.current_pos > stem.origin_pos{ (false, 4) }
            else { (true, 4) }
        }
        else{(stem.dir, 2) };

    let mut pos: i32 = 
        if stem.dir{ handle.gen_range(-1..=weight) }
        else{ handle.gen_range(-weight..=1) };
    pos = 
        if pos > 0{ 1 }
        else if pos < 0{ -1 }
        else{ 0 };

    let stem_new = stem.current_pos+pos;
    if stem_new < 0{
        (1, true)
    }
    else if stem_new >= bredth{
        (bredth-1, false)
    }
    else{
        (stem_new, dir)
    }
}

fn animation(depth: &mut i32, bredth: &mut i32, frequency: &mut i32, number: &mut i32, speed: &mut i32){
    let mut handle = rand::thread_rng();
    let mut colour_curr: i32 = handle.gen_range(0..=5); 

    let mut stems = (0..*number).map(|_|{
        colour_curr = (colour_curr+1)%6;
        let pos = handle.gen_range(0..*bredth);

        Stem{
            current_pos: pos,
            origin_pos: pos,
            colour: colour_curr,
            dir: handle.gen_ratio(1, 2)
        }
    }).collect::<Vec<Stem>>();

    loop{
        for completed in 0..*depth{
            let mut stems_new = Vec::<Stem>::new();
            let mut output = (0..*bredth).map(|_| (" ", 0)).collect::<Vec<(&str, i32)>>();

            let num_stems = stems.len() as i32;
            for stem in &mut stems{
                let (change_pos, change_dir) = wiggle(&mut handle, stem, *bredth, *depth, completed);

                if 0_i32 == handle.gen_range(0..*frequency) && num_stems < *bredth{
                    stem.current_pos = change_pos;
                    stem.dir = change_dir;
                    output[stem.current_pos as usize] = ("o", stem.colour);
                    colour_curr = (colour_curr+1)%6;

                    stems_new.push(Stem{
                        current_pos: wiggle(&mut handle, stem, *bredth, *depth, completed).0,
                        origin_pos: stem.origin_pos,
                        colour: colour_curr,
                        dir: handle.gen_ratio(1, 2)
                    });
                }
                else{
                    stem.current_pos = change_pos;
                    stem.dir = change_dir;
                    
                    output[stem.current_pos as usize] = ("|", stem.colour);
                }
            }

            output.iter().for_each(|col|{
                print!(
                    "{}",
                    match col{
                        (col, 0) => col.red(),
                        (col, 1) => col.yellow(),
                        (col, 2) => col.green(),
                        (col, 3) => col.blue(),
                        (col, 4) => col.magenta(),
                        (col, _) => col.cyan(),
                });
            });
            println!();

            stems_new.iter().for_each(|stem| stems.push(stem.clone()));
            
            thread::sleep(Duration::from_millis(*speed as u64));
        }

        stems.drain((*number as usize)..stems.len());
        stems.iter_mut().for_each(|x| x.current_pos = x.origin_pos );
    }
}

fn set_var(var: &mut i32, splits: &Vec<char>){
    let arg = splits.iter().collect::<String>().yellow();

    match str::parse::<i32>(
        &splits[2..].iter().collect::<String>())
    {
        Ok(value) => {
            if value > 0{
                *var = value;
            }
            else{
                eprintln!(
                    "{} Value of arg {} must be greater than 0!",
                    "[!]".yellow(),
                    arg.yellow()
                );
            }
        }
        Err(error) => match error.kind(){
            IntErrorKind::Empty =>{
                eprintln!(
                    "{} No value provided for arg {}!",
                    "[!]".yellow(),
                    arg.yellow()
                );
            }
            IntErrorKind::InvalidDigit =>{
                eprintln!(
                    "{} Invalid value provided for arg {}!",
                    "[!]".yellow(),
                    arg.yellow()
                );
            }
            _ =>{
                eprintln!(
                    "{} Overflow/underflow error for arg {}!",
                    "[!]".yellow(),
                    arg.yellow()
                );
            }
        }
    }
}

fn parser(depth: &mut i32, bredth: &mut i32, frequency: &mut i32, number: &mut i32, speed: &mut i32){
    for arg in std::env::args().skip(1){
        let splits = arg.chars().collect::<Vec<char>>();

        if splits[0] == '-'{
            if splits.len() > 1{
                match splits[1]{
                    'd' => set_var(depth, &splits),
                    'b' => set_var(bredth, &splits),
                    's' => set_var(speed, &splits),
                    'f' => set_var(frequency, &splits),
                    'n' => set_var(number, &splits),
                    'h' =>{
                        println!("{} roots <operation> <value>", "usage:".white());
                        println!("{}", "operations:".white());
                        println!("   roots {{-d}}<x> : set depth of animation (x rows)");
                        println!("   roots {{-b}}<x> : set bredth of animation (x column)"); 
                        println!("   roots {{-s}}<x> : set speed of animation (x ms delay)");
                        println!("   roots {{-f}}<x> : set frequency of root splits (1/x chance)"); 
                        println!("   roots {{-n}}<x> : set number of starting roots (x roots)"); 
                        std::process::exit(0);
                    }
                    _ =>{
                        eprintln!(
                            "{} Invalid argument {}!",
                            "[!]".yellow(),
                            arg.yellow()
                        );
                    }
                }
            }
            else{
                eprintln!(
                    "{} Invalid argument {}!",
                    "[!]".yellow(),
                    arg.yellow()
                );
            }
        }
        else{
            eprintln!(
                "{} Invalid argument {}!",
                "[!]".yellow(),
                arg.yellow()
            );
        }
    }
}

fn main() {
    let (mut depth, mut bredth, mut frequency, mut number, mut speed) = (100, termsize::get().unwrap().cols as i32, 50, 5, 50);

    parser(&mut depth, &mut bredth, &mut frequency, &mut number, &mut speed);
    animation(&mut depth, &mut bredth, &mut frequency, &mut number, &mut speed);
}
