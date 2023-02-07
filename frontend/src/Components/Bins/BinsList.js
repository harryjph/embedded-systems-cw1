import Bins from "./Bins.js";

function BinsList(props) {
  let binsWidgets = props.AllData.map((bin) => (
    <Bins
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
    />
  ));

  return <div className="flex flex-wrap justify-center items-center space-x-1 space-y-1">{binsWidgets}</div>;
}

export default BinsList;
