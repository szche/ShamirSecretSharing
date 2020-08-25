use std::env;
use std::process;
use rand::Rng;

#[derive(Debug)]
struct Config {
    secret: u32,
    n: u32,
    k: u32,
}

struct Secret {
    fx: Vec<i32>,
    points: Vec<(i32, i32)>,
}

fn main() {
    //Collect terminal arguments
    let args: Vec<String> = env::args().collect();

    //Create config struct
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{:?}", config);

    let secret = Secret::new(&config);
    
}


impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        //Skip the filename
        args.next();

        let n = match args.next() {
            Some(arg) => arg.parse::<u32>().unwrap(),
            None => return Err("Didn't recieve a \'n\' number"),
        };

        let k = match args.next() {
            Some(arg) => arg.parse::<u32>().unwrap(),
            None => return Err("Didn't recieve a \'k\' number"),
        };

        let secret = match args.next() {
            Some(arg) => arg.parse::<u32>().unwrap(),
            None => return Err("Didn't recieve a \'secret\' number"),
        };
        
        //Return the Config struct
        Ok(Config {
            secret,
            n,
            k,
        })
    }
}

impl Secret {
    fn new(config: &Config) -> () {
        let mut rng = rand::thread_rng();

        let mut fx = Vec::new();

        //Generate random k-1 numbers
        for _i in 1.. config.k {
            println!("Random u32: {}", rng.gen::<u32>());
            fx.push(rng.gen::<u32>());
        }

        println!("Polynomial: {} {:?}", config.secret, fx);

        //Generate secret points

    }



}
