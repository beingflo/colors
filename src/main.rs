extern crate graml;
extern crate num_cpus;

use std::thread;
use std::sync::mpsc;

use graml::graph::*;
use graml::coloring::*;

fn main() {
    // Run comparison on this many graphs
    let samples = 50;

    // Number of vertices in each sample graph
    let n = 200;

    // Edge probability in each sample graph
    let p = 0.5;

    // Number of processors
    let workers = num_cpus::get();

    // Communication channel between worker threads and main thread
    let (tx, rx) = mpsc::channel();

    for i in 0..workers {
        // Each worker performs comparison on this many graphs
        let mut slice = samples / workers;

        // Last worker picks up remaining samples
        if i == workers - 1 {
            slice += samples - (samples / workers) * workers;
        }

        let tx_ = tx.clone();

        // Spawn workers
        let _ = thread::spawn(move || {
            for _ in 0..slice {
                // Create new graph
                let g = Graph::random(n, p);

                // Perform colorings
                let c1 = rs_coloring(&g);
                let c2 = cs_coloring(&g);
                let c3 = lf_coloring(&g);
                let c4 = sl_coloring(&g);

                // Check colorings
                assert!(check_coloring(&g, &c1));
                assert!(check_coloring(&g, &c2));
                assert!(check_coloring(&g, &c3));
                assert!(check_coloring(&g, &c4));

                // Count number of colors used
                let n1 = num_colors(&c1);
                let n2 = num_colors(&c2);
                let n3 = num_colors(&c3);
                let n4 = num_colors(&c4);

                // Send to main thread
                tx_.send((n1,n2,n3,n4)).unwrap();
            }
        });
    }

    // Drop original tx such that rx.iter() will yield `None` when last tx_ is dropped
    drop(tx);

    // Print results
    println!("Random graph with {} vertices and {} edge probability\n", n, p);
    println!("rs\tcs\tlf\tsl");
    println!("---------------------------");

    let mut sum = [0; 4];

    // Iterate over all values received by worker threads
    for (n1, n2, n3, n4) in rx.iter() {
        println!("{}\t{}\t{}\t{}", n1, n2, n3, n4);

        sum[0] += n1;
        sum[1] += n2;
        sum[2] += n3;
        sum[3] += n4;
    }


    println!("---------------------------");
    println!("{}\t{}\t{}\t{}", sum[0] as f32/samples as f32,
             sum[1] as f32/samples as f32, sum[2] as f32/samples as f32,
             sum[3] as f32/samples as f32);
}
