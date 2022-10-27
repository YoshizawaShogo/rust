use evolutionary_algorithm::problem::population::{individual::*, *, PopulationBaseEvolution};


fn main() {
    let mut population = BasePopulation::<Sphere<f64>>::new_from_shape(100, 20);
    
    population.advance_epoch(1000, "best", 1, 0.5, 0.9);
    population.show_best_individual();

}

