# Plot the network structure of the genotype-phenotype map from gptable.csv
# Inputs: gptable.csv file path

import sys
import networkx as nx
import matplotlib.pyplot as plt
import random
from curved_edges import curved_edges
from matplotlib.collections import LineCollection
import math

if len(sys.argv)<2:
    print("Insufficient number of arguments.")
    print("Usage: python3 draw_network_structure.py <path to gptable.csv>")

gptable_path = sys.argv[1]

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


# Prepare node and edge list
node_list = []
edge_list = []
with open(gptable_path) as ifile:
    next(ifile)
    for line in ifile:
        words = line.split(",")

        for node in node_list: # Go through already present nodes
            if lev_dist(node[0],words[0]) == 1: # If lev distance between self and that node is 1
                if node[1]['ptype']!=int(words[1]): # And if self and node phenotypes are different
                    edge_list.append((words[0],node[0],{'type':1})) # Add an edge from self to that node with type 1
                else:
                    edge_list.append((words[0],node[0],{'type':0})) # Add an edge from self to that node with type 0

        node_list.append((words[0],{"ptype":int(words[1])}))
        
#print(edge_list)

G = nx.Graph()
G.add_nodes_from(node_list)
G.add_edges_from(edge_list)




pos = {}
def rand():
    return -0.4 + random.random()*0.8
def dist(a,b):
    return math.sqrt(math.pow(a[0]-b[0],2.0)+math.pow(a[1]-b[1],2.0))
def is_not_away(poses,npos):
    cur = False
    for pos in poses.values():
        threshold = 0.1
        if dist(pos,npos) < threshold:
            cur = True
    return cur

for node in node_list:
    new_pos = (rand()+node[1]["ptype"],rand())
    while is_not_away(pos,new_pos):
        new_pos = (rand()+node[1]["ptype"],rand())
    pos[node[0]] = new_pos 

ecolors = []
# Add normal edges
for u,v in G.edges():
    if G[u][v]['type']==0:
        ecolors.append('b')
    else:
        ecolors.append('#00000000')

plt.subplot(111)
nx.draw(G,pos,with_labels=True,node_color = [x[1]["ptype"] for x in node_list], node_size=100, font_size=2, font_color='#9e9e9e', edge_color=ecolors,vmin=0, vmax=4, cmap = plt.cm.get_cmap('inferno'))

# Add curved edges
cedges = []
for edge in edge_list:
    if edge[2]['type']==1:
        cedges.append((edge[0],edge[1]))

curves = curved_edges(cedges,pos)
lc = LineCollection(curves,color='r',linewidth=0.5)
plt.gca().add_collection(lc)



plt.axis('equal')
plt.savefig('network_structure.svg')
