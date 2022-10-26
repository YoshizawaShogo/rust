pub trait PopulationInterface {
    fn new(genes_len: usize, num_of_individual: usize) -> Self;
    fn show_best_individual(&self);
    fn show_all(&self);
}

impl PopulationInterface for BasePopulation {
    fn new(genes_len: usize, num_of_individual: usize) -> Self {
        let mut population = Self {
            individuals: Vec::with_capacity(num_of_individual),
        };

        for _ in 0..num_of_individual {
            population
                .individuals
                .push(BaseIndividual::new_from_len(genes_len));
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

pub trait PopulationEvolution {
    fn evolve(&mut self, f_scale: f32, crossover_rate: f32, epoch: usize);
}

impl PopulationEvolution for BasePopulation {
    fn evolve(&mut self, f_scale: f32, crossover_rate: f32, epoch: usize) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let individual_len = self.individuals.len();
        

        for _ in 0..epoch {
            let mut next_poplation = Vec::new();
            for individual in self.individuals.iter() {
                let mut factor_nums = Vec::new();
                while factor_nums.len() != 3 {
                    let rand_i = rng.gen_range(0, individual_len);
                    if !factor_nums.contains(&rand_i) {
                        factor_nums.push(rand_i);
                    }
                }
                let mut factors = Vec::new();
                for i in factor_nums {
                    factors.push(self.individuals[i].clone());
                }
                next_poplation.push(individual.evolve(&factors, f_scale, crossover_rate));
            }
            self.individuals = next_poplation;
        }
    }
}

pub trait IndividualEvolution {
    fn evolve(
        &self,
        factors: &Vec<BaseIndividual>,
        f_scale: f32,
        crossover_rate: f32,
    ) -> BaseIndividual;
}

impl IndividualEvolution for BaseIndividual {
    fn evolve(
        &self,
        factors: &Vec<BaseIndividual>,
        f_scale: f32,
        crossover_rate: f32,
    ) -> BaseIndividual {
        fn mutate(factors: &Vec<BaseIndividual>, f_scale: f32) -> BaseIndividual {
            assert!(factors.len() == 3);
            let genes_len = factors[0].genes.len();
            let mut genes = Vec::with_capacity(genes_len);
            for i in 0..genes_len {
                genes.push(
                    factors[0].genes[i] + f_scale * (factors[1].genes[i] - factors[2].genes[i])
                );
            }
            //println!("{:?}", genes);
            BaseIndividual::new_from_genes(genes)
        }

        fn cross(
            parent: &BaseIndividual,
            mutant_individual: BaseIndividual,
            crossover_rate: f32,
        ) -> BaseIndividual {
            use rand::Rng;
            let mut rng = rand::thread_rng();

            let genes_len = parent.genes.len();
            let mut genes = Vec::with_capacity(genes_len);
            let cross_i = rng.gen_range(0, genes_len);


            for i in 0..genes_len {
                genes.push( if rng.gen::<f32>() > crossover_rate  || i == cross_i {
                    mutant_individual.genes[i]
                    // if mutant_individual.genes[i] > 1.0 {

                    // } else if mutant_individual.genes[i] < 0.0 {

                    // }
                } else {
                    parent.genes[i]
                });
            }
            BaseIndividual::new_from_genes(genes)
        }

        fn compete(parent: BaseIndividual, trial_individual: BaseIndividual) -> BaseIndividual {
            if parent.evalate() < trial_individual.evalate() {
                parent
            } else {
                trial_individual
            }
        }

        let parent = self.clone();
        let mutant_individual = mutate(factors, f_scale);
        let trial_individual = cross(&parent, mutant_individual, crossover_rate);
        compete(parent, trial_individual)
    }
}

pub trait IndividualInterface {
    fn new_from_len(genes_len: usize) -> Self;
    fn new_from_genes(genes: Vec<f32>) -> Self;
    fn show(&self);
}

pub trait IndividualSolver {
    fn evalate(&self) -> f32;
}

#[derive(Debug, Clone)]
pub struct BaseIndividual {
    genes: Vec<f32>,
}

#[derive(Debug)]
pub struct BasePopulation {
    individuals: Vec<BaseIndividual>,
}

impl IndividualSolver for BaseIndividual {
    fn evalate(&self) -> f32 {
        let mut value = 0f32;
        for gene in &self.genes {
            value += gene.powi(2);
        }
        value
    }
}

impl IndividualInterface for BaseIndividual {
    fn new_from_len(genes_len: usize) -> Self {
        let mut individual = Self {
            genes: Vec::with_capacity(genes_len),
        };

        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..genes_len {
            individual.genes.push(rng.gen::<f32>());
        }

        individual
    }

    fn new_from_genes(genes: Vec<f32>) -> Self {
        Self { genes }
    }

    fn show(&self) {
        println!("{:?}", self);
        println!("Value: {}", self.evalate());
    }
}
