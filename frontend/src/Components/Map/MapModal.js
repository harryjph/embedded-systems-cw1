import Map from "./Map.js";
import React from "react";

const MapModal = (props) => (
        <div className="z-40 modal fixed items-center justify-center w-1/2 h-1/2" style={
                {  
                    top: "calc(50% - 15rem)",
                    left: "calc(50% - 29rem)",
                }
            }
        >
            <Map AllData={props.AllData}/>
        </div>
    )

export default MapModal;