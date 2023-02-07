import CircularProgressBar from "../BasicComponents/CircularProgressBar.js";
import ModalUserDefined from "../BasicComponents/ModalUserDefined.js";
import AddModalWithRenameSupport from "../ModalWithRenameSupport/AddModalWithRenameSupport.js";
import Backdrop from "../BasicComponents/Backdrop.js";
import Card from "../ui/Card.js";

import classes from "./Bins.module.css";

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
  
  function closeHandler() {

{/* 
  This function interacts with the bins node. 
  What it does is dependent on the parent function which called BinsList, 
  and subsequently this bin.

  If AllOfMyBins:
  - props.Text = "Release This Bin"
  - props.PostRequest will be a function defined in AllOfMyBins which
    handles post requests to /bins/<id>/release
  
  If UnownedBins:
  - props.Text = "Claim This Bin"
  - props.PostRequest will be a function defined in UnownedBins which
    handles post requests to /bins/<id>/claim
*/}

    setBinValue(false);
    setRenameBinValue(false);
    props.PostRequest({ID:props.ID})
  }

  return (

    <Card>
      <div className="flex flex-col z-0">

        <div className="grid gap-1 p-2">
          <div className="flex justify-center items-center">
            <h2 className="flex items-center font-bold pr-2">ID</h2>
            <h2 className="flex items-center font-bold pr-2"> {props.ID}</h2>
          </div>
        </div>

        <div className={"grid gap-1 p-2"}>
          <CircularProgressBar upper_value={props.Fullness} />
        </div>

        <div className="flex justify-center items-center p-2">
          <button data-modal-target="popup-modal" data-modal-toggle="popup-modal" className="text-white bg-[#3b5998] hover:bg-[#3b5998]/90 focus:ring-4 focus:outline-none focus:ring-[#3b5998]/50 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:focus:ring-[#3b5998]/55 mr-2 mb-2" onClick={addHandler}>
            {props.Text}
          </button>
          <button className="text-white bg-[#3b5998] hover:bg-[#3b5998]/90 focus:ring-4 focus:outline-none focus:ring-[#3b5998]/50 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:focus:ring-[#3b5998]/55 mr-2 mb-2" onClick={changeNameHandler}>
            Properties
          </button>
        </div>

        {binValue && <ModalUserDefined isOpen={binValue} ID={props.ID} onCancel={cancelHandler} onConfirm={closeHandler} />}
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
