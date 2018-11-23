extern crate graml;
extern crate num_cpus;

use std::thread;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::mpsc;

use graml::graph::*;
use graml::coloring::*;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        // Run comparison on this many graphs
        let samples = 50;

        // Number of vertices in each sample graph
        let n = 200;

        // Edge probability in each sample graph
        let p = 0.9;

        comparison(samples, n, p);
    } else {
        let path = &args[1];
        let meta = fs::metadata(path).unwrap();

        if meta.is_dir() {
            // Handle all the graphs
            for file in fs::read_dir(path).unwrap() {
                let file = file.unwrap().path();
                let g = load_graph(file.clone()).unwrap();

                println!("Graph {} with {} vertices\n", file.file_name().unwrap().to_str().unwrap(), g.num_vertices());

                let (n1, n2, n3, n4, n5) = all_colorings(&g);

                println!("rs\tcs\tlf\tsl\tsdo");
                println!("{}\t{}\t{}\t{}\t{}\n", n1, n2, n3, n4, n5);
            }
        } else {
            let file = Path::new(path);
            let g = load_graph(file.clone()).unwrap();

            println!("Graph {} with {} vertices\n", file.file_name().unwrap().to_str().unwrap(), g.num_vertices());

            let (n1, n2, n3, n4, n5) = all_colorings(&g);

            println!("rs\tcs\tlf\tsl\tsdo\n");
            println!("{}\t{}\t{}\t{}\t{}", n1, n2, n3, n4, n5);

        }
    }
}

fn comparison(samples: usize, n: usize, p: f32) {
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

                let c = all_colorings(&g);

                // Send to main thread
                tx_.send(c).unwrap();
            }
        });
    }

    // Drop original tx such that rx.iter() will yield `None` when last tx_ is dropped
    drop(tx);

    // Print results
    println!("Random graph with {} vertices and {} edge probability\n", n, p);
    println!("rs\tcs\tlf\tsl\tsdo\n");

    let mut sum = [0; 5];

    // Iterate over all values received by worker threads
    for (n1, n2, n3, n4, n5) in rx.iter() {
        println!("{}\t{}\t{}\t{}\t{}", n1, n2, n3, n4, n5);

        sum[0] += n1;
        sum[1] += n2;
        sum[2] += n3;
        sum[3] += n4;
        sum[4] += n5;
    }


    println!("\n{}\t{}\t{}\t{}\t{}", sum[0] as f32/samples as f32,
             sum[1] as f32/samples as f32, sum[2] as f32/samples as f32,
             sum[3] as f32/samples as f32, sum[4] as f32/samples as f32);
}

fn all_colorings<G: StaticGraph>(g: &G) -> (usize, usize, usize, usize, usize) {
    // Perform colorings
    let c1 = rs_coloring(g);
    let c2 = cs_coloring(g);
    let c3 = lf_coloring(g);
    let c4 = sl_coloring(g);
    let c5 = sdo_coloring(g);

    // Check colorings
    assert!(check_coloring(g, &c1));
    assert!(check_coloring(g, &c2));
    assert!(check_coloring(g, &c3));
    assert!(check_coloring(g, &c4));
    assert!(check_coloring(g, &c5));

    // Count number of colors used
    let n1 = num_colors(&c1);
    let n2 = num_colors(&c2);
    let n3 = num_colors(&c3);
    let n4 = num_colors(&c4);
    let n5 = num_colors(&c5);

    (n1, n2, n3, n4, n5)
}
