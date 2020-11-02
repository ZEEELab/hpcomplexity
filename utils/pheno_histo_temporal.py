# Plot a histogram of phenotype distribution for given update range
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

ptype_ids = []

with open(fpath+"/complexity_table.csv") as cfile:
    next(cfile)
    for line in cfile:
        words = line.split(",")
        ptype_id = int(words[0])
        ptype_ids.append(ptype_id)

ptype_ids.append(min(ptype_ids)-1)
ptype_ids.append(max(ptype_ids)+1)


if not os.path.exists('outputs'):
    os.makedirs('outputs')

upds = range(init_upd,final_upd+1,severy)

for upd in upds:
    print("Processing update: %d" % upd)
    hptypes = []
    pptypes = []

    with open(fpath+"/hpop_"+str(upd)+".csv") as hfile:
        next(hfile)
        for line in hfile:
            hptypes.append(int(line.split(",")[1]))

    with open(fpath+"/ppop_"+str(upd)+".csv") as pfile:
        next(pfile)
        for line in pfile:
            pptypes.append(int(line.split(",")[1]))


    labelsh, countsh = np.unique(hptypes, return_counts=True)
    plt.bar(labelsh, countsh, align='center', fill= False, linewidth=1, label="host", edgecolor="blue")
    labelsp, countsp = np.unique(pptypes, return_counts=True)
    plt.bar(labelsp, countsp, align='center', fill= False, linewidth=1, label="parasite", edgecolor="red")


    plt.gca().set_xticks(ptype_ids)
    plt.xlabel("Phenotype ID")
    plt.ylabel("Frequency")
    plt.legend()
    plt.savefig("outputs/histo_"+str(upd)+".png")
    plt.close()

    
