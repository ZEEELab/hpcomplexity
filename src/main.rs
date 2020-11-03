use hpcomplexity::config::Config;
use hpcomplexity::world::World;
use std::time::Instant;
use std::process;
use std::fs;

fn main() {
    let start_time = Instant::now();

    // Initialise main configuration
    let cfgfilename = String::from("config.csv"); 
    let cfg = Config::from_file(&cfgfilename)
        .unwrap_or_else(|err|{
            println!("Problem opening config file {} : {}",&cfgfilename,err);
            process::exit(1);
        });
    //println!("Main configuration: {:}",cfg); //@DEBUG
    
    // Initialise the world (environment + host population + parasite population)
    let world = World::init(&cfg)
        .unwrap_or_else(|err|{
            println!("Unable to create world: {}",err);
            process::exit(1);
        });

    // Create output folder (results)
    fs::create_dir_all("results").unwrap(); 

    // Start the run
    world.start_run(&cfg);

    // Measure and print time
    println!("Completed run! ({:?})",start_time.elapsed());
}
