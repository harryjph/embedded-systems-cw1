//  MODAL RENAME NEW

import Card from "../ui/Card";
import classes from "./AddNewBinsForm.module.css";

import { useRef } from "react";

function ModalWithRenameSupport(props) {
  const binNameInputRef = useRef();
  const binLatInputRef = useRef();
  const binLongInputRef = useRef();
  const binEmptyDistanceReadingInputRef = useRef();
  const binFullDistanceReadingInputRef = useRef();

  function submitHandler(event) {
    event.preventDefault();

    const enteredName = binNameInputRef.current.value;
    const enteredLat = parseFloat(binLatInputRef.current.value);
    const enteredLong = parseFloat(binLongInputRef.current.value);
    const enteredEmptyDistanceReading = parseFloat(binEmptyDistanceReadingInputRef.current.value);
    const enteredFullDistanceReading = parseFloat(binFullDistanceReadingInputRef.current.value);

    const binsData = {
      name: enteredName,
      latitude: enteredLat,
      longitude: enteredLong,
      empty_distance_reading: enteredEmptyDistanceReading,
      full_distance_reading: enteredFullDistanceReading,
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
              placeholder={props.Name}
              ref={binNameInputRef}
            />
          </div>

          <div className={classes.control}>
            <label htmlFor="title">Bin Latitude</label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              type="text"
              required
              id="latitude"
              placeholder={props.Latitude}
              ref={binLatInputRef}
            />
          </div>

          <div className={classes.control}>
            <label htmlFor="title">Bin Longitude</label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              type="text"
              required
              id="longitude"
              placeholder={props.Longitude}
              ref={binLongInputRef}
            />
          </div>

          <div className={classes.control}>
            <label htmlFor="title">Bin Empty Distance Reading</label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              type="text"
              required
              id="fullness"
              placeholder={props.EmptyDistanceReading}
              ref={binEmptyDistanceReadingInputRef}
            />
          </div>

          <div className={classes.control}>
            <label htmlFor="title">Bin Full Distance Reading</label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              type="text"
              required
              id="threshold"
              placeholder={props.FullDistanceReading}
              ref={binFullDistanceReadingInputRef}
            />
          </div>

          <div className="flex items-center justify-between">
            <button className="text-white bg-[#3b5998] hover:bg-[#3b5998]/90 focus:ring-4 focus:outline-none focus:ring-[#3b5998]/50 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:focus:ring-[#3b5998]/55 mr-2 mb-2" onClick={submitHandler}>
              Save Changes
            </button>

            <button className="text-white bg-[#3b5998] hover:bg-[#3b5998]/90 focus:ring-4 focus:outline-none focus:ring-[#3b5998]/50 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:focus:ring-[#3b5998]/55 mr-2 mb-2" onClick={onCancel}>
              Cancel
            </button>
          </div>
        </div>
      </form>
    </div>
  );
}

export default ModalWithRenameSupport;
