mod tsp_nn;
mod graphbuilder;
mod cmd_line;
mod parse;
mod minmax;

use crate::cmd_line::CommandArgs;

use clap::Parser;
use crate::tsp_nn::TSP;
use log::{  info, debug, };
use std::path::Path;
use std::fs::File;
use crate::parse::read_vertex_location;

fn main() {
    
    env_logger::init();


    let cmd_line = CommandArgs::parse();
    debug!("The Command Line, {:?}!",cmd_line);

    // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };


    let mut tsp = TSP::new();

    info!("Reading Vertex location");
    read_vertex_location(&mut file, &mut tsp);

    tsp.calculate(1);
    let (distance, path) = tsp.solution() ;

    // TODO Check or revise this -- 0 is valid length
    let int_distance : i64 = *distance.unwrap_value_or(&0.0) as i64;
    if cmd_line.verbose {
        println!("TSP Distance {}   Path is {:?}  int distances {}", distance, path, int_distance);
    }
    else if cmd_line.path {
        println!("{:?}", path );
    }
    else {
        println!("{}",int_distance);
    }
}
