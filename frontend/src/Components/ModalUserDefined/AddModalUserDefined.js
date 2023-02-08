import { useNavigate } from "react-router-dom";
import ModalUserDefined from "./ModalUserDefined";
import { apiPostForm } from "../../API";

function AddModalWithRenameSupport(props) {
  const history = useNavigate();

  function ReleaseFunction(variables) {
    apiPostForm("/bins/" + variables.ID + "/release").then(() => {
      history("/unowned-bins");
    });
  }

  return (
    <ModalUserDefined 
        isOpen={props.binValue} 
        ID={props.ID} 
        onCancel={props.cancelHandler} 
        onConfirm={ReleaseFunction} 
    />

  );
}

export default AddModalWithRenameSupport;
