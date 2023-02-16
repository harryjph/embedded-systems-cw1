import Bin, {getBinLabel} from "./Bin.js";
import Backdrop from "../BasicComponents/Backdrop.js";
import MapModal from "../Map/MapModal.js";
import {useState} from "react";
import {apiPostJson} from "../../API.js";

function BinsList(props) {
  const [SeeMap, setSeeMap] = useState(false);
  const [SeeRoute, setSeeRoute] = useState(false);
  const [MapData, setMapData] = useState([]);
  const [modalAndBackdropFor, setModalAndBackdropFor] = useState(-1);
  const [SeeRenamingModalAndBackdrop, setSeeRenamingModalAndBackdrop] = useState(-1);

  function binToMapPoint(bin) {
    return {
      latitude: bin.config.latitude,
      longitude: bin.config.longitude,
      text: getBinLabel(bin.config.name, bin.id),
      isBin: true,
    };
  }

  function SeeRoutingMap() {
    const successCallback = (position) => {
      apiPostJson("/bins/route", {
        starting_latitude: position.coords.latitude,
        starting_longitude: position.coords.longitude,
      })
        .then((r) => r.json())
        .then((response) => {
          const binPoints = response.route
            .map((id) => props.AllData.find((bin) => bin.id === id))
            .map(binToMapPoint);
          let initialPosition = {
            latitude: position.coords.latitude,
            longitude: position.coords.longitude,
            text: "You are here",
            isBin: false,
          };
          setMapData([initialPosition].concat(binPoints));
          setSeeMap(true);
          setSeeRoute(true);
        });
    };

    const errorCallback = (error) => {
      alert("Error getting location: " + error);
    };

    navigator.geolocation.getCurrentPosition(successCallback, errorCallback);
  }

  function functionSeeMap(mapIds) {
    let mapData = props.AllData.filter((bin) => mapIds.includes(bin.id))
      .map(binToMapPoint);
    setMapData(mapData);
    setSeeRoute(false);
    setSeeMap(true);
  }

  function functionSeeModalAndBackdrop(id) {
    setModalAndBackdropFor(id);
  }

  function functionSeeRenamingModalAndBackdrop(id) {
    setSeeRenamingModalAndBackdrop(id);
  }

  function cancelModal() {
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
      />
    );
  });

  return (
    <div>
      {SeeMap && <MapModal points={MapData} route={SeeRoute} />}
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
