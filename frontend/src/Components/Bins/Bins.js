import CircularProgressBar from "../BasicComponents/CircularProgressBar.js";
import Modal from "../BasicComponents/Modal.js";
import AddModalWithRenameSupport from "../ModalWithRenameSupport/AddModalWithRenameSupport.js";
import Backdrop from "../BasicComponents/Backdrop.js";
import Card from "../ui/Card.js";

import classes from "./Bins.module.css";

import { useState } from "react";

function Bins(props) {
  const [binValue, setBinValue] = useState(false);
  const [renameBinValue, setRenameBinValue] = useState(false);

  function addHandler() {
    props.PostRequest({ID:props.ID})
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
      <h2 className={"classes.content"}> ID {props.ID}</h2>

      <div className={"classes.content"}>
        <CircularProgressBar upper_value={props.Fullness} />
      </div>

      <div className="classes.actions">
        <button className="btn" onClick={addHandler}>
          {props.Text}
        </button>
        <button className="btn" onClick={changeNameHandler}>
          Properties
        </button>
      </div>

      {binValue && (
        <Modal ID={props.ID} onCancel={cancelHandler} onConfirm={closeHandler} />
      )}
      {binValue && <Backdrop onClick={cancelHandler} />}

      {renameBinValue && (
        <AddModalWithRenameSupport
          ID={props.ID}
          Name={props.Name}
          Latitude={props.Latitude}
          Longitude={props.Longitude}
          Fullness={props.Fullness}
          Threshold={props.Threshold}
          onCancel={closeHandler}
          onConfirm={closeHandler}
        />
      )}
      {renameBinValue && <Backdrop onClick={cancelHandler} />}
    </Card>
  );
}

export default Bins;
