extern crate graml;
extern crate num_cpus;
extern crate crossbeam;

use std::env;
use std::fs;
use std::thread;
use std::path::Path;

use graml::graph::*;
use graml::coloring::*;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let mut graphs = Vec::new();
    let mut names = Vec::new();

    if args.len() == 1 {
        // Run comparison on this many graphs
        let samples = 50;

        // Number of vertices in each sample graph
        let n = 200;

        // Edge probability in each sample graph
        let p = 0.9;

        graphs = (0..samples).map(move |_| Graph::random(n, p)).collect::<Vec<Graph>>();
        names = (0..samples).map(move |_| format!("Random({},{})", n, p).into()).collect::<Vec<String>>();
    } else {
        let path = &args[1];
        let meta = fs::metadata(path).unwrap();

        if meta.is_dir() {
            // Handle all the graphs
            for file in fs::read_dir(path).unwrap() {
                let file = file.unwrap().path();
                let g = load_graph(file.clone()).unwrap();
                let name = file.file_name().unwrap().to_str().unwrap();

                graphs.push(g);
                names.push(name.into());
            }
        } else {
            let file = Path::new(path);
            let g = load_graph(file.clone()).unwrap();
            let name = file.file_name().unwrap().to_str().unwrap();

            graphs.push(g);
            names.push(name.into());
        }
    }

    parallel_coloring(graphs, names);
}

fn parallel_coloring<G: StaticGraph + Send + 'static>(graphs: Vec<G>, names: Vec<String>) {
    assert_eq!(graphs.len(), names.len());
    let samples = graphs.len();

    // Number of processors
    let workers = num_cpus::get();

    // Communication channel between worker threads and main thread
    let (tx_res, rx_res) = crossbeam::unbounded();
    let (tx_job, rx_job) = crossbeam::unbounded();

    let mut max_width = 0;
    for (graph, name) in graphs.into_iter().zip(names.into_iter()) {
        max_width = max_width.max(name.len());
        tx_job.send((graph, name)).unwrap();
    }

    // Drop original tx_job such that rx_.iter() will yield `None` when all jobs are done
    drop(tx_job);

    for _ in 0..workers {
        // Send results over tx_ to main thread
        let tx_ = tx_res.clone();

        // Wait on rx_ for jobs by main thread
        let rx_ = rx_job.clone();

        // Spawn workers
        thread::spawn(move || {
            for (graph, name) in rx_.iter() {
                // Color graph
                let c = all_colorings(&graph);

                // Send result back to main thread
                tx_.send((c, name)).unwrap();
            }
        });
    }

    // Drop original tx_res such that rx_res.iter() will yield `None` when last tx_ is dropped
    drop(tx_res);

    // Print results
    let spacing = 8;
    println!("{0:<1$}{2:>7$}{3:>7$}{4:>7$}{5:>7$}{6:>7$}\n", "", max_width, "rs", "cs", "lf", "sl", "sdo", spacing);

    let mut sum = [0; 5];

    // Iterate over all values received by worker threads
    for ((n1, n2, n3, n4, n5), name) in rx_res.iter() {
        println!("{0:<1$}{2:>7$}{3:>7$}{4:>7$}{5:>7$}{6:>7$}", name, max_width, n1, n2, n3, n4, n5, spacing);

        sum[0] += n1;
        sum[1] += n2;
        sum[2] += n3;
        sum[3] += n4;
        sum[4] += n5;
    }


    println!("\n{0:<1$}{2:>7$.2}{3:>7$.2}{4:>7$.2}{5:>7$.2}{6:>7$.2}", "", max_width, sum[0] as f32/samples as f32,
             sum[1] as f32/samples as f32, sum[2] as f32/samples as f32,
             sum[3] as f32/samples as f32, sum[4] as f32/samples as f32, spacing);
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
