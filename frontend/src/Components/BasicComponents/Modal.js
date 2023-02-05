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
        <div className="flex justify-center items-center">
          <button className="p-3 hover:bg-sky-700 rounded-sm hover:font-bold" onClick={YesSelect}>Yes</button>
          <button className="p-3 hover:bg-sky-700 rounded-sm hover:font-bold" onClick={NoSelect}>Cancel</button>
        </div>
      </p>
    </div>
  );
}

export default Modal;