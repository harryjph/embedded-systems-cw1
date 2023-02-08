import Bin from "./Bin.js";
import { useState } from "react";

import ModalUserDefined from "../BasicComponents/ModalUserDefined.js";
import AddModalWithRenameSupport from "../ModalWithRenameSupport/AddModalWithRenameSupport.js";
import Backdrop from "../BasicComponents/Backdrop.js";

function BinsList(props) {
  const [SeeMap, setSeeMap] = useState(false);
  const [SeeModalAndBackdrop, setSeeModalAndBackdrop] = useState(false);
  const [SeeRenamingModalAndBackdrop, setSeeRenamingModalAndBackdrop] = useState(false);

  function functionSeeMap() {
    setSeeMap(true);
  }

  function functionSeeModalAndBackdrop() {
    setSeeModalAndBackdrop(true);
  }

  function functionSeeRenamingModalAndBackdrop() {
    setSeeRenamingModalAndBackdrop(true);
  }

  function cancelModal() {
    setSeeMap(false);
    setSeeModalAndBackdrop(false);
    setSeeRenamingModalAndBackdrop(false);
  }
  
  function confirmModalAndBackdrop() {
    setSeeMap(false);
    setSeeModalAndBackdrop(false);
    setSeeRenamingModalAndBackdrop(false);
  }
  
  function ShowModal() {
    setSeeModalAndBackdrop(true);
  }
  
  function ShowRenamingModal() {
    setSeeRenamingModalAndBackdrop(true);
  }



  let binsWidgets = props.AllData.map((bin) => (
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
      showPropertiesButton={props.showPropertiesButton}

      varSeeMap={SeeMap}
      varSeeModalAndBackdrop={SeeModalAndBackdrop}
      varSeeRenamingModalAndBackdrop={SeeRenamingModalAndBackdrop}

      foofunctionSeeMap={functionSeeMap}
      foofunctionSeeModalAndBackdrop={functionSeeModalAndBackdrop}
      foofunctionSeeRenamingModalAndBackdrop={functionSeeRenamingModalAndBackdrop}
      foocancelModal={cancelModal}
      fooconfirmModalAndBackdrop={confirmModalAndBackdrop}
      fooShowModal={ShowModal}
      fooShowRenamingModal={ShowRenamingModal}
    />
  ));

  return <div>
    {(SeeModalAndBackdrop || SeeRenamingModalAndBackdrop) && 
      (<Backdrop 
        onClick={cancelModal}
      />)
    }
    <div className="flex flex-wrap justify-center items-center space-x-1 space-y-1">
      {binsWidgets}

    </div>
  </div>;
{/*
        {binValue && (
          <ModalUserDefined 
            isOpen={binValue} 
            ID={props.ID} 
            onCancel={cancelHandler} 
            PostRequest={props.PostRequest} 
          />
        )}

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
          />
        )}

        {renameBinValue && 
          <Backdrop 
            onClick={cancelHandler} 
          />
        }
*/}
}

export default BinsList;
