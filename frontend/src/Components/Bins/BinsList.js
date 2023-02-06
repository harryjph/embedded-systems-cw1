import Bins from "./Bins.js";

function BinsList(props) {
  let binsWidgets = props.AllData.map((bin) => (
    <Bins
      key={bin.id}
      ID={bin.id}
      Name={bin.config.name}
      Latitude={bin.config.latitude}
      Longitude={bin.config.longitude}
      Threshold={bin.config.full_threshold}
      Fullness={Math.floor(bin.fullness * 100)}
    />
  ));

  return <div className="classes.list">{binsWidgets}</div>;
}

export default BinsList;
