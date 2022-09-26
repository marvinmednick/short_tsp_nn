use crate::graphbuilder::GraphBuilder;

use log::{  info ,/* error ,*/ debug, /* warn ,*/ trace };


#[derive(Debug,Clone)]
struct Vertex {
    vertex_id: usize,
    xpos: f64,
    ypos: f64,
}

pub struct TSP {
    unprocessed_vertex: Vec<Vertex>,
    processed_vertex: Vec<Vertex>,
    total_distance: Vec<f64>,
}


impl GraphBuilder for &mut TSP{
    fn add_vertex(&mut self, vertex_id: usize, xpos: f64, ypos: f64) {
        self.unprocessed_vertex.push(Vertex { vertex_id, xpos, ypos });
    }
}


impl TSP {

    pub fn new() -> TSP {
        TSP {
            unprocessed_vertex: Vec::<Vertex>::new(),
            processed_vertex: Vec::<Vertex>::new(),
            total_distance: Vec::<f64>::new(),
        }

    }


    pub fn calculate(&mut self, starting_vertex: usize) {

    }


    pub fn solution(&self) -> Option<(f64, Vec<usize>)> {
        let result : f64 = self.unprocessed_vertex.len() as f64;
        Some((result,Vec::<usize>::new()))
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
