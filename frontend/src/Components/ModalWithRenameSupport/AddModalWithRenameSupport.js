//  Naming of this file is unintuitive, TLDR this calls the actual modal but supports the PUSH request

import { useNavigate } from "react-router-dom";
import ModalWithRenameSupport from "./ModalWithRenameSupport";
import { apiPostJson } from "../../API";

function AddModalWithRenameSupport(props) {
  const history = useNavigate();

  function modBinsHandler(binsData) {
    apiPostJson("/bins/" + props.ID + "/config", binsData).then(() => {
      history("/my-bins");
    });
  }

  return (
    <ModalWithRenameSupport
      ID={props.ID}
      Name={props.Name}
      Latitude={props.Latitude}
      Longitude={props.Longitude}
      EmptyDistanceReading={props.EmptyDistanceReading}
      FullDistanceReading={props.FullDistanceReading}
      onUpdateBinConfig={modBinsHandler}
      closeHandler={props.onCancel}
    />
  );
}

export default AddModalWithRenameSupport;
