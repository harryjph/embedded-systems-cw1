from typing import List
from time import timedelta, datetime

class Node: 
    def __init__(self, coordinates: tuple(float, float), fill_level: float):
        self.x_coord: float = coordinates[0]
        self.y_coord: float = coordinates[1]
        self.fullness: float = fill_level 
        self.time_to_full: timedelta = timedelta(seconds = 0) # this will be updated once there is more data available on past
        self.last_emptied: datetime = datetime.now() # to initialise

    def update_fullness(self, fill_level):
        # TODO make it so that the frequency with which it requires fullness data is proportional 
        # to the time it has passed from the last emptying action (using also statistical data 
        # related to each single bin on how long it takes to fill on average)
        self.fullness = fill_level # maybe add the prompt to get data on fullness from here?  
        if fill_level == 0.0: # might have to change this based on how it is conveyed that the bin has been emptied 
            self.last_emptied = datetime.now()

class Link: 
    cost: float
    nodes: tuple(Node, Node)

class Network: 
    nodes: List[Node]
    links: List[Link] # all the possible links
    start_point: Node # place where collection must start
    end_point: Node # place where collection must end
    
    # get only the links that connect two nodes that need servicing
    def get_active_links(self) -> List[Link]:
        pass
