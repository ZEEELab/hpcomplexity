use hpcomplexity::config::Config;
use hpcomplexity::world::World;
use std::time::Instant;
use std::process;
use std::fs;

fn main() {
    let start_time = Instant::now();

    // Initialise main configuration
    let cfg = Config::init();
    //println!("Main configuration: {:}",cfg); //@DEBUG
    
    // Initialise the world (environment + host population + parasite population)
    let mut world = World::init(&cfg)
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
