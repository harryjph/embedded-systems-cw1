from typing import List, Dict, Tuple
from datetime import timedelta, datetime
import networkx as nx

from dataclasses import dataclass
from copy import deepcopy
import math
import numpy as np

id = 1000

def get_id():
    global id
    id += 1
    return id 


@dataclass 
class Node: 
    x_coord: float 
    y_coord: float 
    fullness: float 
    node_id: int = get_id() 
    at_capacity: float = 0.75 # percentage at which a node will be considered active (ie in need for emptying)
    needs_emptying: bool = False
    time_to_full: timedelta = timedelta(seconds = 0) # this will be updated once there is more data available on past
    last_emptied: datetime = datetime.now() # to initialise

    def __key__(self):
        return (self.x_coord, self.y_coord)

    def __hash__(self):
        return hash(self.__key__())
    
    def __str__(self):
        return "x: " + str(self.x_coord) +", y: " + str(self.y_coord) + ", id: " + str(self.node_id) + "\n"

    def update_fullness(self, fill_level):
        # TODO make it so that the frequency with which it requires fullness data is proportional 
        # to the time it has passed from the last emptying action (using also statistical data 
        # related to each single bin on how long it takes to fill on average, data held by time_to_full)
        self.fullness = fill_level # maybe add the prompt to get data on fullness from here?  
        if fill_level > 0.75:
            self.needs_emptying = True
        if fill_level == 0.0: # might have to change this based on how it is conveyed that the bin has been emptied 
            self.last_emptied = datetime.now()

@dataclass
class Link: 
    def __init__(self, nodes, cost = None):
        self.nodes: Tuple[Node, Node] = nodes
        if cost is None: 
            self.cost: float = self.get_cost() 
        else: self.cost = cost

    @property
    def is_active(self) -> bool:
        if self.nodes[0].needs_emptying and self.nodes[1].needs_emptying: 
            return True
        else: 
            return False
    
    def __str__(self):
        return "" + str(self.nodes[0].x_coord) + ""+ str(self.nodes[0].y_coord)+ "," + str(self.nodes[1].x_coord) + ""+ str(self.nodes[1].y_coord)+ "\n"
    
    def __eq__(self, other):
        if self.nodes == other.nodes and self.cost == other.cost: return True
        else: return False

    def get_cost(self):
        delta_x = (self.nodes[0].x_coord - self.nodes[1].x_coord)**2
        delta_y = (self.nodes[0].y_coord - self.nodes[1].y_coord)**2
        return math.sqrt(delta_x + delta_y) 

    def is_link(self, node1: Node, node2: Node = None) -> bool:
        if node2 is None: 
            if node1 in self.nodes: return True
            else: return False 
        if node1 in self.nodes and node2 in self.nodes : return True 
        else: return False
    
    def other_node(self, node): 
        others = [node1 for node1 in self.nodes if node1 != node]
        return others[0]
    





class Network: 
    def __init__(self, nodes, links, start_point):
        self.nodes: List[Node] = nodes 
        self.links: List[Link] = links # all the possible links
        self.start_point: Node = start_point # place where collection must start and end
        self.max_cost: float = max([link.cost for link in links])
    
    # get only the links that connect two nodes that need servicing
    # might not need this
    def get_active_links(self) -> List[Link]:
        return [link for link in self.links if link.is_active]

    def get_cost(self, node1: Node, node2: Node) -> float:
        for link in self.links: 
            if link.is_link(node1, node2): return link.cost 
        return self.max_cost # should not happen as all nodes should have a link connecting them
        # TODO decide whether there needs to be something handling the function being called on the node with itself. Can it happen?
    
    def get_link(self, node1: Node, node2: Node) -> Link: 
        for link in self.links:
            if link.is_link(node1, node2): return link
        # return nothing otherwise because the two nodes MUST have a link connecting them 
    
    def check_emptying(self) -> None :
        for node in self.nodes:
            if node.fullness >= node.at_capacity: node.needs_emptying = True 

    def min_cost(self, costs: Dict[Node, Tuple[float, Link]]) -> Node: 
        min_cost: float = self.max_cost
        min_node: Node = self.start_point
        for node, cost in costs.items(): 
            if cost[0] < min_cost: 
                min_node = node
                min_cost = cost[0]
        return min_node

    def update_costs(self, node: Node, nodes: List[Node], costs: Dict[Node, Tuple[float, Link]]) -> Dict[Node, Tuple[float, Link]]:
        new_costs = {}
        for new_node in nodes: 
            cost = self.get_cost(node, new_node)
            if cost < costs[new_node][0]: 
                link = self.get_link(node, new_node)
                new_costs[new_node] = (cost, link)
            else: 
                new_costs[new_node] = costs[new_node]
        return new_costs
    
    def init_costs(self, active_nodes: List[Node]) -> Dict[Node, Tuple[float, Link]]: 
        costs: List[Tuple[float, Link]] = []
        for node in active_nodes:
            link = self.get_link(node, self.start_point)
            cost = self.get_cost(node, self.start_point)
            costs.append((cost, link))
        return dict(zip(active_nodes, costs))

    def prims_mst(self) -> Tuple[List[Link], List[Node]]:
        self.check_emptying()
        active_nodes: List[Node] = [node for node in self.nodes if node.needs_emptying]
        # starts with start_point, initialise the cost dict to hold all the distances from the start 
        mst_links: List[Link] = []
        mst_nodes: List[Node] = []
        costs: Dict[Node, Tuple[float, Link]] = self.init_costs(active_nodes)
        while len(active_nodes) != 0: 
            if len(mst_nodes) == 0: 
                costs: Dict[Node, Tuple[float, Link]] = self.init_costs(active_nodes)
                mst_nodes.append(self.start_point)
                active_nodes.remove(self.start_point)
            else:
                min_cost_node = self.min_cost(costs)
                print("min cost node", min_cost_node)
                link = costs[min_cost_node][1]
                node1, node2 = link.nodes
                mst_links.append(link)
                # by appending them both it will be easier to keep track of 
                # nodes with more than two links (they should all appear two times
                # by the end of the algorithm)
                mst_nodes.append(node1)
                mst_nodes.append(node2)
                active_nodes.remove(min_cost_node)
                costs = self.update_costs(min_cost_node, active_nodes, costs)
        return (mst_links, mst_nodes)

    
    def get_odd(self, links: List[Link], nodes: List[Node]) -> Tuple[List[Node], List[Link]]:
        links_by_node = links_per_node(links, nodes)
        odd_nodes: List[Node] = [node for node in nodes if len(links_by_node[node]) % 2 != 0]
        odd_links = []
        for node in odd_nodes: 
            odd_links.extend(links_by_node[node])
        return (odd_nodes, odd_links)
        # need to find out which algorithm can be used to find minimum weight perfect matching tree
        # right now looking at edmond's blossom algorithm but seems quite complex (LP formulation)
    
    def is_relaxed(self, node, relaxed_links) -> bool :
        node_links = []
        for link in relaxed_links: 
            if node in link.nodes: 
                node_links.append(link)
        if len(node_links) > 2:
            print("too long node links")
            for link in node_links:
                print(link)
        assert len(node_links) <= 2
        if len(node_links) == 2: return True
        else: return False

    def is_entered(self, node, relaxed_links) -> bool:
        node_links = []
        for link in relaxed_links: 
            if node in link.nodes: 
                node_links.append(link)
        assert len(node_links) <= 2
        if len(node_links) == 1: return True
        else: return False


    def remove_links_relaxed(self, node, relaxed_nodes, remaining_links):
        node_links = [link for link in remaining_links if node in link.nodes]
        for node2 in relaxed_nodes: 
            links = [link for link in node_links if node2 in link.nodes]
            for link in links: 
                remaining_links.remove(link)
    
    def is_closing_link(self, link, relaxed_links) -> bool:
        node1, node2 = link.nodes
        if self.is_entered(node1, relaxed_links) and self.is_entered(node2, relaxed_links): return True
        else: return False

    def christofides(self) -> List[Link]: 
        # prims to get mst 
        mst_links, mst_nodes = self.prims_mst()
        # get all vertices with odd number of connections 
        links_odd, nodes_odd = self.get_odd(mst_links, mst_nodes)
        # instantiate networx graph 
        graph = nx.Graph()
        # add nodes 
        graph.add_nodes_from(nodes_odd)
        # add links 
        for link in links_odd: 
            graph.add_edge(link.nodes[0], link.nodes[1], weight=link.cost)
        # get minimum weight perfect matching and add the nodes and links to the lists 
        min_w = nx.min_weight_matching(graph)
        mwpf_links = []
        for (node1, node2) in min_w: 
            link = self.get_link(node1, node2)    
            mwpf_links.append(link)    
        # add nodes and links to mst nodes and links 
        mst_links.extend(mwpf_links)
        mst_nodes.extend(nodes_odd)
        # do relaxation 

    def is_dead_end(self, links, start_node, node, link) -> bool:
        # this funcion checks if the current link brings to a node whose only other link is the final link
        other = link.other_node(node)
        nodes = []
        nodes.append(other)
        nodes.append(start_node)
        by_node = links_per_node(links, nodes)
        if len(by_node[start_node]) == 1: 
            final_link = by_node[start_node][0]
            other_node_links = [other_link for other_link in by_node[other] if other_link != link]
            if len(other_node_links) == 1 and final_link == other_node_links[0]: return True
        return False 
    
    def beginning_of_end(self, start_node, links) -> Tuple[Node, List[Link]]:
        forced_path = True
        path = []
        next_node = deepcopy(start_node)
        available_links = deepcopy(links)
        boe = start_node
        while forced_path and len(available_links) != 0:
            by_node = links_per_node(available_links, [next_node])
            links_from_node = by_node[next_node]
            if len(links_from_node) > 1:
                forced_path = False 
            else: 
                link = links_from_node[0]
                #print("adding to path", link)
                path.append(link)
                #for pathho in path: print("pattho", pathho)
                available_links.remove(link)
                boe = link.other_node(next_node)
                next_node = boe
        return boe, path
                

            


    def euler_tour(self, links_all):
        links = []
        #eliminate duplicates TODO figure out how to handle them
        for link in links_all: 
            if link not in links: links.append(link)
        sorted_links = [links[0]]
        last_node = links[0].nodes[0]
        start_node = last_node
        links = [link for link in links if link != links[0]]
        while len(links) != 0: 
            boe, path_to_end = self.beginning_of_end(start_node, links)
            #print("sorted so far")
            #for slink in sorted_links: print(slink)
            other = sorted_links[-1].other_node(last_node)
            possible_links = [link for link in links if other in link.nodes and last_node not in link.nodes and link not in path_to_end]
            if len(possible_links) == 0: 
                path_to_end.reverse()
                for last_link in path_to_end:
                    sorted_links.append(last_link)
                break
            next_link = possible_links[0]
            sorted_links.append(next_link)
            # this should also handle double links to make a proper Euler tour
            links = [av_link for av_link in links if av_link != next_link]
            last_node  = other 
        
        return sorted_links
    
    def get_common_node(self, link1, link2) -> Node:
        node11, node12 = link1.nodes
        if node11 in link2.nodes: return node11
        elif node12 in link2.nodes: return node12
        else: return None # should not happen when we are calling this function

    
    def hamiltonian_cycle(self, links: List[Link]) -> List[Link]:
        visited_nodes: List[Node] = []
        first_link = links[0]
        relaxed_links: List[Link] = [first_link]
        links.remove(first_link)
        start_node = self.get_common_node(first_link, links[1])
        visited_nodes.append(start_node)
        prev_node = first_link.other_node(start_node)
        visited_nodes.append(prev_node)
        # use prev node when you need to do a shortening
        needs_relaxation = False
        for link in links: 
            other = link.other_node(prev_node)
            if other in visited_nodes:
                print("invalid link", link)
                #set up train of invalidity to do relaxation in future iteration (when possible)
                if not needs_relaxation: 
                    # if this is the first invalid link that is encountered 
                    # otherwise need to keep the orphan as the original one
                    # since in a train of invalid links
                    orphan = prev_node
                prev_node = other # needed to find the final destination when multiple invalid links are met in sequence
                needs_relaxation = True
            else: 
                if needs_relaxation: 
                    print("solving relaxation", link)
                    # do relaxation
                    new_link = self.get_link(orphan, other)
                    relaxed_links.append(new_link)
                    visited_nodes.append(other)
                    prev_node = other
                    needs_relaxation = False # relaxation resolved
                else:
                    print("appending normally", link)
                    # add link normally
                    relaxed_links.append(link)
                    visited_nodes.append(other)
                    prev_node = other
        # get only two nodes with only one link and find closing link 
        by_node = links_per_node(relaxed_links, visited_nodes)
        incomplete_nodes = []
        for node, links in by_node.items():
            if len(links) < 2: 
                incomplete_nodes.append(node) 
                assert len(links) == 1
        assert len(incomplete_nodes) == 2
        closing_link = self.get_link(incomplete_nodes[0], incomplete_nodes[1])
        relaxed_links.append(closing_link)
        return relaxed_links

                






       
# TODO add this back to class, should not be needed outside
def links_per_node(links: List[Link], nodes: List[Node], node_rel=None) -> Dict[Node, int]:
    if node_rel is not None:
        nodes.append(node_rel)
    links_by_node: Dict[Node, int] = {}
    for node in nodes: 
        node_links = [link for link in links if link.is_link(node)]
        links_by_node[node] = node_links
    return links_by_node

