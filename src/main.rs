use std::env;
use std::fs;
use std::path::Path;
use std::thread;

use graml::coloring::*;
use graml::graph::*;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let graphs = if args.len() == 1 {
        // Run comparison on this many graphs
        let samples = 50;

        // Number of vertices in each sample graph
        let n = 200;

        // Edge probability in each sample graph
        let p = 0.9;

        (0..samples)
            .map(move |_| JobType::Random(n, p, format!("Random({},{})", n, p)))
            .collect::<Vec<JobType>>()
    } else {
        let path = &args[1];
        let meta = fs::metadata(path).unwrap();
        let mut graphs = Vec::new();

        if meta.is_dir() {
            // Handle all the graphs
            for file in fs::read_dir(path).unwrap() {
                let file = file.unwrap().path();
                graphs.push(JobType::File(file.as_path().to_str().unwrap().to_string()));
            }
        } else {
            graphs.push(JobType::File(path.to_string()));
        }

        graphs
    };

    parallel_coloring(graphs);
}

#[derive(Debug, Clone)]
enum JobType {
    Random(usize, f32, String),
    File(String),
}

fn parallel_coloring(graphs: Vec<JobType>) {
    let samples = graphs.len();

    // Number of processors
    let workers = num_cpus::get();

    // Communication channel between worker threads and main thread
    let (tx_res, rx_res) = crossbeam::unbounded();
    let (tx_job, rx_job) = crossbeam::unbounded();

    for graph in graphs.into_iter() {
        tx_job.send(graph).unwrap();
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
            for graph in rx_.iter() {
                let name;
                let g = match graph {
                    JobType::Random(n, p, gname) => {
                        name = gname;
                        Graph::random(n, p)
                    }
                    JobType::File(ref gname) => {
                        let file = Path::new(&gname);
                        name = file.file_name().unwrap().to_str().unwrap().to_string();
                        load_graph(file).unwrap()
                    }
                };

                // Color graph
                let c = all_colorings(&g);

                // Send result back to main thread
                tx_.send((c, name)).unwrap();
            }
        });
    }

    // Drop original tx_res such that rx_res.iter() will yield `None` when last tx_ is dropped
    drop(tx_res);

    // Print results
    let spacing = 8;
    let width = 20;
    println!(
        "{0:<1$}{3:>2$}{4:>2$}{5:>2$}{6:>2$}{7:>2$}\n",
        "", width, spacing, "rs", "cs", "lf", "sl", "sdo"
    );

    let mut sum = vec![0; 6];

    // Iterate over all values received by worker threads
    for (n, name) in rx_res.iter() {
        println!(
            "{0:<1$}{3:>2$}{4:>2$}{5:>2$}{6:>2$}{7:>2$}",
            name, width, spacing, n[0], n[1], n[2], n[3], n[4]
        );

        sum[0] += n[0];
        sum[1] += n[1];
        sum[2] += n[2];
        sum[3] += n[3];
        sum[4] += n[4];
    }

    println!(
        "\n{0:<1$}{3:>2$.2}{4:>2$.2}{5:>2$.2}{6:>2$.2}{7:>2$.2}",
        "",
        width,
        spacing,
        sum[0] as f32 / samples as f32,
        sum[1] as f32 / samples as f32,
        sum[2] as f32 / samples as f32,
        sum[3] as f32 / samples as f32,
        sum[4] as f32 / samples as f32,
    );
}

fn all_colorings<G: StaticGraph>(g: &G) -> Vec<usize> {
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

    vec![n1, n2, n3, n4, n5]
}
