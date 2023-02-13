import React from "react";
import { Button } from "flowbite-react";
import { HiOutlineExclamationCircle } from "react-icons/hi";
import { useState } from "react";

function ModalUserDefined(props) {
  const [binID, setBinID] = useState(props.ID);

  function onCancel() {
    props.closeHandler();
  }
  return (
    <div className="modal">
      <div className="text-center">
        <HiOutlineExclamationCircle className="z-1 mx-auto mb-4 h-14 w-14 text-gray-400 dark:text-gray-200" />
        <h3 className="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">Are you sure?</h3>
        <div className="flex justify-center items-center">
          <Button
            className="m-1 text-white bg-[#C81E1E] hover:bg-[#9B1C1C]/90 
            focus:ring-4 focus:outline-none focus:ring-[#3b5998]/50 font-medium rounded-lg text-sm px-5 py-1 text-center inline-flex items-center dark:focus:ring-[#3b5998]/55 mr-2 mb-2
            "
            onClick={submitHandler}
          >
            Yes
          </Button>
          <Button
            className="m-1 text-white bg-[#6B7280] hover:bg-[#4B5563]/90 focus:ring-4 focus:outline-none focus:ring-[#3b5998]/50 font-medium rounded-lg text-sm px-5 py-1 text-center inline-flex items-center dark:focus:ring-[#3b5998]/55 mr-2 mb-2"
            onClick={onCancel}
          >
            Cancel
          </Button>
        </div>
      </div>
    </div>
  );
}

export default ModalUserDefined;
