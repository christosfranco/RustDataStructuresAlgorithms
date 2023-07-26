use criterion::{black_box, criterion_group, criterion_main, Criterion};
// mod quicksort;
use gnuplot::*;
use DataStructuresAlgorithms::quicksort::quicksort::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    // pre setup
    let mut vec = random_vector(1 * 10 ^ 3);

    // this is the only timed function
    c.bench_function("quicksort 20", |b| {
        b.iter(|| quick_sort(black_box(&mut vec)))
    });

    //validation of correctness
    assert_eq!(true, check_sorted(&vec));
}

// Main function
fn main() {
    // Run the benchmark and save the results
    let mut criterion = Criterion::default().with_plots();
    criterion_benchmark(&mut criterion);
    criterion.final_summary();

    // let mut fg = Figure::new();
    // fg.axes2d()
    //     .set_title("A plot", &[])
    //     .set_legend(Graph(0.5), Graph(0.9), &[], &[])
    //     .set_x_label("x", &[])
    //     .set_y_label("y^2", &[])
    //     .lines(
    //         &[-3., -2., -1., 0., 1., 2., 3.],
    //         &[9., 4., 1., 0., 1., 4., 9.],
    //         &[Caption("Parabola")],
    //     );
    // fg.show().unwrap();
}

criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
