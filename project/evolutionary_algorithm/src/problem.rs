pub trait PopulationInterface {
    fn new(gene_len: usize, num_of_individual: usize) -> Self;
    fn show_best_individual(&self);
    fn show_all(&self);
}

impl PopulationInterface for BasePopulation {
    fn new(gene_len: usize, num_of_individual: usize) -> Self {
        let mut population = Self {
            individuals: Vec::with_capacity(num_of_individual)
        };

        for _ in 0..num_of_individual {
            population.individuals.push(BaseIndividual::new(gene_len));
        }

        population
    }

    fn show_best_individual(&self) {
        let mut best_individual = &self.individuals[0];
        let mut best_value = best_individual.evalate();

        for individual in &self.individuals {
            let value = individual.evalate();
            if best_value > value {
                best_individual = &individual;
                best_value = value;
            }
        }

        println!("{:?}", best_individual);
        println!("Value: {}", best_value);
    }

    fn show_all(&self) {
        for individual in &self.individuals {
            individual.show();
        }
    }
}

pub trait IndividualInterface {
    fn new(gene_len: usize) -> Self;
    fn show(&self);
}

pub trait Solver {
    fn evalate(&self) -> f32;
}

#[derive(Debug)]
pub struct BaseIndividual {
    genes: Vec<f32>
}

#[derive(Debug)]
pub struct BasePopulation {
    individuals: Vec<BaseIndividual>
}

impl Solver for BaseIndividual {
    fn evalate(&self) -> f32{
        let mut value = 0f32;
        for gene in &self.genes {
            value += gene.sqrt();
        }
        value
    }
}

impl IndividualInterface for BaseIndividual {
    fn new(gene_len: usize) -> Self {
        let mut individual = Self { genes: Vec::with_capacity(gene_len) };

        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..gene_len{
            individual.genes.push(rng.gen::<f32>());
        }

        individual
    }
    fn show(&self) {
        println!("{:?}", self);
        println!("Value: {}", self.evalate());
    }
}