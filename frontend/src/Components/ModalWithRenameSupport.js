//  MODAL RENAME NEW

import Card from './ui/Card'
import classes from './AddNewBinsForm.module.css'

import {useRef} from 'react';

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
        console.log(props);
        console.log("sent "+props.ID+" data to server")
        props.onConfirm();
    }

    function onCancel(){
        console.log('Cancel update of ' + props.ID);
        props.closeHandler();
    }

    return(
        <div className="modal">

            <form className={classes.form} onSubmit={submitHandler}>
                <div className={classes.control}>

                    <div className={classes.control}>
                        <label htmlFor='title'>Bin Name</label>
                        <input type="text" required id="name" placeholder={props.Name} ref={binNameInputRef}/>
                    </div>

                    <div className={classes.control}>
                        <label htmlFor='title'>Bin Latitude</label>
                        <input type="text" required id="latitude" placeholder={props.Latitude} ref={binLatInputRef}/>
                    </div>

                    <div className={classes.control}>
                        <label htmlFor='title'>Bin Longitude</label>
                        <input type="text" required id="longitude" placeholder={props.Longitude} ref={binLongInputRef}/>
                    </div>

                    <div className={classes.control}>
                        <label htmlFor='title'>Bin Fullness</label>
                        <input type="text" required id="fullness" placeholder={Math.floor(props.Fullness)} ref={binFullnessInputRef}/>
                    </div>

                    <div className={classes.control}>
                        <label htmlFor='title'>Bin Fullness Threshold</label>
                        <input type="text" required id="threshold" value={props.Threshold} ref={binFullnessThresholdRef}/>
                    </div>

                    <div className={classes.control}>
                    <button className = 'btn' onClick={submitHandler}>Save Changes</button>
                    </div>

                    <div className={classes.control}>
                    <button className = 'btn' onClick={onCancel}>Cancel</button>
                    </div>

                </div>
            </form>

        </div>
    )
}

export default ModalWithRenameSupport;