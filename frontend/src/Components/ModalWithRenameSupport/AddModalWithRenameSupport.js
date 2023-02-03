//  Naming of this file is unintuitive, TLDR this calls the actual modal but supports the PUSH request

import { useNavigate } from "react-router-dom";
import ModalWithRenameSupport from "./ModalWithRenameSupport";
import {apiPostJson} from "../../API";

function AddModalWithRenameSupport(props) {
  const history = useNavigate();

  function modBinsHandler(binsData) {
    apiPostJson("/bins/" + props.ID + "/config", binsData).then(() => {
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
