use crate::genotype::Genotype;

pub struct Environment;

impl Environment{
    pub fn init() -> Environment { // Returns an empty struct
        Environment{}
    }

    pub fn g_to_ptype_id(&self, geno: &Genotype) -> u64 { // Main function reponsible for GP partitioning
        let gtype_id = geno.get_as_integer(); // Genotype id (gtype converted to integer)
        if gtype_id < 16 {return 0 as u64}
        else if gtype_id < 24 {return 1 as u64}
        else if gtype_id < 30 {return 2 as u64}
        else {return 3 as u64};
    }

    pub fn get_pop_ptypes(&self,pop: &Vec<Genotype>) -> Vec<u64> {
        let pop_ptypes = pop.iter().map(|x| self.g_to_ptype_id(x)).collect();
        pop_ptypes
    }
}




