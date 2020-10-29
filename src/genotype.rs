use crate::config::Config;
use rand::Rng;
use std::fmt;
#[derive(Clone)]
pub struct Genotype{
    pub sequence : Vec<u32>
}

impl Genotype{
    pub fn new_rand(cfg: &Config) -> Genotype {
        let mut seq = Vec::new();

        let mut rng = rand::thread_rng();
        for idx in 0..cfg.gen_len_max {
            if idx < cfg.init_len {
                seq.push(rng.gen_range(0 as u32,2 as u32));
            }
            else {
                seq.push(0 as u32);
            }
        }
        Genotype{sequence:seq}
    }

    pub fn get_func_length(&self) -> u32 { // Returns the functional length of the genotype (sans ending zeroes)
        for idx in (0..self.sequence.len()).rev() {
            if self.sequence[idx]==1{
                return (idx as u32)+1;
            } 
        }
        return 0 as u32;
    }

    pub fn get_as_integer(&self) -> u64 {
        let mut asint = 0 as u64;
        for idx in 0..(self.get_func_length()) {
            asint += (self.sequence[idx as usize] as u64) * 2u64.pow(idx);
        }
        asint
    }

    pub fn get_as_string(&self) -> String {
        let buf = self.sequence.iter().map(|i| i.to_string()).rev().collect::<String>();
        buf
    }

    pub fn mutate(&mut self,mutrate: &f64) {
        let mut rng = rand::thread_rng();
        for idx in 0..self.sequence.len(){
            let rn : f64 = rng.gen();
            if rn < *mutrate { self.sequence[idx] = 1 - self.sequence[idx]; } 
        }
    } 
}

impl fmt::Debug for Genotype{
    fn fmt(& self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"{:?}",self.sequence)
    }
}
