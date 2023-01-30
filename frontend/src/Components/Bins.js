import CircularProgressBar from './CircularProgressBar.js';
import Modal from './Modal.js';
import ModalWithRenameSupport from './ModalWithRenameSupport.js';
import Backdrop from './Backdrop.js';

import classes from './Bins.module.css';

import {useState} from 'react';

function Bins(props) {
  const [binValue, setBinValue] = useState(false);
  const [renameBinValue, setRenameBinValue] = useState(false);

    function addHandler() {
      setBinValue(true);
      console.log('add ' + props.ID);
    }
    
    function changeNameHandler() {
      setRenameBinValue(true);
      console.log('change name ' + props.ID);
    }

    function closeHandler() {
      {/* TODO: add code here that sends the username renaming in*/}
      setBinValue(false);
      setRenameBinValue(false);
      console.log('remove ' + props.ID);
    }

    return(
        <div className='card'>
          <h2 className={'classes.content'}> ID {props.ID}</h2>


          <div className={'classes.content'}>
            <CircularProgressBar upper_value={props.Fullness}/>
          </div>

          <div className='classes.actions'>
            <button className = 'btn' onClick={addHandler}>Add to Favorites</button>
            <button className = 'btn' onClick={changeNameHandler}>Change Name</button>
          </div>
          
          {binValue && <Modal ID = {props.ID} onCancel={closeHandler} onConfirm={closeHandler}/>}
          {(binValue) && <Backdrop onClick={closeHandler}/>}

          {renameBinValue && <ModalWithRenameSupport ID={props.ID} onCancel={closeHandler} onConfirm={closeHandler}/>}
          {(renameBinValue) && <Backdrop onClick={closeHandler}/>}
        </div>
    );
}


export default Bins;