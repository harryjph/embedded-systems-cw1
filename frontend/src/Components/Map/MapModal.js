import Map from "./Map.js";
import React from "react";

const MapModal = (props) => (
        <div className="modal items-center justify-center w-1/2 h-1/2">
            <Map AllData={props.AllData}/>
        </div>
    )

export default MapModal;