extern crate rand;

mod graph;
mod coloring;

pub use graph::*;
pub use coloring::*;

fn main() {
    let mut sum = [0; 4];
    let n = 20;

    println!("rs\tcs\tlf\tsl");
    for _ in 0..n {
        let g = EdgeList::random(200, 0.9);

        let c1 = rs_coloring(&g);
        let c2 = cs_coloring(&g);
        let c3 = lf_coloring(&g);
        let c4 = sl_coloring(&g);

        let n1 = num_colors(&c1);
        let n2 = num_colors(&c2);
        let n3 = num_colors(&c3);
        let n4 = num_colors(&c4);

        sum[0] += n1;
        sum[1] += n2;
        sum[2] += n3;
        sum[3] += n4;

        println!("{}\t{}\t{}\t{}", n1, n2, n3, n4);
    }

    println!("---------------------------");
    println!("{}\t{}\t{}\t{}", sum[0] as f32/n as f32,
             sum[1] as f32/n as f32, sum[2] as f32/n as f32,
             sum[3] as f32/n as f32);
}
