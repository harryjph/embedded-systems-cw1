import pytest
import sys 
import datetime
import numpy as np
import networkx as nx
#this is some nasty workaround for PYTHONPATH problems
sys.path.append('.')
from routing.bin_grid import *
from tests.utils import *


def test_method1():
    # for now it does not use coordinates but it might later so be ready to change it
    links, nodes = get_instances_1()
    nw = Network(nodes, links, nodes[0])
    link46 = nw.get_link(nodes[3], nodes[5])
    link35= nw.get_link(nodes[2], nodes[4])
    link12= nw.get_link(nodes[0], nodes[1])
    max_price = 0
    for link in links: max_price += link.cost
    links.sort(key=lambda x: x.cost)
    pm = PerfectMatching(nodes, links, max_price)
    print(pm)
    pm.perfect_matching(links) 
    for link in pm.curr_best_links:
        print("best link", link)
    assert link46 in pm.curr_best_links
    assert link35 in pm.curr_best_links
    assert link12 in pm.curr_best_links
    assert pm.curr_best_cost == 16.0

def test_method2():
    links, nodes = get_instances_1()
    max_price = 0
    for link in links: max_price += link.cost
    links.sort(key=lambda x: x.cost)
    pm = PerfectMatching(nodes, links, max_price)
    print(pm)
    start = datetime.now()
    pm.perfect_matching(links) 
    elapsed = start - datetime.now()
    print("elapsed", elapsed)
    for link in pm.curr_best_links:
        print("best link", link)
    assert elapsed < timedelta(seconds=10)

def test_method3():
    links, nodes = get_instances_1()
    graph = nx.Graph()
    graph.add_nodes_from(nodes)
    for link in links: 
        graph.add_edge(link.nodes[0], link.nodes[1], weight=link.cost)
    print(graph)
    min_w = nx.min_weight_matching(graph)
    for smt in min_w: 
        print(smt)
        for link in links: 
            if link.is_link(smt[0], smt[1]):
                print("link", link)

test_method3()
    
