//  MODAL RENAME NEW

import classes from "./AddNewBinsForm.module.css";

import {useState} from "react";

function ModalWithRenameSupport(props) {
  const [binName, setBinName] = useState(props.Name);
  const [binLatitude, setBinLatitude] = useState(props.Latitude);
  const [binLongitude, setBinLongitude] = useState(props.Longitude);
  const [binEmptyDistanceReading, setBinEmptyDistanceReading] = useState(props.EmptyDistanceReading);
  const [binFullDistanceReading, setBinFullDistanceReading] = useState(props.FullDistanceReading);

  function submitHandler() {
    const binsData = {
      name: binName,
      latitude: parseFloat(binLatitude),
      longitude: parseFloat(binLongitude),
      empty_distance_reading: parseFloat(binEmptyDistanceReading),
      full_distance_reading: parseFloat(binFullDistanceReading),
    };

    props.onUpdateBinConfig(binsData);
    props.closeHandler();
  }

  function onCancel() {
    props.closeHandler();
  }

  return (
    <div className="modal">
      <form>
        <div className="relative z-1 w-full mb-6 group">
          <div className={classes.control}>
            <label htmlFor="title">Bin Name</label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              type="text"
              required
              id="name"
              value={binName}
              onChange={(e) => setBinName(e.target.value)}
            />
          </div>

          <div className={classes.control}>
            <label htmlFor="title">Bin Latitude</label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              type="text"
              required
              id="latitude"
              value={binLatitude}
              onChange={(e) => setBinLatitude(e.target.value)}
            />
          </div>

          <div className={classes.control}>
            <label htmlFor="title">Bin Longitude</label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              type="text"
              required
              id="longitude"
              value={binLongitude}
              onChange={(e) => setBinLongitude(e.target.value)}
            />
          </div>

          <div className={classes.control}>
            <label htmlFor="title">Bin Empty Distance Reading</label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              type="text"
              required
              id="fullness"
              value={binEmptyDistanceReading}
              onChange={(e) => setBinEmptyDistanceReading(e.target.value)}
            />
          </div>

          <div className={classes.control}>
            <label htmlFor="title">Bin Full Distance Reading</label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              type="text"
              required
              id="threshold"
              value={binFullDistanceReading}
              onChange={(e) => setBinFullDistanceReading(e.target.value)}
            />
          </div>

          <div className="flex justify-center items-center">
            <button
              className="m-1 p-3 inline-flex items-center px-4 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
              onClick={submitHandler}
            >
              Save Changes
            </button>

            <button
              className="m-1 p-3 inline-flex items-center px-4 py-2 text-sm font-medium text-center text-white bg-red-700 rounded-lg hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
              onClick={onCancel}
            >
              Cancel
            </button>
          </div>
        </div>
      </form>
    </div>
  );
}

export default ModalWithRenameSupport;
