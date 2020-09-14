use std::env;
use std::fs;
fn handle_args(args: &Vec<String>) {
    let argc: usize = args.len();
    if argc != 4 {
        println!("Error. Wrong number of arguments");
        println!("Usage: cargo run <filename> <repalcee> <replacer>");
        println!(" E.g. cargo run tmp.txt wiht with");
        std::process::exit(1);
    }
}



fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    handle_args(&args);
    let filename: String = args[1].clone();
    let tmp_filename = filename.clone()+".new";
    let replacee: String = args[2].clone();
    let replacer: String = args[3].clone();
    // println!("{}", argc);
    //println!("{}", filename);
    //println!("{}", replacee);
    //println!("{}", replacer);
    let mut contents = match fs::read_to_string(&filename) {
        Err(why) => panic!("Coult not read {} because {}", &filename, why),
        Ok(contents) => contents,
    };
    println!("{}", contents);
    let old_str = contents.clone();
    contents = contents.replace(&replacee, &replacer);
    if contents.eq(&old_str) {
        println!("Booh!, no \"{}\" found", &replacee);
        std::process::exit(1);
    }
    println!("{}", contents);
    let mut _wrote = match fs::write(&tmp_filename, contents.as_bytes()) {
        Err(why) => panic!("Coult not read {} because {}", &tmp_filename, why),
        Ok(_wrote) => _wrote,
    };
    fs::rename(&tmp_filename, &filename)?;
    Ok(())
}
