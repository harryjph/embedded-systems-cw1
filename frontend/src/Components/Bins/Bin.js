import CircularProgressBar from "../BasicComponents/CircularProgressBar.js";
import ModalUserDefined from "../BasicComponents/ModalUserDefined.js";
import AddModalWithRenameSupport from "../ModalWithRenameSupport/AddModalWithRenameSupport.js";
import Card from "../ui/Card.js";
import { useCallback } from "react";

function Bin(props) {
  const binName = props.Name === "" ? "Unnamed (ID: " + props.ID + ")" : props.Name;

  const propertiesButton = props.showPropertiesButton ? (
    <button
      className="m-1 inline-flex items-center px-4 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
      onClick={() => props.foofunctionSeeRenamingModalAndBackdrop(props.ID)}
    >
      Properties
    </button>
  ) : (
    <div />
  );

  const showMap = useCallback(() => {
    props.foofunctionSeeMap([props.ID]);
  }, [props]);

  const showModal = useCallback(() => {
    props.foofunctionSeeModalAndBackdrop(props.ID);
  }, [props]);

  const temperature = props.Temperature >= 40 ? <span className="text-[#ff0000]">High (> 40°C)</span> : <span className="text-[#00ff20]">Normal</span>

  return (
    <Card className="block w-72">
      <div className="flex flex-col z-0 px-5">
        <div className="grid gap-1 p-2">
          <div className="flex justify-center items-center">
            <h2 className="flex items-center font-bold text-3xl pr-2">{binName}</h2>
          </div>
        </div>

        <div className={"grid gap-1 p-2"}>
          <CircularProgressBar value={props.Fullness} />
        </div>

        <div className={"flex gap-2 p-3 ml-auto mr-auto"}>
          <span className="font-bold">Temperature: {temperature}</span>
        </div>

        <div className="flex justify-center items-center p-2">
          <button
            data-modal-target="popup-modal"
            data-modal-toggle="popup-modal"
            className="m-1 inline-flex items-center px-4 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            onClick={showModal}
          >
            {props.Text}
          </button>
          {propertiesButton}

          <button
            data-modal-target="popup-modal"
            data-modal-toggle="popup-modal"
            className="m-1 inline-flex items-center px-4 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            onClick={showMap}
          >
            Map
          </button>
        </div>

        {props.varSeeModalAndBackdrop && (
          <ModalUserDefined ID={props.ID} onCancel={props.foocancelModal} PostRequest={props.PostRequest} />
        )}
        {props.varSeeRenamingModalAndBackdrop && (
          <AddModalWithRenameSupport
            ID={props.ID}
            Name={props.Name}
            Latitude={props.Latitude}
            Longitude={props.Longitude}
            Fullness={props.Fullness}
            EmptyDistanceReading={props.EmptyDistanceReading}
            FullDistanceReading={props.FullDistanceReading}
            onCancel={props.foocancelModal}
          />
        )}
      </div>
    </Card>
  );
}

export default Bin;
