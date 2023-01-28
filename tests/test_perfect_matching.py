import pytest
import sys 
#this is some nasty workaround for PYTHONPATH problems
sys.path.append('.')
from routing.bin_grid import *
def test_method1():
    # for now it does not use coordinates but it might later so be ready to change it
    nodes = []
    links = []
    node1 = Node(0,1,1)
    nodes.append(node1)
    node2 = Node(0,2,1)
    nodes.append(node2)
    node3 = Node(0,3,1)
    nodes.append(node3)
    node4 = Node(0,4,1)
    nodes.append(node4)
    node5 = Node(0,5,1)
    nodes.append(node5)
    node6 = Node(0,6,1)
    nodes.append(node6)
    link12 = Link(10, (node1, node2))
    links.append(link12)
    link13 = Link(15, (node1, node3))
    links.append(link13)
    link14 = Link(9, (node1, node4))
    links.append(link14)
    link15 = Link(5, (node1, node5))
    links.append(link15)
    link23 = Link(17, (node2, node3))
    links.append(link23)
    link24 = Link(10, (node2, node4))
    links.append(link24)
    link25 = Link(11, (node2, node5))
    links.append(link25)
    link34 = Link(1, (node3, node4))
    links.append(link34)
    link35 = Link(4, (node3, node5))
    links.append(link35)
    link45 = Link(20, (node4, node5))
    links.append(link45)
    link16 = Link(21, (node1, node6))
    links.append(link16)
    link26 = Link(15, (node2, node6))
    links.append(link26)
    link36 = Link(13, (node3, node6))
    links.append(link36)
    link46 = Link(2, (node4, node6))
    links.append(link46)
    link56 = Link(25, (node5, node6))
    links.append(link56)
    max_price = 0
    for link in links: max_price += link.cost
    links.sort(key=lambda x: x.cost)
    pm = PerfectMatching(nodes, links, max_price)
    print(pm)
    print(pm.perfect_matching(links)) 
    print(pm.curr_best_links)
    assert link35 in pm.curr_best_links
    assert link46 in pm.curr_best_links
    assert link12 in pm.curr_best_links
    assert pm.curr_best_cost == 16.0

test_method1()