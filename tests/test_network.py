import pytest
import sys
sys.path.append('.')
from routing.bin_grid import *
from tests.utils import *

def test_min_cost():
    links, nodes = get_instances_1()
    costs = {nodes[0]: (5, Link(nodes[0], nodes[1])), nodes[1]: (4, Link((nodes[1], nodes[2]))), nodes[2]: (6, Link(nodes[2], nodes[3]))}
    nw = Network(nodes, links, nodes[5])
    min_node = nw.min_cost(costs) 
    assert min_node == nodes[1]

def test_update_costs():
    links, nodes = get_instances_1()
    costs = {nodes[0]: (5, Link(nodes[0], nodes[1])), nodes[1]: (4, Link((nodes[1], nodes[2]))), nodes[2]: (6, Link(nodes[2], nodes[3]))}
    new_nodes = []
    new_nodes.append(nodes[0])
    new_nodes.append(nodes[1])
    new_nodes.append(nodes[2])
    nw = Network(nodes, links, nodes[0])
    new_costs = nw.update_costs(nodes[3], new_nodes, costs)
    min_node = nw.min_cost(new_costs)
    assert min_node == nodes[2]

def test_mst_1():
    links, nodes = get_instances_2() 
    nw = Network(nodes, links, nodes[0])
    mst_links, mst_nodes = nw.prims_mst()
    node3 = [node for node in mst_nodes if node == nodes[2]]
    link12 = nw.get_link(nodes[0], nodes[1])
    link13 = nw.get_link(nodes[0], nodes[2])
    link35 = nw.get_link(nodes[2], nodes[4])
    link34 = nw.get_link(nodes[2], nodes[3])
    for node in mst_nodes:
        print(node)
    for link in mst_links:
        print(link)
    assert len(node3) == 3
    assert link12 in mst_links
    assert link13 in mst_links
    assert link35 in mst_links
    assert link34 in mst_links

def test_get_odd():
    # odd links might contain some links twice as they will be added for both nodes
    _, nodes = get_instances_1()
    links = [] 
    links.append(Link((nodes[0], nodes[1]), 1))
    links.append(Link((nodes[0], nodes[2]), 1))
    links.append(Link((nodes[0], nodes[3]), 1))
    links.append(Link((nodes[3], nodes[4]), 1))
    links.append(Link((nodes[2], nodes[3]), 1))
    nw = Network(nodes, links, nodes[0])
    odd_nodes, odd_links = nw.get_odd(links, nodes)
    assert len(odd_nodes) == 4 
    assert Link((nodes[0], nodes[1]), 1) in odd_links
    assert Link((nodes[0], nodes[2]), 1) in odd_links
    assert Link((nodes[0], nodes[3]), 1) in odd_links
    assert Link((nodes[3], nodes[4]), 1) in odd_links
    assert Link((nodes[2], nodes[3]), 1) in odd_links
    assert nodes[2] not in odd_nodes

#def test_relaxing():
#    links , nodes = get_instances_2()
#    all_links = []
#    nw = Network(nodes, links, nodes[0])
#    all_links.append(nw.get_link(nodes[0], nodes[1]))
#    all_links.append(nw.get_link(nodes[0], nodes[2]))
#    all_links.append(nw.get_link(nodes[0], nodes[3]))
#    all_links.append(nw.get_link(nodes[0], nodes[4]))
#    all_links.append(nw.get_link(nodes[1], nodes[2]))
#    all_links.append(nw.get_link(nodes[3], nodes[4]))
#    final_links = nw.relax_double(all_links, nodes)
#    assert nw.get_link(nodes[0], nodes[1]) in final_links
#    assert nw.get_link(nodes[0], nodes[2]) in final_links
#    assert nw.get_link(nodes[0], nodes[3]) in final_links
#    assert nw.get_link(nodes[0], nodes[4]) in final_links
#    assert nw.get_link(nodes[3], nodes[4]) in final_links
#    assert nw.get_link(nodes[1], nodes[2]) in final_links

def test_euler_1():
    links, nodes = get_instances_2()
    nw = Network(nodes, links, nodes[0])
    all_links = []
    nw = Network(nodes, links, nodes[0])
    all_links.append(nw.get_link(nodes[0], nodes[1]))
    all_links.append(nw.get_link(nodes[0], nodes[2]))
    all_links.append(nw.get_link(nodes[0], nodes[3]))
    all_links.append(nw.get_link(nodes[0], nodes[4]))
    all_links.append(nw.get_link(nodes[1], nodes[2]))
    all_links.append(nw.get_link(nodes[3], nodes[4]))
    for link in all_links: print(link)
    euler_tour = nw.euler_tour(all_links)
    assert len(euler_tour) == 6

def test_euler_2():
    links, nodes = get_instances_2()
    nw = Network(nodes, links, nodes[0])
    all_links = []
    nw = Network(nodes, links, nodes[0])
    all_links.append(nw.get_link(nodes[0], nodes[1]))
    all_links.append(nw.get_link(nodes[0], nodes[2]))
    all_links.append(nw.get_link(nodes[0], nodes[3]))
    all_links.append(nw.get_link(nodes[0], nodes[4]))
    all_links.append(nw.get_link(nodes[1], nodes[2]))
    all_links.append(nw.get_link(nodes[3], nodes[4]))
    all_links.append(nw.get_link(nodes[3], nodes[4]))
    for link in all_links: print(link)
    euler_tour = nw.euler_tour(all_links)
    assert len(euler_tour) == 6

def test_hamilton():
    links, nodes = get_instances_2()
    nw = Network(nodes, links, nodes[0])
    all_links = []
    nw = Network(nodes, links, nodes[0])
    all_links.append(nw.get_link(nodes[0], nodes[1]))
    all_links.append(nw.get_link(nodes[0], nodes[2]))
    all_links.append(nw.get_link(nodes[0], nodes[3]))
    all_links.append(nw.get_link(nodes[0], nodes[4]))
    all_links.append(nw.get_link(nodes[1], nodes[2]))
    all_links.append(nw.get_link(nodes[3], nodes[4]))
    all_links.append(nw.get_link(nodes[3], nodes[4]))
    for link in all_links: print(link)
    euler_tour = nw.euler_tour(all_links)
    hamiltonian_cycle = nw.hamiltonian_cycle(euler_tour)
    for link in hamiltonian_cycle: 
        print(link)
    assert len(hamiltonian_cycle) == 5
    by_node = links_per_node(hamiltonian_cycle, nodes)
    for node, links in by_node.items(): 
        assert len(links) == 2

test_get_odd()