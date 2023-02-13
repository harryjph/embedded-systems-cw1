import Bin from "./Bin.js";

import ModalUserDefined from "../BasicComponents/ModalUserDefined.js";
import AddModalWithRenameSupport from "../ModalWithRenameSupport/AddModalWithRenameSupport.js";
import Backdrop from "../BasicComponents/Backdrop.js";

import MapModal from "../Map/MapModal.js";

import useOutsideAlerter from "../BasicComponents/CustomHook";

import { useState } from "react";

//import {Avatar} from "flowbite-react"
import { Modal, Button } from "flowbite-react";

function BinsList(props) {
  const [SeeMap, setSeeMap] = useState(false);
  const [MapData, setMapData] = useState([]);
  const [modalAndBackdropFor, setModalAndBackdropFor] = useState(-1);
  const [SeeRenamingModalAndBackdrop, setSeeRenamingModalAndBackdrop] = useState(-1);

  function SeeRoutingMap() {
    setMapData(props.AllData);
    setSeeMap(true);
  }

  function functionSeeMap(mapIds) {
    let mapData = props.AllData.filter((bin) => mapIds.includes(bin.id));
    setMapData(mapData);
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

  let binsWidgets = props.AllData.map((bin) => {
    const seeModal = modalAndBackdropFor === bin.id;
    const seeProperties = SeeRenamingModalAndBackdrop === bin.id;
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
      {SeeMap && <MapModal AllData={MapData} />}
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
