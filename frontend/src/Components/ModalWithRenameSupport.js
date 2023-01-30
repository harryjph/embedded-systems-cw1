function ModalWithRenameSupport(props) {

    function YesSelect(params) {
        console.log('Rename Yes ' + props.ID);
        props.onConfirm();
    }
    function NoSelect() {
        console.log('Rename No ' + props.ID);
        props.onCancel();
    }

    return(
        <div className="modal">

                <label>
                    <input type="text" id="renameVal"/>
                </label>

                <p>
                    Are you sure?
                    <button className = 'btn' onClick={YesSelect}>Yes</button>
                    <button className = 'btn bttn--alt' onClick={NoSelect}>Cancel</button>
                </p>

        </div>
    )
}

export default ModalWithRenameSupport;