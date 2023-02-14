import L from "leaflet";
import { createControlComponent } from "@react-leaflet/core";
import "leaflet-routing-machine";

const createRoutineMachineLayer = (props) => {
    var routes = [];
    for(var j = 1; j < props.RoutingData.length; j+=2){
        routes.push(L.Routing.control({
            waypoints: 
                [
                    L.latLng(props.RoutingData[j-1].latitude,       props.RoutingData[j-1].longitude),
                    L.latLng(props.RoutingData[j].latitude,         props.RoutingData[j].longitude)
                ],
            show: false,
            createMarker: 
            function() { 
                return null; 
            }
        }));


    }

    return routes;
};

const Routing = createControlComponent(createRoutineMachineLayer);

export default Routing;
