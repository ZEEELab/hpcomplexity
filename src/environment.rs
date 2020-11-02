use crate::genotype::Genotype;
use crate::config::Config;
use std::fs::File;
use std::io::Write;

pub struct Environment{
    pub complexity_table: Vec<f64>
}

impl Environment{
    pub fn init() -> Environment {
        Environment{complexity_table: Vec::new()}
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

    pub fn gen_complexity_table(&mut self, cfg: &Config) {
        // Generate the phenotype complexity table
        print!("Generating phenotype complexity table...");
        // Generate population containing ALL possible genotypes
        let mut test_pop = Vec::new();
        for gid in 0..2u64.pow(cfg.gen_len_max){
            test_pop.push(Genotype::new(gid,cfg));
        }
        // Get phenotypes of genotype space
        let ptypes = self.get_pop_ptypes(&test_pop); // This might not be sorted
        
        // Get unique phenotype ids (sort and dedup)
        let mut ptype_ids = ptypes.clone();
        ptype_ids.sort(); // Sort ptype_ids
        ptype_ids.dedup(); // Remove duplicates
        
        // Count frequency of phenotypes
        let ptype_freqs : Vec<usize>= ptype_ids.iter().map(|id| ptypes.iter().filter(|&n| *n == *id).count()).collect();
        //println!("{:?},{:?}",ptype_ids,ptype_freqs); //@DEBUG

        // Calculate complexity of each phenotype id
        let ctable : Vec<f64> = ptype_freqs.iter().map(|x| (test_pop.len() as f64).ln() - (*x as f64).ln()).collect();
        // println!("{:?}",ctable); //@DEBUG

        self.complexity_table = ctable;
        
        println!("Done!");
        // Save complexity table to file
        print!("Writing complexity table to file...");
        let mut cmptable_file = File::create("results/complexity_table.csv").unwrap();
        cmptable_file.write(b"ptype_id,frequency,complexity\n").unwrap();
        for pidx in 0..ptype_ids.len(){
            cmptable_file.write(&format!("{},{},{}\n",ptype_ids[pidx],ptype_freqs[pidx],self.complexity_table[pidx]).as_bytes()).unwrap();
        }
        println!("Done!");
    }
}




