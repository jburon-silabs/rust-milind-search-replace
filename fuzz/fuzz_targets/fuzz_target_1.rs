#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut contents = match String::from_utf8(data.to_vec()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let replacee: String = "x".to_string();
    let replacer:  String = "z".to_string();
    //println!("{}", contents);
    let old_str = contents.clone();
    contents = contents.replace(&replacee, &replacer);
    if contents.eq(&old_str) {
        println!("Booh!, no \"{}\" found", &replacee);
        //std::process::exit(0);
    }
    println!("{}", contents);
});

