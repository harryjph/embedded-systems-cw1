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
        return "cost: " + str(self.cost) +", node1: x: " + str(self.nodes[0].x_coord) + " y: "+ str(self.nodes[0].y_coord)+ ", node2: x: " + str(self.nodes[1].x_coord) + " y: "+ str(self.nodes[1].y_coord)+ "\n"
    
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
        for node in active_nodes: 
            print("active node", node)
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
    
    def is_relaxed(node, relaxed_links) -> bool :
        node_links = []
        for link in relaxed_links: 
            if node in link.nodes: 
                node_links.append(link)
        assert len(node_links) <= 2
        if len(node_links) == 2: return True
        else: return False

    def remove_links_relaxed(node, relaxed_nodes, remaining_links):
        node_links = [link for link in remaining_links if node in link.nodes]
        for node2 in relaxed_nodes: 
            links = [link for link in node_links if node2 in link.nodes]
            for link in links: 
                remaining_links.remove(link)

    def relax_double(self, links: List[Link], nodes: List[Node]) -> List[Link]:
        relaxed_nodes: List[Node] = []
        relaxed_links: List[Link] = []
        remaining_links: List[Link] = deepcopy(links)
        remaining_nodes: List[Link] = deepcopy(nodes)
        for link in links: 
            if link in remaining_links:
                node1, node2 = link.nodes
                if node1 and node2 in relaxed_nodes:
                    continue 
                elif node1 in relaxed_nodes or node2 in relaxed_nodes:
                    if node1 in relaxed_nodes: 
                        node_rel = node1
                        node_next = node2
                    elif node2 in relaxed_nodes: 
                        node_rel = node2 
                        node_next = node1
                    # get all the possible nodes that could be reached from node1 (that haven't been relaxed yet )
                    # there should always be one becasue the previous part of the algorithm ensured that each node 
                    # had an even number of connections 
                    by_node = links_per_node(remaining_links, remaining_nodes@[node_rel])
                    node_rel_links = by_node[node_rel]
                    # get the shortest possible link from node1
                    node_rel_neighbour = min(node_rel_links)
                    # get the nearest non-relaxed neighbour of node1
                    node_alt = [node for node in node_rel_neighbour.nodes if node != node_rel]
                    # get its link with node2 (this can be obtained among all the links)
                    new_link = self.get_link(node_next, node_alt)
                    relaxed_links.append(new_link)
                    # remove the two current links from the next possible ones
                    remaining_links.remove(link)
                    if new_link in remaining_links: remaining_links.remove(new_link)
                    # check if any of the two nodes can now be relaxed 
                    # (in which case remove from remaining links all the links between the relaxed nodes)
                    if self.is_relaxed(node_alt): 
                        self.remove_links_relaxed(node_alt, remaining_links)
                        relaxed_nodes.append(node_alt)
                    if self.is_relaxed(node_next):
                        self.remove_links_relaxed(node_next, remaining_links)
                        relaxed_nodes.append(node_next)
                else: 
                    relaxed_links.append(link)
                    remaining_links.remove(link)
                    # check if adding this made the nodes relaxed
                    if self.is_relaxed(node1): 
                        self.remove_links_relaxed(node1, remaining_links)
                        relaxed_nodes.append(node1)
                    if self.is_relaxed(node2):
                        self.remove_links_relaxed(node2, remaining_links)
                        relaxed_nodes.append(node2)
        return relaxed_links

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

       
# TODO add this back to class, should not be needed outside
def links_per_node(links: List[Link], nodes: List[Node]) -> Dict[Node, int]:
    links_by_node: Dict[Node, int] = {}
    for node in nodes: 
        node_links = [link for link in links if link.is_link(node)]
        links_by_node[node] = node_links
    return links_by_node

# TODO delete :(
class PerfectMatching: 
    def __init__(self, nodes: List[Node], links: List[Link], max_price: float):
        self.to_match: List[Node] = nodes
        self.links: List[Link] = links
        self.nodes: List[Node] = nodes # to go back to the initial state 
        self.curr_best_cost: float = max_price
        self.curr_best_nodes: List[Node] = []
        self.curr_best_links: List[Link] = []
        self.nodes_matched: List[Node] = []
        self.links_matched: List[Link] = []
        self.cost: float = 0.0
        self.max_cost: float = max_price

    def undo(self, to_match, matched, nodes_matched, cost): 
        self.to_match = to_match
        self.nodes_matched = nodes_matched
        self.links_matched = matched
        self.cost = cost
        
    def perfect_matching(self, links_unmatched) -> bool: 
        #print("links unmatched, beginning of call ", len(links_unmatched))
        for link in links_unmatched:
            # save initial state 
            # TODO add things to save as you go 
            to_match_init = deepcopy(self.to_match)
            matched_init = deepcopy(self.links_matched)
            nodes_matched_init = deepcopy(self.nodes_matched)
            cost_init = deepcopy(self.cost)

            # add link to the list of matched links and update cost
            #for link_matched in self.links_matched: 
                #print("matched: ", link_matched)
            self.links_matched.append(link)
            #print("appended link:", link)
            node1, node2 = link.nodes
            # add to nodes that have been matched already
            self.nodes_matched.append(node1)
            self.nodes_matched.append(node2)
            self.to_match.remove(node1)
            self.to_match.remove(node2)

            # check cost is not worse than the best one
            self.cost += link.cost
            if self.cost >= self.curr_best_cost: 
                self.undo(to_match_init, matched_init, nodes_matched_init, cost_init)
                #print("can't be better")
                return False
            
            # remove links with the nodes that have just been added
            next_links = []
            for available_link in links_unmatched[1:]:
                if available_link.is_link(node1) or available_link.is_link(node2): 
                    #print("is link of chosen", available_link)
                    pass
                else:
                    next_links.append(available_link)
            #print("next links", len(next_links))
            
            # check that all the unmatched node have at least one link in the available ones
            # this might be unnecessary as as long as there is a node there will be a link
            links_by_node = links_per_node(next_links, self.to_match)
            for node in self.to_match: 
                if node not in links_by_node.keys(): 
                    self.undo(to_match_init, matched_init, nodes_matched_init, cost_init)
                    #print("missing node")
                    return False 
            
            #available_link_init = deepcopy(links_unmatched)
            
            # call the recursion
            self.perfect_matching(next_links)
            # undo for next attempt (as not only we need a valid perfect match, but also the minimum one)
            self.undo(to_match_init, matched_init, nodes_matched_init, cost_init)    
                

        # all possible links have been used
        # check that all nodes have been matched ù
        # TODO determine how to handle the possibility of having odd nodes 
        #print("to match at the end: ", self.to_match)
        if len(self.to_match) == 0: 
            if self.cost < self.curr_best_cost: 
                #print("new best ", self.cost)
                self.curr_best_cost = self.cost 
                self.curr_best_links = self.links_matched
                self.curr_best_nodes = self.nodes_matched
            return True
        else: 
            #print("stuff to match still")
            return False
        

