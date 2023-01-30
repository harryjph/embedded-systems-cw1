import Bins from 'C:/Users/Orkun/functional_circles/src/Components/Bins.js';
import classes from './BinsList.module.css';

import {useEffect, useState} from "react";

function BinsList(props) {

    const [bins, setBins] = useState([]);

    let binsWidgets = props.AllData.map(bin => (<Bins key={bin.id} ID={bin.id} Fullness={bin.fullness}/>));
    
    return (
        <div className='classes.list'>
            {binsWidgets}
        </div>
    );
}

export default BinsList;