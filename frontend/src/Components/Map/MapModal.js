import Map from "./Map.js";
import React from "react";

const MapModal = (props) => (
    <div className="z-20 h-96 w-96 fixed ">
        <Map AllData={props.AllData}/>
    </div>
    )

export default MapModal;