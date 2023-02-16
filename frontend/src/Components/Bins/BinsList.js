import Bin from "./Bin.js";
import Backdrop from "../BasicComponents/Backdrop.js";
import MapModal from "../Map/MapModal.js";
import { useState } from "react";
import { apiPostJson } from "../../API.js";

function BinsList(props) {
  const [SeeMap, setSeeMap] = useState(false);
  const [SeeRoute, setSeeRoute] = useState(false);
  const [MapData, setMapData] = useState([]);
  const [RouteData, setRouteData] = useState([]);
  const [modalAndBackdropFor, setModalAndBackdropFor] = useState(-1);
  const [SeeRenamingModalAndBackdrop, setSeeRenamingModalAndBackdrop] = useState(-1);

  function SeeRoutingMap() {
    const successCallback = (position) => {
      apiPostJson("/bins/route", {
        starting_latitude: position.coords.latitude,
        starting_longitude: position.coords.longitude,
      })
        .then((r) => r.json())
        .then((response) => {
          const binIds = response.route;
          let mapData = props.AllData.filter((bin) => binIds.includes(bin.id));
          setMapData(mapData);
          let InitialLatLng = [position.coords.latitude, position.coords.longitude];
          let LatterLatLongData = mapData.map((bin) => {
            return [bin.config.latitude, bin.config.longitude];
          });
          let LatLongData = [InitialLatLng].concat(LatterLatLongData);
          setRouteData(LatLongData);
          setSeeMap(true);
        });
      setSeeRoute(true);
    };

    const errorCallback = (error) => {
      alert("Error getting location: " + error);
    };

    navigator.geolocation.getCurrentPosition(successCallback, errorCallback);
  }

  function functionSeeMap(mapIds) {
    let mapData = props.AllData.filter((bin) => mapIds.includes(bin.id));
    setMapData(mapData);
    setSeeMap(true);
  }

  function functionSeeRoute() {
    setSeeRoute(true);
  }

  function functionDontSeeRoute() {
    setSeeRoute(false);
  }

  function functionSeeModalAndBackdrop(id) {
    setModalAndBackdropFor(id);
  }

  function functionSeeRenamingModalAndBackdrop(id) {
    setSeeRenamingModalAndBackdrop(id);
  }

  function cancelModal() {
    setSeeRoute(false);
    setSeeMap(false);
    setModalAndBackdropFor(-1);
    setSeeRenamingModalAndBackdrop(-1);
  }

  if (props.AllData.length === 0) {
    return <p className="text-center text-2xl">There are no bins!</p>;
  }

  let binsWidgets = props.AllData.map((bin) => {
    const seeModal = modalAndBackdropFor === bin.id;
    const seeProperties = SeeRenamingModalAndBackdrop === bin.id;
    // noinspection JSUnresolvedVariable
    return (
      <Bin
        PostRequest={props.PostRequest}
        Text={props.Text}
        key={bin.id}
        ID={bin.id}
        Name={bin.config.name}
        Latitude={bin.config.latitude}
        Longitude={bin.config.longitude}
        EmptyDistanceReading={bin.config.empty_distance_reading}
        FullDistanceReading={bin.config.full_distance_reading}
        Fullness={Math.floor(bin.fullness * 100)}
        Temperature={bin.temperature}
        Humidity={bin.humidity}
        showPropertiesButton={props.showPropertiesButton}
        varSeeMap={SeeMap}
        varSeeModalAndBackdrop={seeModal}
        varSeeRenamingModalAndBackdrop={seeProperties}
        foofunctionSeeMap={functionSeeMap}
        foofunctionSeeModalAndBackdrop={functionSeeModalAndBackdrop}
        foofunctionSeeRenamingModalAndBackdrop={functionSeeRenamingModalAndBackdrop}
        foocancelModal={cancelModal}
        foofunctionSeeRoute={functionSeeRoute}
        foofunctionDontSeeRoute={functionDontSeeRoute}
      />
    );
  });

  return (
    <div>
      {SeeMap && <MapModal AllData={MapData} RoutingData={RouteData} SeeRoute={SeeRoute} />}
      <div className="flex items-center justify-center m-5">
        <button
          className="m-1 inline-flex items-center px-4 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
          onClick={SeeRoutingMap}
        >
          Compute Route
        </button>
      </div>
      <div className="flex flex-wrap w-[calc(100vw-2.5rem)] justify-center gap-x-1">
        {(modalAndBackdropFor >= 0 || SeeRenamingModalAndBackdrop >= 0 || SeeMap) && <Backdrop onClick={cancelModal} />}
        {binsWidgets}
      </div>
    </div>
  );
}

export default BinsList;
