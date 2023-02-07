import React from 'react';
import { Tooltip, Button, Modal} from 'flowbite-react'
import { HiOutlineExclamationCircle } from "react-icons/hi";

function ModalUserDefined(props) {
  function YesSelect() {
    console.log("Yes has been selected");
    props.onConfirm();
  }
  function NoSelect() {
    console.log("No has been selected");
    props.onCancel();
  }

//   <div>
//   <p>
//     Are you sure?
//   </p>
//   <div className="flex items-center justify-between">
//     <button className="" onClick={YesSelect}>Yes</button>
//     <button className="" onClick={NoSelect}>Cancel</button>
//   </div>
// </div>

  return (
    <div className="modal">
    {console.log(props.isOpen)}
      <div className="text-center">
        <HiOutlineExclamationCircle className="z-1 mx-auto mb-4 h-14 w-14 text-gray-400 dark:text-gray-200" />
        <h3 className="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
          Are you sure you want to delete this product?
        </h3>
        <div className="flex justify-center gap-4">
          <Button
            color="failure"
            onClick={YesSelect}
          >
            Yes
          </Button>
          <Button
            color="gray"
            onClick={NoSelect}
          >
            Cancel
          </Button>
        </div>
      </div>
    </div>
  );
}

export default ModalUserDefined;