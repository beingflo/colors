extern crate graml;
extern crate num_cpus;

use std::thread;
use std::sync::mpsc;

use graml::graph::*;
use graml::coloring::*;

fn main() {
    let samples = 50;
    let n = 200;
    let p = 0.5;

    let workers = num_cpus::get();
    let (tx, rx) = mpsc::channel();

    for _ in 0..workers {
        let slice = samples / workers;
        let tx_ = tx.clone();

        let _ = thread::spawn(move || {
            for _ in 0..slice {
                let g = AdjMatrix::random(n, p);

                let c1 = rs_coloring(&g);
                let c2 = cs_coloring(&g);
                let c3 = lf_coloring(&g);
                let c4 = sl_coloring(&g);

                let n1 = num_colors(&c1);
                let n2 = num_colors(&c2);
                let n3 = num_colors(&c3);
                let n4 = num_colors(&c4);

                tx_.send((n1,n2,n3,n4)).unwrap();
            }
        });
    }

    // Drop original tx such that rx.iter() will yield None when last tx_ is dropped
    drop(tx);

    println!("rs\tcs\tlf\tsl");

    let mut sum = [0; 4];
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
