use crate::graphbuilder::GraphBuilder;
use std::collections::BTreeMap;

use crate::minmax::{MinMax,MinMax::NA,MinMax::Value};

use log::{  info ,/* error ,*/ debug, /* warn ,*/ trace };


#[derive(Debug,Clone)]
struct Vertex {
    vertex_id: usize,
    xpos: f64,
    ypos: f64,
}

pub struct TSP {
    unprocessed_vertex: BTreeMap<usize, Vertex>,
    processed_vertex: Vec<usize>,
    total_distance:   MinMax<f64>,
}


impl GraphBuilder for &mut TSP{
    fn add_vertex(&mut self, vertex_id: usize, xpos: f64, ypos: f64) {
        self.unprocessed_vertex.insert(vertex_id,Vertex { vertex_id, xpos, ypos });
    }
}


impl TSP {

    pub fn new() -> TSP {
        TSP {
            unprocessed_vertex: BTreeMap::<usize, Vertex>::new(),
            processed_vertex: Vec::<usize>::new(),
            total_distance:   MinMax::NA,
        }

    }


    pub fn calculate(&mut self, starting_vertex : usize) {

        let mut _count = 0;
        let mut current_vertex_id = starting_vertex;
        // TODO check for valid starting vertex
        let start_vertex = self.unprocessed_vertex.get(&current_vertex_id).unwrap();
        let start_xpos = start_vertex.xpos.clone();
        let start_ypos = start_vertex.ypos.clone();
        let mut cur_xpos = 0.0;
        let mut cur_ypos = 0.0;

        let mut current_distance : f64  = 0.0;

        while self.unprocessed_vertex.len() > 0 {
            debug!("Processing vertex {}",current_vertex_id);
            let vertex = self.unprocessed_vertex.remove(&current_vertex_id).unwrap();
            cur_xpos = vertex.xpos.clone();
            cur_ypos = vertex.ypos.clone();
            // put the current vertex on the processed stach
            self.processed_vertex.push(vertex.vertex_id.clone());
            _count += 1;
            // loop through the remaining vertexe in order to find the
            // nearest neighbor
            let mut min_dist_squared  = MinMax::<f64>::Max;

            for (id, v) in &self.unprocessed_vertex {
                let distance_squared = (cur_xpos - v.xpos).powf(2.0) + (cur_ypos-v.ypos).powf(2.0);
                trace!("Distance squared for {} to {} is {}",current_vertex_id,v.vertex_id,distance_squared);
                if Value(distance_squared) < min_dist_squared {
                    debug!("Found {} as new nearest neighbor for {} (dist={})",v.vertex_id,current_vertex_id,distance_squared);
                    min_dist_squared = Value(distance_squared);
                    current_vertex_id = v.vertex_id.clone();
                    debug!("(dist={})",min_dist_squared);
                }
            }
            debug!("Nearest Neighbor is {} - distance of {}",current_vertex_id, min_dist_squared);

            // if we found a nearest neighbor, then add the distance to it 
            // and continue to the next vertex
            if min_dist_squared != MinMax::Max {
                // add the distanced
                current_distance += min_dist_squared.unwrap_value().sqrt();
                debug!("current Distance={}",current_distance);
            }
        }
        // curre

        let final_distance_squared = (cur_xpos - start_xpos).powf(2.0) + (cur_ypos-start_ypos).powf(2.0);
        self.total_distance += Value(current_distance+final_distance_squared.sqrt());

    }

    pub fn solution(&self) -> (MinMax<f64>, &Vec<usize>) {
        (self.total_distance,&self.processed_vertex)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    fn init_log() {
       let _ = env_logger::builder().is_test(true).try_init();
       info!("Init {}",module_path!());

    }

    #[test]
    fn test_simple_5() {
        init_log();

        let mut tsp = TSP::new();

        // set position to all 0.0
        for x in 1..=5  {
            tsp.define_vertex(x,0.0,0.0);
        }

        let mut i = 1;
        tsp.define_edge(1,2,i);   i+=1;
        tsp.define_edge(3,2,i);   i+=1;
        tsp.define_edge(3,4,i);   i+=1;
        tsp.define_edge(4,5,i);   i+=1;   
        tsp.define_edge(1,5,i);   i+=1;
        tsp.define_edge(1,4,i);   i+=1;
        tsp.define_edge(1,3,i);   i+=1;
        tsp.define_edge(2,4,i);   i+=1;
        tsp.define_edge(2,5,i);   i+=1;
        tsp.define_edge(3,5,i);   // i+=1;
        tsp.calculate(1);
        let (distance, path) = tsp.solution();
        assert_eq!(distance,Value(15));
        assert_eq!(path,&vec![5,4,3,2,1]);

    }

    #[test]
    fn test_simple_4() {
        init_log();

        let mut tsp = TSP::new();

        // set position to all 0.0
        for x in 1..=4  {
            tsp.define_vertex(x,0.0,0.0);
        }

        let mut i = 1;
        tsp.define_edge(1,2,i);   i+=1;
        tsp.define_edge(3,2,i);   i+=1;
        tsp.define_edge(3,4,i);   i+=1;
        tsp.define_edge(1,4,i);   i+=1;
        tsp.define_edge(1,3,i);   i+=1;
        tsp.define_edge(2,4,i);   //i+=1;
        tsp.calculate(1);
        let (distance, path) = tsp.solution();
        assert_eq!(distance,Value(10));
        assert_eq!(path,&vec![4,3,2,1]);

    }

    #[test]
    fn test_float_4() {
        init_log();
        let mut tsp = TSP::<f32>::new();

        // set position to all 0.0
        for x in 1..=4  {
            tsp.define_vertex(x,0.0,0.0);
        }
        let mut i = 1.0;
        tsp.define_edge(1,2,i);   i+=1.0;
        tsp.define_edge(3,2,i);   i+=1.0;
        tsp.define_edge(3,4,i);   i+=1.0;
        tsp.define_edge(1,4,i);   i+=1.0;
        tsp.define_edge(1,3,i);   i+=1.0;
        tsp.define_edge(2,4,i);   //i+=1;
        tsp.calculate(1);
        let (distance, path) = tsp.solution();
        assert_eq!(distance,Value(10.0));
        assert_eq!(path,&vec![4,3,2,1]);

    }

    #[test]
    fn test_float_10_4() {
        init_log();
        let mut tsp = TSP::<f32>::new();
        tsp.define_vertex(1, 3.433752748235324,2.9215164273513206);
        tsp.define_vertex(2, 0.266027289402357, 3.367553812393056);
        tsp.define_vertex(3, 3.107592426409198, 3.091359997997841);
        tsp.define_vertex(4, 1.2770174634306963, 1.4543288785259425);
        tsp.generate_edges_by_dist();
        tsp.calculate(1);
        let (distance, path) = tsp.solution();
        debug!("Distance {} , path {:?}",distance, path);
        let mut int_distance : MinMax<i64> = MinMax::NA;
        if let Value(dist) = distance {
            int_distance  = Value (dist as i64)
        }
        assert_eq!(int_distance,Value(7));

    }

    #[test]
    fn test_float_1_2() {
        init_log();
        let mut tsp = TSP::<f32>::new();
        tsp.define_vertex(1,1.185111439847509,1.1487624635211768);
        tsp.define_vertex(2,1.4444704252469853,1.9471010355780376);
        tsp.generate_edges_by_dist();
        tsp.calculate(1);
        let (distance, path) = tsp.solution();
        debug!("Distance {} , path {:?}",distance, path);
        let mut int_distance : MinMax<i64> = MinMax::NA;
        if let Value(dist) = distance {
            int_distance  = Value (dist as i64)
        }
        assert_eq!(int_distance,Value(1));

    }

}
