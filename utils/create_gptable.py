# This utility allows a user to create an optimized gptable based on parameters

# Options: Given a set of genotypes, these options will output
# 1 - Strongly ordered - most linear gptable (i.e. minimal number of phenotype skips)
# 2 - Minimally ordered - As many phenotype skips as possible

# Note that the phenotypes are numbered from 0 to N in order of complexity. As such each iteration checks for whether the configuration is allowed under this complexity ordering

import random
import math
import numpy as np
import copy

# PARAMETERS
gtype_len = 5 # Numbered 0 to 2^(gtype_len)-1
num_ptypes = 4 # Numbered 0 to num_ptypes-1
opt_type = 2 # Strongly ordered or minimally ordered (see above)

# Simulated annealing parameters
T_init = 1
T_final = 0.01
generations = 10000

# Mutator function 
def mutate(table_old):
    table = copy.deepcopy(table_old)
    rand_pid1 = random.randrange(num_ptypes) # Phenotype from which to pop  a random element
    while len(table[rand_pid1]) <= 1: # Ensure you're not emptying a phenotype
        rand_pid1 = random.randrange(num_ptypes)

    rand_pid2 = random.randrange(num_ptypes) # Phenotype in which to add random element
    while rand_pid1==rand_pid2: # Make sure the second phenotype is different
        rand_pid2 = rand_pid2 = random.randrange(num_ptypes)

    removed_gtype = table[rand_pid1].pop(random.randrange(len(table[rand_pid1])))

    table[rand_pid2].append(removed_gtype)

    table.sort(key=len,reverse=True)

    return table

# Levenshtein distance
def lev_dist(s, t):
    """ 
        iterative_levenshtein(s, t) -> ldist
        ldist is the Levenshtein distance between the strings 
        s and t.
        For all i and j, dist[i,j] will contain the Levenshtein 
        distance between the first i characters of s and the 
        first j characters of t
    """

    rows = len(s)+1
    cols = len(t)+1
    dist = [[0 for x in range(cols)] for x in range(rows)]

    # source prefixes can be transformed into empty strings 
    # by deletions:
    for i in range(1, rows):
        dist[i][0] = i

    # target prefixes can be created from an empty source string
    # by inserting the characters
    for i in range(1, cols):
        dist[0][i] = i
        
    for col in range(1, cols):
        for row in range(1, rows):
            if s[row-1] == t[col-1]:
                cost = 0
            else:
                cost = 1
            dist[row][col] = min(dist[row-1][col] + 1,      # deletion
                                    dist[row][col-1] + 1,      # insertion
                                    dist[row-1][col-1] + cost) # substitution

    #for r in range(rows):
    #    print(dist[r])


    return dist[row][col]
    
# Integer to binary string
def id_to_binary(num):
    as_str = "{0:b}".format(num)
    while len(as_str)!=gtype_len:
        as_str = "0%s" % as_str
    return as_str

# Calculate energy of gptable
def energy(table,etype=1):
    if etype==1:
        # Strongly ordered
        E = 0.0
        # Check number of phenotype skips by comparing pairwise
        # 1.0 energy cost for each skip (0.5)
        for pid1 in range(len(table)):
            for gid1 in table[pid1]:
                gtype1 = id_to_binary(gid1)
                for pid2 in range(len(table)):
                    for gid2 in table[pid2]:
                        gtype2 = id_to_binary(gid2)

                        if lev_dist(gtype1,gtype2)==1 and abs(pid1-pid2)>1:
                            E += 1.0 # (Half the cost because this comparison will happen twice)

        # Check if the number of genotypes is ordered
        for pid in range(len(table)-1):
            if len(table[pid]) <= len(table[pid+1]):
                E += 5.0

        return E


    elif etype==2:
        # Minimally ordered
        E = 0.0
        # Check number of phenotype skips by comparing pairwise
        # 1.0 energy cost for each non skip (0.5)
        for pid1 in range(len(table)):
            for gid1 in table[pid1]:
                gtype1 = id_to_binary(gid1)
                for pid2 in range(len(table)):
                    for gid2 in table[pid2]:
                        gtype2 = id_to_binary(gid2)

                        if lev_dist(gtype1,gtype2)==1 and abs(pid1-pid2)<=1:
                            E += 0.5 # (Half the cost because this comparison will happen twice)

        # Check if the number of genotypes is ordered, wrong ordering increases energy
        for pid in range(len(table)-1):
            if len(table[pid]) <= len(table[pid+1]):
                E += 5.0

        return E

    else:
        print("Wrong energy type, exiting...")
        exit(0)

def cooling(temp_init,temp_fin,steps,time): # time == Monte carlo step number
    tau = float(steps)/math.log(temp_init/temp_fin)
    return temp_init * math.exp(-float(time)/tau)

def is_accepted(E_cur,E_new,T): # Acceptance probability
    if E_new < E_cur:
        return True
    else:
        delta = E_new-E_cur
        prob =  np.exp(-delta/T)
        if prob>random.random():
            return True
        else:
            return False



# Create a blank gptable
gptable = [[] for i in range(num_ptypes)]

# Add genotypes to gptable randomly
for gid in range(2**gtype_len):
    gptable[random.randrange(num_ptypes)].append(gid)

optGP = copy.deepcopy(gptable)
optEnergy = energy(gptable,opt_type)

curEnergy = energy(gptable,opt_type)

for gen in range(generations+1):
    temp = cooling(T_init,T_final,generations,gen)
    print(gen,temp,curEnergy,optEnergy)

    gptable_new = mutate(gptable)
    newEnergy = energy(gptable_new,opt_type)
    
    if is_accepted(curEnergy,newEnergy,temp):
        gptable = copy.deepcopy(gptable_new)
        curEnergy = newEnergy
    
    if curEnergy < optEnergy:
        optGP = copy.deepcopy(gptable)
        optEnergy = curEnergy


if optEnergy == 0.0:
    print("Perfect gptable found with ordering type %d" % opt_type)
else:
    print("Imperfect ordering gptable found, type %d" % opt_type)


# Best config in optGP 
with open("gptable.csv","w+") as ofile:
    ofile.write("GTYPE,PTYPE_ID\n")
    for gidx in range(2**gtype_len):
        for pidx in range(num_ptypes):
            if gidx in optGP[pidx]:
                ofile.write("%s,%d\n" % (id_to_binary(gidx),pidx))