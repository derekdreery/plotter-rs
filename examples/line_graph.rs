extern crate plotter;

fn main() {
    plotter::plot_linegraph([(1.2, 1.0), (1.2, 2.0)].iter()).unwrap();
}
