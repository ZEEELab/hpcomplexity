use crate::config::Config;
use crate::genotype::Genotype;
use crate::environment::Environment;
use crate::ga::ga_step;
use std::fs::File;
use std::io::Write;
use std::error::Error;

pub struct World{
    pub pop_host : Vec<Genotype>,
    pub pop_para : Vec<Genotype>,
    pub env: Environment
}

impl World{
    pub fn init(cfg: &Config) -> Result<World, Box<dyn Error>> {
        let mut new_hpop = Vec::new();
        let mut new_ppop = Vec::new();
        
        let new_env = Environment::init(); // This environment doesn't have a complexity table!

        for _ in 0..cfg.popsize_host{
            new_hpop.push(Genotype::new_rand(cfg));
        }
        for _ in 0..cfg.popsize_para{
            new_ppop.push(Genotype::new_rand(cfg));
        }
        //println!("{:?}",new_hpop); //@DEBUG
        //println!("{:?}",new_ppop); //@DEBUG
        Ok(World{pop_host:new_hpop,pop_para:new_ppop,env:new_env})
    }

    pub fn start_run(mut self, cfg: &Config) { // Main driver function

        self.env.gen_complexity_table(cfg); // Environment now has complexity table

        println!("Starting run...");
        // Do steps of the run
        for step in 0..(cfg.max_steps+1) {  
            println!("Step : {} out of {}",step,cfg.max_steps);
            // First, get fitness of both host and parasite populations
            let (hfit,pfit) = self.get_hp_fitness(cfg);

            // Every timestep consists of two ga optimization steps
            // 1. Optimization of host population
            self.pop_host = ga_step(self.pop_host,hfit,&cfg.mut_host); 
            // 2. Optimization of parasite population
            self.pop_para = ga_step(self.pop_para,pfit,&cfg.mut_para); 

            if step%cfg.save_every==0{
                self.save_state(&step);
            }
        }
    }

    fn get_hp_fitness(&self, cfg : &Config) -> (Vec<f64>,Vec<f64>) {
        // Get list of host and parasite population phenotypes
        let hptypes = self.env.get_pop_ptypes(&self.pop_host); 
        let pptypes = self.env.get_pop_ptypes(&self.pop_para);

        let mut hfit = Vec::new();
        let mut pfit = Vec::new();
        

        // OPTIMIZE THIS PART, DOING DOUBLE THE REQUIRED WORK!!!

        // For every host genotype, if matching parasite phenotype, decrease fitness
        for idxh in 0..hptypes.len(){
            if pptypes.contains(&hptypes[idxh]){
                hfit.push(1.0-cfg.para_effect);
            }
            else{
                hfit.push(1.0);
            }
        }

        // for every parasite genotype, if matching host phenotype, add fitness advantage
        for idxp in 0..pptypes.len(){
            if hptypes.contains(&pptypes[idxp]){
                pfit.push(cfg.para_self_fit+cfg.para_perm_fit);
            }
            else{
                pfit.push(cfg.para_self_fit);
            }
        }

        (hfit,pfit)
    }

    fn save_state(&self, step: &u32){ // Saves current world state
        let mut hpop_file = File::create(format!("results/hpop_{}.csv",step)).unwrap();
        hpop_file.write(b"genotype,ptype_id\n").unwrap();
        let hstring : String = self.pop_host.iter().map(|x| x.get_as_string()+&format!(",{}\n",self.env.g_to_ptype_id(x))).collect();
        hpop_file.write(hstring.as_bytes()).unwrap();

        let mut ppop_file = File::create(format!("results/ppop_{}.csv",step)).unwrap();
        ppop_file.write(b"genotype,ptype_id\n").unwrap();
        let pstring : String = self.pop_para.iter().map(|x| x.get_as_string()+&format!(",{}\n",self.env.g_to_ptype_id(x))).collect();
        ppop_file.write(pstring.as_bytes()).unwrap();
    }
}