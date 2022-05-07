use eom::traits::*;
use eom::*;
use ndarray::arr1;
mod double_vibration;
mod single_vibration;

fn main() {
    let dt = 0.01;
    //let eom = ode::Lorenz63::default();
    //let eom = single_vibration::Vibration1::default();
    let eom = double_vibration::Vibration1::default();
    let mut teo = explicit::RK4::new(eom, dt);
    let ts = adaptor::time_series(arr1(&[0., 0.2, 0., 0.4]), &mut teo);
    let end_time = 10000;
    println!("time,v_1,u_1,v_2,u_2");
    for (t, v) in ts.take(end_time).enumerate() {
        println!("{},{},{},{},{}", t as f64 * dt, v[0], v[1], v[2], v[3]);
    }
}
