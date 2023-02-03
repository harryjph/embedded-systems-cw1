//  Naming of this file is unintuitive, TLDR this calls the actual modal but supports the PUSH request

import { useNavigate } from "react-router-dom";
import ModalWithRenameSupport from "./ModalWithRenameSupport";

function AddModalWithRenameSupport(props) {
  const history = useNavigate();

  function modBinsHandler(binsData) {
    fetch("https://es1.harryphillips.co.uk/bins/{props.ID}/config", {
      method: "POST",
      body: JSON.stringify(binsData),
      headers: {
        "Content-Type": "application/json",
      },
    }).then(() => {
      console.log("posted form to rename");
      history("/");
    });
  }

  return (
    <ModalWithRenameSupport
      ID={props.ID}
      Name={props.Name}
      Latitude={props.Latitude}
      Longitude={props.Longitude}
      Fullness={props.Fullness}
      Threshold={props.Threshold}
      onAddBins={modBinsHandler}
      closeHandler={props.onCancel}
    />
  );
}

export default AddModalWithRenameSupport;
