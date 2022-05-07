use eom::traits::*;
use eom::*;
use ndarray::arr1;
mod single_vibration;

fn main() {
    let dt = 0.01;
    //let eom = ode::Lorenz63::default();
    let eom = single_vibration::Vibration1::default();
    let mut teo = explicit::RK4::new(eom, dt);
    let ts = adaptor::time_series(arr1(&[1.0, 1.0]), &mut teo);
    let end_time = 10000;
    println!("time,u,v,");
    for (t, v) in ts.take(end_time).enumerate() {
        println!("{},{},{}", t as f64 * dt, v[0], v[1]);
    }
}
