function Modal(props) {
  function YesSelect() {
    console.log("Yes " + props.ID);
    props.onConfirm();
  }
  function NoSelect() {
    console.log("No " + props.ID);
    props.onCancel();
  }

  return (
    <div className="modal">
      <p>
        Are you sure?
        <button className="btn" onClick={YesSelect}>
          Yes
        </button>
        <button className="btn bttn--alt" onClick={NoSelect}>
          Cancel
        </button>
      </p>
    </div>
  );
}

export default Modal;
