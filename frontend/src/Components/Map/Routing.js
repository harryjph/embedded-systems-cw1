import L from "leaflet";
import { createControlComponent } from "@react-leaflet/core";
import "leaflet-routing-machine";

const createRoutineMachineLayer = ({RoutingData}) => {
    const returnWayPointArr = () => {
        const waypoints = [];
        // waypoints[0] = L.latLng(51.5014, 0.179);
        // waypoints[1] = L.latLng(51.5014, 0.1419);
        for(var i = 0; i < RoutingData.length; i++) {
            waypoints[i] = L.latLng(RoutingData[i][0], RoutingData[i][1]);
        }
        console.log(RoutingData)
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
    // console.log("hi");
    return instance;
};

const Routing = createControlComponent(createRoutineMachineLayer);

export default Routing;
