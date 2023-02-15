import L from "leaflet";
import {createControlComponent} from "@react-leaflet/core";
import "leaflet-routing-machine";

const createRoutineMachineLayer = ({RoutingData}) => {
    const returnWayPointArr = () => {
        const waypoints = [];
        for(let i = 0; i < RoutingData.length; i++) {
            waypoints[i] = L.latLng(RoutingData[i][0], RoutingData[i][1]);
        }
    return waypoints;
        
    }
  return L.Routing.control({
      waypoints: returnWayPointArr(),
      lineOptions: {
        styles: [{color: "#6FA1EC", weight: 4}]
      },
      show: false,
      addWaypoints: false,
      routeWhileDragging: true,
      draggableWaypoints: true,
      fitSelectedRoutes: true,
      showAlternatives: false
    });
};

const Routing = createControlComponent(createRoutineMachineLayer);

export default Routing;
