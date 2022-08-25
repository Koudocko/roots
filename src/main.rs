use colored::Colorize;
use rand::{Rng, rngs::ThreadRng};
use std::{time::Duration, thread, num::IntErrorKind};

#[derive(Clone)]
struct Stem{
    current_pos: i32,
    origin_pos: i32,
    colour: i32,
}

fn wiggle(handle: &mut ThreadRng, stem: i32, bredth: i32)-> i32{
    let pos: i32 = handle.gen_range(-1..=1);

    let stem_new = stem+pos;
    if stem_new < 0{
        0
    }
    else if stem_new >= bredth{
        bredth-1
    }
    else{
        stem_new
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
        }
    }).collect::<Vec<Stem>>();

    loop{
        for completed in 0..*depth{
            let mut stems_new = Vec::<Stem>::new();
            let mut output = (0..*bredth).map(|_| (" ", 0)).collect::<Vec<(&str, i32)>>();

            for stem in &mut stems{
                let change = wiggle(&mut handle, stem.current_pos, *bredth);

                if *depth-completed > (stem.current_pos-stem.origin_pos).abs(){
                    if 0_i32 == handle.gen_range(0..*frequency){
                        if output[change as usize].0 == " "{
                            stem.current_pos = change;
                            output[stem.current_pos as usize] = ("o", stem.colour);
                            colour_curr = (colour_curr+1)%6;

                            stems_new.push(Stem{
                                current_pos: wiggle(&mut handle, stem.current_pos, *bredth),
                                origin_pos: stem.origin_pos,
                                colour: colour_curr,
                            });
                        }
                    }
                    else{
                        if output[change as usize].0 == " "{
                            stem.current_pos = change;
                        }
                        
                        output[stem.current_pos as usize] = ("|", stem.colour);
                    }
                } 
                else{
                    if stem.current_pos > stem.origin_pos{
                        stem.current_pos -= 1;
                    }
                    else{
                        stem.current_pos += 1;
                    }

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
    let (mut depth, mut bredth, mut frequency, mut number, mut speed) = (100, 100, 50, 5, 50);

    parser(&mut depth, &mut bredth, &mut frequency, &mut number, &mut speed);
    animation(&mut depth, &mut bredth, &mut frequency, &mut number, &mut speed);
}
