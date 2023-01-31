import Card from './ui/Card'
import classes from './AddNewBinsForm.module.css'

import {useRef} from 'react';

function AddNewBinsForm() {
    const binIDInputRef = useRef();
    const binNameInputRef = useRef();
    const binLatInputRef = useRef();
    const binLongInputRef = useRef();
    const binFullnessInputRef = useRef();

    function submitHandler(event) {
        event.preventDefault();

        const enteredID = binIDInputRef.current.value;
        const enteredName = binNameInputRef.current.value;
        const enteredLat = binLatInputRef.current.value;
        const enteredLong = binLongInputRef.current.value;
        const enteredFullness = binFullnessInputRef.current.value;

        const meetupData = {
            id: enteredID,
            name: enteredName,
            latitude: enteredLat,
            longitude: enteredLong,
            fullness: enteredFullness,
        };

        console.log(meetupData);
    }

    return <Card>
        <form className={classes.form} onSubmit={submitHandler}>
            <div className={classes.control}>

                <div className={classes.control}>
                    <label htmlFor='title'>Bin ID</label>
                    <input type="text" required id="id" ref={binIDInputRef}/>
                </div>

                <div className={classes.control}>
                    <label htmlFor='title'>Bin Username</label>
                    <input type="text" required id="name" ref={binNameInputRef}/>
                </div>

                <div className={classes.control}>
                    <label htmlFor='title'>Bin Latitude</label>
                    <input type="text" required id="latitude" ref={binLatInputRef}/>
                </div>

                <div className={classes.control}>
                    <label htmlFor='title'>Bin Longitude</label>
                    <input type="text" required id="longitude" ref={binLongInputRef}/>
                </div>

                <div className={classes.control}>
                    <label htmlFor='title'>Bin Fullness</label>
                    <input type="text" required id="fullness" ref={binFullnessInputRef}/>
                </div>

                <div className={classes.control}>
                <button className = 'btn'>Add Bin</button>
                </div>

            </div>
        </form>
    </Card>
}

export default AddNewBinsForm