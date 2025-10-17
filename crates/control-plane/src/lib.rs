use data_plane::Sum;

fn run(){

    let sum:Sum = Sum::new(1,2);

    println!("{}", Sum::compute(&sum));
}
