use crate::genotype::Genotype;
use crate::config::Config;
use std::boxed::Box;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::collections::HashMap;
use std::error::Error;

pub struct Environment{
    pub complexity_table: Vec<f64>, // Generate by environment.rs from gp_table
    pub gp_table: HashMap<u64,u64>
}

impl Environment{
    pub fn init(cfg:&Config, gpfilename: &String) -> Result<Environment,Box<dyn Error>> {
        let mut gp_map = HashMap::new();
        let gpfile_content = fs::read_to_string(gpfilename)?; // The ? here returns an error if file cannot be opened

        for line in gpfile_content.lines().skip(1){
            let as_vec : Vec<&str> = line.split(",").collect();
            if as_vec.len()<2{
                continue;
            }
            if u64::from_str_radix(as_vec[0],2).unwrap()>=2u64.pow(cfg.gen_len_max){
                let er: Box<dyn Error> = String::from("Genotype in gptable over limit").into();
                return Err(er);
            }
            gp_map.insert(u64::from_str_radix(as_vec[0],2).unwrap(),as_vec[1].parse::<u64>().unwrap());
        }

        Ok(Environment{complexity_table: Vec::new(),gp_table: gp_map})
    }

    pub fn g_to_ptype_id(&self, geno: &Genotype) -> u64 { // Main function reponsible for GP partitioning
        let gtype_id = geno.get_as_integer(); // Genotype id (gtype converted to integer)
        
        // Uncomment these for automated gptable generation
        //if gtype_id < 16 {return 0 as u64}
        //else if gtype_id < 24 {return 1 as u64}
        //else if gtype_id < 30 {return 2 as u64}
        //else {return 3 as u64};
        self.gp_table[&gtype_id]
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




