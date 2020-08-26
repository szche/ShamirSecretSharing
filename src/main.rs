use std::env;
use std::process;
use rand::Rng;

#[derive(Debug)]
struct Config {
    secret: u32,
    n: u32,
    k: u32,
}

#[derive(Debug)]
struct Secret {
    fx: Vec<u128>,
    points: Vec<(u128, u128)>,
}

fn main() {
    //Create config struct
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Created config:");
    println!("{:?}", config);

    let secret = Secret::new(&config);
    println!("Created secret!");
    println!("{:?}", secret);
    
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
    //Generate the polynomial
    fn new(config: &Config) -> Secret {
        let mut rng = rand::thread_rng();

        let mut fx: Vec<u128> = vec![config.secret.into()];
        let mut points: Vec< (u128, u128) > =  vec![];
        //Generate random k-1 numbers of the polynomial
        for _i in 1.. config.k {
            let number = rng.gen::<u16>();
            //println!("Random u16: {}", number); 
            fx.push(number.into());
        }

        //println!("Polynomial: {:?}", fx);

        //Generate secret points
        for i in 1..config.n {
            let mut power = 0;
            let mut sum = 0;
            for x in fx.iter() {
                //println!("{} * {}^{}", x, i, power);
                sum += x * ((i.pow(power)) as u128);
                power += 1;
            }
            //println!("Point: ({}, {})", i, sum);
            points.push( (i as u128, sum) );
        }
        
        Secret { fx, points,}
    }
}
