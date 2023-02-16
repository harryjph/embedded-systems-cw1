import {MapContainer, TileLayer, Marker, Popup} from "react-leaflet";
import L from "leaflet";
import icon from "leaflet/dist/images/marker-icon.png";
import iconShadow from "leaflet/dist/images/marker-shadow.png";
import "leaflet/dist/leaflet.css";
import "./Map.css";
import Routing from "./Routing.js";
import {IoIosTrash} from "react-icons/io";
import {popupHead} from "./popupStyles";

let DefaultIcon = L.icon({
  iconUrl: icon,
  shadowUrl: iconShadow,
  iconSize: [25,41],
  iconAnchor: [12,41],
});

L.Marker.prototype.options.icon = DefaultIcon;

function Map({ points, route }) {
  if (points.length === 0) {
    return <p>No map points!</p>;
  }

  let [centreLat, centreLong] = [0, 0];
  points.forEach((point) => {
    centreLat += point.latitude;
    centreLong += point.longitude;
  });
  centreLat /= points.length;
  centreLong /= points.length;

  const routeData = points.map((point) => [point.latitude, point.longitude]);
  routeData.push(routeData[0]);

  return (
    <MapContainer
      className="z-30 w-full h-full p-10"
      center={[centreLat, centreLong]}
      zoom={13}
      scrollWheelZoom={false}
    >
      <TileLayer
        attribution='&copy; <a href="https://osm.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />
      {route && <Routing RoutingData={routeData} />}
      {points.map((point, i) => {
        return (
          <Marker position={[point.latitude, point.longitude]} key={i}>
            <Popup>
              {point.isBin && <IoIosTrash className="z-30 mx-auto mb-4 h-14 w-14 text-gray-400 dark:text-gray-200" />}
              <div className="m-2" style={popupHead}>
                {point.text}
              </div>
            </Popup>
          </Marker>
        );
      })}
    </MapContainer>
  );
}

export default Map;
