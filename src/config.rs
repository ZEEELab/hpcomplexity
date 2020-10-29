use std::fmt;

pub struct Config {
    pub popsize_host : u32,
    pub popsize_para : u32, 

    pub gen_len_max : u32,
    pub init_len : u32,
    
    pub srca_host : f64,
    pub srcb_host : f64,

    pub srca_para : f64,
    pub srcb_para : f64,

    pub para_effect : f64,
    pub para_self_fit : f64,
    pub para_perm_fit : f64,
    
    pub mut_host : f64,
    pub mut_para : f64, 

    pub max_steps : u32,
    pub save_every : u32
}

impl Config {
    pub fn init() -> Config {
        Config{
            // ############ PARAMETERS ###########
            popsize_host: 100, //Size of host population
            popsize_para: 100, //Size of parasite population

            gen_len_max: 5, //Maximum length of genotype (for both host and parasite)
            init_len: 1, // Initial length of genotypes (0s in front)

            //Sequence replication cost, hosts (cost = srca*length + srcb)
            srca_host: 0.0, 
            srcb_host: 1.0,

            //Sequence replication cost, parasites (cost = srca*length + srcb)
            srca_para: 0.0,
            srcb_para: 1.0,

            //Parasite effect on host
            para_effect : 0.8, //This fraction of host fitness goes to parasite if matching parasite present
            para_self_fit : 0.1, //If non zero, parasites can exist freely (relative magnitude wrt para_perm_fit important)
            para_perm_fit : 0.5, //Additional fitness advantage when a permissive host exists

            // Genetic algorithm parameters
            mut_host: 0.002, // Mutation probability for hosts
            mut_para: 0.002, // Mutation probability for parasites

            // Run time
            max_steps: 1000,
            save_every: 1000
        }
    }
}

impl Default for Config{
    fn default() -> Config {
        Config{
            popsize_host: 0,
            popsize_para: 0, 
            gen_len_max: 0,
            init_len: 0, 
            srca_host: 0.0, 
            srcb_host: 1.0,
            srca_para: 0.0,
            srcb_para: 1.0,
            para_effect: 0.0,
            para_self_fit: 0.0,
            para_perm_fit: 0.0,
            mut_host: 0.00, 
            mut_para: 0.00, 
            max_steps: 0,
            save_every: 0
        }
    }
}

impl fmt::Display for Config{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.popsize_host,
            self.popsize_para,
            self.gen_len_max,
            self.init_len,
            self.srca_host,
            self.srcb_host,
            self.srca_para,
            self.srcb_para,
            self.para_effect,
            self.para_self_fit,
            self.para_perm_fit,
            self.mut_host,
            self.mut_para,
            self.max_steps,
            self.save_every)
    }
}