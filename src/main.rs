mod dynamic_time_warp;

use dynamic_time_warp::dynamic_time_warp;

fn main() {
    let dtw = dynamic_time_warp(&[7, 5, 5, 1, 5, 3], &[4, 2, 2, 2, 2]);
    println!("{:?}", dtw);
    println!("{}", dtw.rows().into_iter().nth(0).unwrap());
    println!("{}", dtw.rows().into_iter().nth(1).unwrap());
    println!("{}", dtw.rows().into_iter().nth(2).unwrap());
    println!("{}", dtw.rows().into_iter().nth(3).unwrap());
    println!("{}", dtw.rows().into_iter().nth(4).unwrap());
    println!("{}", dtw.rows().into_iter().nth(5).unwrap());
}
