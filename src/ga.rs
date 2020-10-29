use crate::genotype::Genotype;
use rand::Rng;


pub fn ga_step(pop: Vec<Genotype>,mut fit: Vec<f64>,mutrate: &f64) -> Vec<Genotype> {
    // normalise fitnesses
    fit = normalise(fit);

    // create new population
    let mut new_pop = Vec::new();

    // Pick from population with probability prop. to fitness
    // and mutate and add to new_pop
    for _ in 0..pop.len() {
        let mut geno = pop[idx_from_prob_dist(&fit)].clone();
        geno.mutate(mutrate);
        new_pop.push(geno);
    }

    new_pop
}

pub fn normalise(vec: Vec<f64>) -> Vec<f64> {
    let sum: f64 = vec.iter().sum();
    vec.iter().map(|x| x/sum).collect()
}

pub fn idx_from_prob_dist(probs: &Vec<f64>) -> usize {
    let rn : f64 = rand::thread_rng().gen();
    let mut sum = 0.0;
    for idx in 0..probs.len(){
        sum += probs[idx];
        if rn <= sum {
            return idx;} 
    }
    1
}