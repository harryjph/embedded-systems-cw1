import Bins from "./Bins.js";
import classes from "./BinsList.module.css";

import { useEffect, useState } from "react";

function BinsList(props) {
  const [bins, setBins] = useState([]);

  let binsWidgets = props.AllData.map(
    (bin) => (
      <Bins 
        PostRequest={props.PostRequest}
        Text={props.Text}
        key={bin.id}
        ID={bin.id}
        Name={bin.config.name}
        Latitude={bin.config.latitude}
        Longitude={bin.config.longitude}
        Threshold={bin.config.full_threshold}
        Fullness={Math.floor(bin.fullness * 100)}
      />
    )
  );
  

  return <div className="flex flex-wrap justify-center items-center space-x-1 space-y-1">{binsWidgets}</div>;
}

export default BinsList;
