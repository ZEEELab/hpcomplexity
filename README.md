# hpcomplexity

## TODO

### Configuration files
2. Partitioning configuration file containing GP map partitioning information (Number of possible genotypes = 2^(l\_max))
    1. A set of boolean functions (one for each phenotype) that determine whether a genotype belongs to the phenotype or not (must be mutually exclusive, error if not)
    2. A sorting function that analyzes entire genotype space (using phenotype functions) and assigns phenotype indexes in increasing orders of complexity
    2. A parent function that returns phenotype index of a given genotype.

### Env class
1. Contains instances of h[ost]pop + p[arasite]pop classes
2. Executes ga on hpop and ppop alternatively and uses the other population to determine fitness.

### HPop class
1. Main element is a set of host genomes

### Ppop class
1. Main element is a set of parasite genomes

### GA class
1. Performs optimization for Env mainly  

