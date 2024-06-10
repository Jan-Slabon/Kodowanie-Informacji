extern crate rand;
use rand::{distributions::Uniform, Rng};
use std::ops::{Add, Index, Mul};

extern crate image;
use image::{ImageBuffer, Rgb};

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn get_value_safe<T : Index<usize>>(container : &T, index : i32, bound : usize, default : T::Output) -> T::Output
where <T as Index<usize>>::Output: Sized + Copy
{
    if index >= bound as i32 || index < 0 { default } else { container[index as usize] }
}
pub fn ising_hamiltonian<const N: usize>(lattice: &[i32; N], index: usize) -> f64 {
    let weigth_params = (1, 1);
    let mut energy: i32 = 0;

    energy += 
        weigth_params.0 * get_value_safe(lattice, index as i32 - 1, N, 0) * lattice[index] +
        weigth_params.0 * get_value_safe(lattice, (index + 1) as i32, N, 0) * lattice[index] +
        weigth_params.1 * get_value_safe(lattice, index as i32 - WIDTH as i32, N, 0) * lattice[index] +
        weigth_params.1 * get_value_safe(lattice, (index + WIDTH) as i32, N, 0) * lattice[index];

    -10. * energy as f64
}
pub fn continous_ising_hamiltonian<const N: usize>(lattice: &[f64; N], index: usize) -> f64 {
    let weigth_params = (1., 1.);
    let mut energy: f64 = 0.;

    energy += 
        weigth_params.0 * get_value_safe(lattice, index as i32 - 1, N, 0.) * lattice[index] +
        weigth_params.0 * get_value_safe(lattice, (index + 1) as i32, N, 0.) * lattice[index] +
        weigth_params.1 * get_value_safe(lattice, index as i32 - WIDTH as i32, N, 0.) * lattice[index] +
        weigth_params.1 * get_value_safe(lattice, (index + WIDTH) as i32, N, 0.) * lattice[index];

    -10. * energy
}
pub fn general_gibbs_hamiltonian<const N: usize>(lattice: &[f64; N], index: usize) -> f64 {
    let weigth_params = (3., 1., 5., 1.);
    let mut energy: f64 = 0.;

    energy += 
        weigth_params.0 * get_value_safe(lattice, index as i32 - 1, N, 0.) * lattice[index] +
        weigth_params.0 * get_value_safe(lattice, (index + 1) as i32, N, 0.) * lattice[index] +
        weigth_params.1 * get_value_safe(lattice, index as i32 - WIDTH as i32, N, 0.) * lattice[index] +
        weigth_params.1 * get_value_safe(lattice, (index + WIDTH) as i32, N, 0.) * lattice[index] +
        weigth_params.2 * get_value_safe(lattice, index as i32 - WIDTH as i32 - 1, N, 0.) * lattice[index] +
        weigth_params.2 * get_value_safe(lattice, (index + WIDTH + 1) as i32, N, 0.) * lattice[index] +
        weigth_params.3 * get_value_safe(lattice, index as i32 - WIDTH as i32 + 1, N, 0.) * lattice[index] +
        weigth_params.3 * get_value_safe(lattice, (index + WIDTH - 1) as i32, N, 0.) * lattice[index];

    -10. * energy
}
pub fn metropolis_hastings_discrete_single_flip<const N: usize>(H: fn(&[i32; N], usize) -> f64, beta: f64) -> [u8; N] {
    let mut rng = rand::thread_rng();
    let mut latice: [i32; N] = [0; N].map(|_: i32| if rng.gen_bool(0.5) { 1 } else { -1 });
    let mut debug_counter = 0;
    let mut negative_flip_counter = 0;
    let iter = 1_000_000;
    for _ in 0..iter {
        let next_flip = rng.gen_range(0..N);
        let mut new_latice = latice;
        let mut negative_flipped: bool = false;
        new_latice[next_flip] = if latice[next_flip] == -1 {
            negative_flipped = true;
            1
        } else {
            -1
        };
        let energy_difference = H(&new_latice, next_flip) - H(&latice, next_flip);
        let p = (-beta * energy_difference).exp();
        if energy_difference < 0. {
            latice = new_latice;
            if negative_flipped {
                negative_flip_counter += 1;
            }
            debug_counter += 1;
        } else {
            let sample: bool = rng.gen_bool(p);
            if sample {
                if negative_flipped {
                    negative_flip_counter += 1;
                }
                latice = new_latice;
                debug_counter += 1;
            }
        }
    }
    println!(
        "Amout of aproved flips {}, amout of flips that were on -1 {}",
        debug_counter as f32 / iter as f32,
        negative_flip_counter as f32 / iter as f32
    );
    let final_latice = latice.map(|x : i32| if x == 1 {1 as u8} else {0 as u8});
    return final_latice;
}

pub fn metropolis_hastings_continous_single_flip<const N: usize>(H: fn(&[f64; N], usize) -> f64, beta: f64) -> [u8; N] {
    let mut rng = rand::thread_rng();
    let mut latice: [f64; N] = [0.0; N].map(|_| rng.sample(Uniform::new(-1., 1.)));
    let mut debug_counter = 0;

    let iter = 1_000_000;
    for _ in 0..iter {
        let next_flip = rng.gen_range(0..N);
        let mut new_latice = latice;

        new_latice[next_flip] = rng.sample(Uniform::new(-1.0, 1.0));

        let energy_difference = H(&new_latice, next_flip) - H(&latice, next_flip);
        let p = (-beta * energy_difference).exp();
        if energy_difference < 0. {
            latice = new_latice;
            debug_counter += 1;
        } else {
            let sample: bool = rng.gen_bool(p);
            if sample {
                latice = new_latice;
                debug_counter += 1;
            }
        }
    }
    println!(
        "Amout of aproved flips {}",
        debug_counter as f32 / iter as f32
    );
    let final_latice = latice.map(|x : f64|{
        (128. * ((x + 2.)/2.)) as u8
    });
    return final_latice;
}

pub fn gen_sampled_distribution<D>(monte_carlo_generator : fn( fn(&[D; WIDTH * HEIGHT], usize) -> f64, f64) -> [u8; WIDTH * HEIGHT],
                                                                        hamiltonian : fn(&[D; WIDTH * HEIGHT], usize) -> f64) -> ()
{
    let final_configuration = monte_carlo_generator(hamiltonian, 1.0);
    let mut image = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let value = final_configuration[i * HEIGHT + j] as u8;
            let pixel = Rgb([value, value, value] as [u8; 3]); 
            image.put_pixel(i as u32, j as u32, pixel);
        }
    }
    // write it out to a file
    image.save("IsingModelContinous.png").unwrap();
}