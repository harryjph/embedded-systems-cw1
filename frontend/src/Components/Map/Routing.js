import L from "leaflet";
import { createControlComponent } from "@react-leaflet/core";
import "leaflet-routing-machine";

const createRoutineMachineLayer = ({RoutingData}) => {
    const returnWayPointArr = () => {
        const waypoints = [];
        for(var i = 0; i < RoutingData.length; i++) {
            waypoints[i] = L.latLng(RoutingData[0], RoutingData[1]);
        }
        
        return waypoints;
    }
const instance = L.Routing.control({
    waypoints: returnWayPointArr(),
    lineOptions: {
    styles: [{ color: "#6FA1EC", weight: 4 }]
    },
    show: false,
    addWaypoints: false,
    routeWhileDragging: true,
    draggableWaypoints: true,
    fitSelectedRoutes: true,
    showAlternatives: false
});
console.log(instance);
return instance;
};

const Routing = createControlComponent(createRoutineMachineLayer);

export default Routing;
