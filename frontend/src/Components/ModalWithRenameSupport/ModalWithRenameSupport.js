//  MODAL RENAME NEW

import Card from "../ui/Card";
import classes from "./AddNewBinsForm.module.css";

import { useRef } from "react";

function ModalWithRenameSupport(props) {
  const binNameInputRef = useRef();
  const binLatInputRef = useRef();
  const binLongInputRef = useRef();
  const binFullnessInputRef = useRef();
  const binFullnessThresholdRef = useRef();

  function submitHandler(event) {
    event.preventDefault();

    const enteredName = binNameInputRef.current.value;
    const enteredLat = binLatInputRef.current.value;
    const enteredLong = binLongInputRef.current.value;
    const enteredFullness = binFullnessInputRef.current.value;
    const enteredThreshold = binFullnessThresholdRef.current.value;

    const binsData = {
      id: props.ID,
      config: {
        name: enteredName,
        latitude: enteredLat,
        longitude: enteredLong,
        full_threshold: enteredThreshold,
      },
      fullness: enteredFullness,
    };

    props.onAddBins(binsData);
    props.onConfirm();
  }

  function onCancel() {
    props.closeHandler();
  }

  return (
    <div className="modal">
      <form>
        <div className="relative z-0 w-full mb-6 group">
          
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
            <label htmlFor="title">Bin Fullness</label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              type="text"
              required
              id="fullness"
              placeholder={Math.floor(props.Fullness)}
              ref={binFullnessInputRef}
            />
          </div>

          <div className={classes.control}>
            <label htmlFor="title">Bin Fullness Threshold</label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              type="text"
              required
              id="threshold"
              value={props.Threshold}
              ref={binFullnessThresholdRef}
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
