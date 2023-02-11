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
        

