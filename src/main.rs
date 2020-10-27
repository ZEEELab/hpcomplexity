use hpcomplexity::config::Config;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    //Initialise main configuration
    let cfg = Config::init();
    println!("Main configuration: {:}",cfg);
    
    //Initialise partitioning scheme (PTypes class)
    //let ptypes = Ptypes::init();

}
