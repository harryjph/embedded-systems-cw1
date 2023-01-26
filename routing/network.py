from typing import List, Dict
from time import timedelta, datetime
from dataclasses import dataclass
from copy import deepcopy

@dataclass 
class Node: 
    x_coord: float 
    y_coord: float 
    fullness: float 
    needs_emptying: bool = False
    at_capacity: float = 0.75 # percentage at which a node will be considered active (ie in need for emptying)
    time_to_full: timedelta = timedelta(seconds = 0) # this will be updated once there is more data available on past
    last_emptied: datetime = datetime.now() # to initialise

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
    cost: float
    nodes: tuple(Node, Node)

    @property
    def is_active(self) -> bool:
        if self.nodes[0].needs_emptying and self.nodes[1].needs_emptying: 
            return True
        else: 
            return False
    
    def is_link(self, node1: Node, node2: Node = None) -> bool:
        if node2 is None: 
            for link in self.links:
                if node1 in link.nodes: return True
            return False 
        if node1 in self.nodes and node2 in self.nodes : return True 
        else: return False
    




@dataclass
class Network: 
    nodes: List[Node]
    links: List[Link] # all the possible links
    start_point: Node # place where collection must start and end
    max_cost: float = max([link.cost for link in links])
    
    # get only the links that connect two nodes that need servicing
    # might not need this
    def get_active_links(self) -> List[Link]:
        return [link for link in self.links if link.is_active]

    def get_cost(self, node1: Node, node2: Node) -> float:
        for link in self.links: 
            if link.is_link(node1, node2): return link.cost 
        return self.mac_cost # should not happen as all nodes should have a link connecting them
        # TODO decide whether there needs to be something handling the function being called on the node with itself. Can it happen?
    
    def get_link(self, node1: Node, node2: Node) -> Link: 
        for link in self.links:
            if link.is_link(node1, node2): return link
        # return nothing otherwise because the two nodes MUST have a link connecting them 
    
    def min_cost(self, costs: Dict[Node, tuple(float, Link)]) -> Node: 
        min_cost: float = self.max_cost
        min_node: Node = self.start_point
        for node, cost in costs.items(): 
            if cost[1] < min_cost: 
                min_node = node
                min_cost = cost[1]
        return min_node

    def update_costs(self, node: Node, nodes: List[Node], costs: Dict[Node, tuple(float, Link)]) -> None: 
        for new_node in nodes: 
            cost = self.get_cost(node, new_node)
            if cost < costs[new_node][0]: 
                costs[new_node] = (cost, self.get_link(node, new_node))
    
    def init_costs(self, active_nodes: List[Node]) -> Dict[Node, tuple(float, Link)]: 
        costs: List[tuple(float, Link)] = []
        for node in active_nodes:
            link = self.get_link(node, self.start_point)
            cost = self.get_cost(node, self.start_point)
            costs.append((cost, link))
        return dict(zip(active_nodes, costs))

    def prims_mst(self) -> tuple(List[Link], List[Node]):
        active_nodes: List[Node] = [node for node in self.nodes if node.needs_emptying]
        # starts with start_point, initialise the cost dict to hold all the distances from the start 
        mst_links: List[Link] = []
        mst_nodes: List[Node] = [self.start_point]
        costs: Dict[Node, tuple(float, Link)] = self.init_costs(active_nodes)
        while len(active_nodes) != 0: 
            min_cost_node = self.min_cost(costs)
            mst_links.append(costs[min_cost_node][1])
            mst_nodes.append(min_cost_node)
            active_nodes.remove(min_cost_node)
            self.update_costs(min_cost_node, active_nodes, costs)
        return (mst_links, mst_nodes)

    
    def get_odd(self, links: List[Link], nodes: List[Node]) -> tuple(List[Node], List[Link]):
        links_by_node = links_per_node(links, nodes)
        odd_nodes: List[Node] = [node for node in nodes if links_by_node[node] % 2 != 0]
        odd_links = []
        for node in odd_nodes: 
            odd_links.extend(links_by_node[node])
        return (odd_nodes, odd_links)
        # need to find out which algorithm can be used to find minimum weight perfect matching tree
        # right now looking at edmond's blossom algorithm but seems quite complex (LP formulation)
    
    def relax_double(self) -> List[Link]:
        mst_links, mst_nodes = self.prims_mst()
        odd_nodes, odd_links = self.get_odd(mst_links, mst_nodes)
        max_price = 0.0
        for link in odd_links:
            max_price += link.cost
        perfect_matching_cl = PerfectMatching(odd_nodes, odd_links, max_price)
        extra_links = perfect_matching_cl.perfect_matching(odd_links)
        extra_links.extend(mst_links)
        # TODO relaxation
        # put together the list of links obtained from the perfect matching and that of the mst
        

def links_per_node(links: List[Link], nodes: List[Node]) -> Dict[Node, int]:
    links_by_node: Dict[Node, int] = {}
    for node in nodes: 
        node_links = [link for link in links if link.is_link(node)]
        links_by_node[node] = len(node_links)
    return links_by_node

class PerfectMatching: 
    def __init__(self, nodes: List[Node], links: List[Link], max_price: float,   ):
        self.to_match: List[Node] = nodes
        self.links: List[Link] = links
        self.nodes: List[Node] = nodes # to go back to the initial state 
        self.curr_best_cost: float = max_price
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
        for link in links_unmatched:
            # save initial state 
            # TODO add things to save as you go 
            to_match_init = deepcopy(self.to_match)
            matched_init = deepcopy(self.links_matched)
            nodes_matched_init = deepcopy(self.nodes_matched)
            cost_init = deepcopy(self.cost)

            # add link to the list of matched links and update cost
            self.links_matched.append(link)
            node1, node2 = link.nodes
            # add to nodes that have been matched already
            self.nodes_matched.append(node1)
            self.nodes_matched.append(node2)
            self.to_match.remove(node1)
            self.to_match.remove(node1)

            # check cost is not worse than the best one
            self.cost += link.cost
            if self.cost >= self.curr_best_cost: 
                self.undo(to_match_init, matched_init, nodes_matched_init, cost_init)
                return False
            
            # remove links with the nodes that have just been added
            next_links = []
            for available_link in links_unmatched[1:]:
                if not available_link.is_link(node1) and not available_link.is_link(node2): 
                    next_links.append(available_link)
            
            # check that all the unmatched node have at least one link in the available ones
            # this might be unnecessary as as long as there is a node there will be a link
            links_by_node = links_per_node(next_links, self.to_match)
            for node in self.to_match: 
                if node not in links_by_node.keys(): 
                    self.undo(to_match_init, matched_init, nodes_matched_init, cost_init)
                    return False 
            
            #available_link_init = deepcopy(links_unmatched)
            
            # call the recursion
            self.perfect_matching(next_links)
            # undo for next attempt (as not only we need a valid perfect match, but also the minimum one)
            self.undo(to_match_init, matched_init, nodes_matched_init, cost_init)    
                

        # all possible links have been used
        # check that all nodes have been matched ù
        # TODO determine how to handle the possibility of having odd nodes 
        if len(self.to_match) == 0: 
            if self.cost < self.curr_best_cost: 
                self.curr_best_cost = self.cost 
                self.curr_best_links = self.links_matched
            self.to_match = deepcopy(self.nodes)
            self.cost = deepcopy(self.max_cost)
            return True
        else: return False
        




            



