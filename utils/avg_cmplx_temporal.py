# Plot the average complexity of host and parasite populations over time
# Inputs: Results folder path, initial update number, final update number, update step to plot (if pop file present)

import sys
import numpy as np
import matplotlib.pyplot as plt
import os

if len(sys.argv)<2:
    print("Insufficient number of arguments.")
    print("Usage: python avg_cmplx_temporal.py <results_folder_path> <init_update> <final_update> <save_every>")

fpath = sys.argv[1]
init_upd = int(sys.argv[2])
final_upd = int(sys.argv[3])
severy = int(sys.argv[4])


ctable = {}

with open(fpath+"/complexity_table.csv") as cfile:
    next(cfile)
    for line in cfile:
        words = line.split(",")
        ptype_id = int(words[0])
        complexity = float(words[2])
        ctable[ptype_id] = complexity

print("Complexity table,")
print(ctable)

upds = range(init_upd,final_upd+1,severy)
h_cmp_t = []
p_cmp_t = []

for upd in upds:
    hcmps = []
    pcmps = []
    
    with open(fpath+"/hpop_"+str(upd)+".csv") as hfile:
        next(hfile)
        for line in hfile:
            hcmps.append(ctable[int(line.split(",")[1])])
    
    with open(fpath+"/ppop_"+str(upd)+".csv") as pfile:
        next(pfile)
        for line in pfile:
            pcmps.append(ctable[int(line.split(",")[1])])

    print("Calculating for update: %d" % (upd))
    h_cmp_t.append(np.mean(hcmps))
    p_cmp_t.append(np.mean(pcmps))

plt.plot(upds,h_cmp_t,label="Host")
plt.plot(upds,p_cmp_t,label="Parasite")
plt.xlabel("Time (updates)")
plt.ylabel("Average population complexity")
plt.legend()

if not os.path.exists('outputs'):
    os.makedirs('outputs')

plt.savefig("outputs/avg_cmplx_graph.png")
print("Graph saved as avg_cmplex_graph.png")

with open("outputs/avg_cmplx.csv","w+") as ofile:
    ofile.write("upd,avg_hcomp,avg_pcomp\n")
    for i in  range(len(upds)):
        ofile.write("%d,%f,%f\n" % (upds[i],h_cmp_t[i],p_cmp_t[i]))

print("Data saved as avg_cmplx.csv")









