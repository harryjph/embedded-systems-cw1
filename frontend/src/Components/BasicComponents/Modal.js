function Modal(props) {
  function YesSelect() {
    props.onConfirm();
  }
  function NoSelect() {
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
