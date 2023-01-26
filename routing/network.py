from typing import List, Dict
from time import timedelta, datetime
from dataclasses import dataclass

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

    def links_per_node(self, links: List[Link], nodes: List[Node]) -> Dict[Node, int]:
        links_by_node: Dict[Node, int] = {}
        for node in nodes: 
            node_links = [link for link in links if link.is_link(node)]
            links_by_node[node] = len(node_links)
        return links_by_node
    
    def perfect_matching(self) -> List[Link]:
        mst_links, mst_nodes = self.prims_mst()
        links_by_node = self.links_per_node(mst_links, mst_nodes)
        odd_links: List[Node] = [node for node in mst_nodes if links_by_node[node] % 2 != 0]
        # need to find out which algorithm can be used to find minimum weight perfect matching tree
        # right now looking at edmond's blossom algorithm but seems quite complex (LP formulation)


