import Map from "./Map.js";
import React from "react";

const MapModal = (props) => (
    <div className="z-20 h-95 h-fixed w-fixed w-95 fixed">
        <Map AllData={props.AllData}/>
    </div>
    )

export default MapModal;