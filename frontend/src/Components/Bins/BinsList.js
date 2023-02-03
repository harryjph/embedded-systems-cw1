import Bins from './Bins.js';
import classes from './BinsList.module.css';

import {useEffect, useState} from "react";

function BinsList(props) {
    
    const [bins, setBins] = useState([]);
    console.log(props.AllData);
    
    let binsWidgets = props.AllData.map(bin => 
                                            (<Bins 
                                                key={bin.id} 
                                                ID={bin.id} 
                                                Name={bin.config.name} 
                                                Latitude={bin.config.latitude} 
                                                Longitude={bin.config.longitude} 
                                                Threshold={bin.config.full_threshold}
                                                Fullness={Math.floor(bin.fullness * 100)}/>
                                            )
                                        );
    
    return (
        <div className='classes.list'>
            {binsWidgets}
        </div>
    );
}

export default BinsList;