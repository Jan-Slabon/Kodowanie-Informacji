mod Fibbonaci;
use metropolis_hastings::{continous_ising_hamiltonian, gen_sampled_distribution, general_gibbs_hamiltonian, ising_hamiltonian, metropolis_hastings_continous_single_flip, metropolis_hastings_discrete_single_flip};

use crate::Fibbonaci::{fibbonaci_code, fibbonaci_decode};

mod metropolis_hastings;
fn main() {
    gen_sampled_distribution(metropolis_hastings_continous_single_flip, general_gibbs_hamiltonian);
}
