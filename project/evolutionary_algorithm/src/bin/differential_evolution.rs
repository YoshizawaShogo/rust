use evolutionary_algorithm::problem::{BaseIndividual, IndividualInterface, BasePopulation, PopulationInterface};



fn main() {
    let tmp = BasePopulation::new(10, 5);
    tmp.show_all();
    tmp.show_best_individual()
}

