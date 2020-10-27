use std::fmt;

pub struct Config {
    pub popsize_host : u32,
    pub popsize_para : u32, 

    pub gen_len_max : u32,
    
    pub srca_host : f64,
    pub srcb_host : f64,

    pub srca_para : f64,
    pub srcb_para : f64,

    
    pub mut_host : f64,
    pub mut_para : f64, 
}

impl Config {
    pub fn init() -> Config {
        Config{
            // ############ PARAMETERS ###########
            popsize_host: 100, //Size of host population
            popsize_para: 100, //Size of parasite population

            gen_len_max: 100, //Maximum length of genotype (for both host and parasite)

            //Sequence replication cost, hosts (cost = srca*length + srcb)
            srca_host: 0.0, 
            srcb_host: 1.0,

            //Sequence replication cost, parasites (cost = srca*length + srcb)
            srca_para: 0.0,
            srcb_para: 1.0,

            // Genetic algorithm parameters
            mut_host: 0.002, // Mutation probability for hosts
            mut_para: 0.002, // Mutation probability for parasites
        }
    }
}

impl Default for Config{
    fn default() -> Config {
        Config{
            popsize_host: 0,
            popsize_para: 0, 
            gen_len_max: 0, 
            srca_host: 0.0, 
            srcb_host: 1.0,
            srca_para: 0.0,
            srcb_para: 1.0,
            mut_host: 0.00, 
            mut_para: 0.00, 
        }
    }
}

impl fmt::Display for Config{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{},{},{},{},{},{},{},{},{}",
            self.popsize_host,
            self.popsize_para,
            self.gen_len_max,
            self.srca_host,
            self.srcb_host,
            self.srca_para,
            self.srcb_para,
            self.mut_host,
            self.mut_para)
    }
}