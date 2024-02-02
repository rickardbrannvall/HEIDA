import networkx as nx
import matplotlib.pyplot as plt


# function that displays a diagram of the netowrk

def diagram(nodes=3):
# Create a directed graph
    G = nx.DiGraph()
    
    # Add nodes
    G.add_node("Interface Node")
    G.add_node("Decoding Node")
    G.add_node("Aggregation Node")
    for i in range(nodes):
        G.add_node(f"Edge Node {i+1}")
    
    # Add edges
    G.add_edge("Aggregation Node", "Decoding Node", label="Encrypted Aggregate Result")
    G.add_edge("Decoding Node", "Interface Node", label="Decrypted Result")
    for i in range(nodes):
        if nodes//2 == i:
            G.add_edge(f"Edge Node {i+1}", "Aggregation Node", label="Encrypted Partial Results")
        else:
            G.add_edge(f"Edge Node {i+1}", "Aggregation Node") #, label="Enc. Result"
    
    # Set positions of nodes
    pos = {
        "Decoding Node": (0, 0),
        "Aggregation Node": (0, 1),
        "Interface Node": (0, -1),
    }
    for i in range(nodes):
        pos[f"Edge Node {i+1}"] = (-nodes//2+i+1-0.5*((nodes+1)%2), 2)
    #pos = None
    
    # Draw the graph
    fig, axs = plt.subplots(1, 1, figsize=(nodes*3,8))
    nx.draw(G, pos, with_labels=True, font_weight='bold')
    edge_labels = nx.get_edge_attributes(G, 'label')
    nx.draw_networkx_edge_labels(G, pos, edge_labels=edge_labels, label_pos=0.5, rotate=False) # , font_size=8
    
    # Show the graph
    #plt.savefig("figs/network.png")
    #plt.show()

# function that 
