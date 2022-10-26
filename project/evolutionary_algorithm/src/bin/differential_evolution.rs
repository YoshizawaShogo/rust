use evolutionary_algorithm::problem::{BasePopulation, PopulationInterface, PopulationEvolution};



fn main() {
    let mut tmp = BasePopulation::new(10, 10);
    tmp.show_all();
    tmp.show_best_individual();
    println!();

    tmp.evolve(0.5, 0.9, 1000);

    println!();
    tmp.show_all();
    tmp.show_best_individual();
}

