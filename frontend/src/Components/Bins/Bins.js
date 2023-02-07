import CircularProgressBar from "../BasicComponents/CircularProgressBar.js";
import ModalUserDefined from "../BasicComponents/ModalUserDefined.js";
import AddModalWithRenameSupport from "../ModalWithRenameSupport/AddModalWithRenameSupport.js";
import Backdrop from "../BasicComponents/Backdrop.js";
import Card from "../ui/Card.js";

import { useState } from "react";

function Bins(props) {
  const [binValue, setBinValue] = useState(false);
  const [renameBinValue, setRenameBinValue] = useState(false);

  function addHandler() {
    setBinValue(true);
  }

  function changeNameHandler() {
    setRenameBinValue(true);
  }

  function cancelHandler() {
    setBinValue(false);
    setRenameBinValue(false);
  }

  /**
   * This function interacts with the bins node.
   * What it does is dependent on the parent function which called BinsList,
   * and subsequently this bin.
   *
   * If AllOfMyBins:
   * - props.Text = "Release This Bin"
   * - props.PostRequest will be a function defined in AllOfMyBins which
   *   handles post requests to /bins/<id>/release
   *
   * If UnownedBins:
   * - props.Text = "Claim This Bin"
   * - props.PostRequest will be a function defined in UnownedBins which
   *   handles post requests to /bins/<id>/claim
   */
  function closeHandler() {
    setBinValue(false);
    setRenameBinValue(false);
    props.PostRequest({ ID: props.ID });
  }

  return (
    <Card className="block max-w-sm">
      <div className="flex flex-col z-0 px-5">
        <div className="grid gap-1 p-2">
          <div className="flex justify-center items-center">
            <h2 className="flex items-center font-bold pr-2">{props.Name}</h2>
          </div>
        </div>

        <div className={"grid gap-1 p-2"}>
          <CircularProgressBar upper_value={props.Fullness} />
        </div>
        <div className="flex justify-center items-center p-2">
          <button
            data-modal-target="popup-modal"
            data-modal-toggle="popup-modal"
            className="m-1 inline-flex items-center px-4 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            onClick={addHandler}
          >
            {props.Text}
          </button>
          <button
            className="m-1 inline-flex items-center px-4 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            onClick={changeNameHandler}
          >
            Properties
          </button>
        </div>

        {binValue && (
          <ModalUserDefined isOpen={binValue} ID={props.ID} onCancel={cancelHandler} onConfirm={closeHandler} />
        )}

        {binValue && <Backdrop onClick={cancelHandler} />}
        {renameBinValue && (
          <AddModalWithRenameSupport
            ID={props.ID}
            Name={props.Name}
            Latitude={props.Latitude}
            Longitude={props.Longitude}
            Fullness={props.Fullness}
            EmptyDistanceReading={props.EmptyDistanceReading}
            FullDistanceReading={props.FullDistanceReading}
            onCancel={cancelHandler}
            onConfirm={closeHandler}
          />
        )}
        {renameBinValue && <Backdrop onClick={cancelHandler} />}
      </div>
    </Card>
  );
}

export default Bins;
