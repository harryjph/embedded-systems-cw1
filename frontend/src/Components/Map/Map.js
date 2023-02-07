import { MapContainer, TileLayer, Marker, Popup } from "react-leaflet";
import L from "leaflet";
import icon from "leaflet/dist/images/marker-icon.png";
import iconShadow from "leaflet/dist/images/marker-shadow.png";
import "leaflet/dist/leaflet.css";

import { IoIosTrash } from "react-icons/io";

import "./Map.css";

import { popupHead } from "./popupStyles";

let DefaultIcon = L.icon({
  iconUrl: icon,
  shadowUrl: iconShadow,
});

L.Marker.prototype.options.icon = DefaultIcon;

function Map(props) {
  return (
    <div>
      <MapContainer className="z-0 h-96 w-96" center={[51.505, -0.09]} zoom={13} scrollWheelZoom={false}>
        <TileLayer
          attribution='&copy; <a href="https://osm.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
        {props.AllData.map((popup) => {
          return (
            <Marker position={[popup.config.latitude, popup.config.longitude]} key={popup.id}>
              <Popup className="request-popup">
                <IoIosTrash className="z-1 mx-auto mb-4 h-14 w-14 text-gray-400 dark:text-gray-200" />
                <div className="m-2" style={popupHead}>
                  {popup.config.name}
                  <br />
                  {Math.floor((100 * popup.config.empty_distance_reading) / popup.config.full_distance_reading) +
                    "% Full"}
                </div>
              </Popup>
            </Marker>
          );
        })}
      </MapContainer>
    </div>
  );
}

export default Map;