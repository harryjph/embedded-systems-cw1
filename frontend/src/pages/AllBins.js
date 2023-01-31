import BinsList from '../Components/BinsList.js';
import {useEffect, useState} from "react";
import axios from 'axios';

const DUMMY_DATA = [
    {
        id: '1',
        name: 'Gilbert',
        latitude: '51.501',
        longitude: '-0.142',
        fullness: 50
    },
    {
        id: '2',
        name: 'Godfried',
        latitude: '51.501',
        longitude: '-0.145',
        fullness: 65
    },
    {
        id: '3',
        name: 'Stephen',
        latitude: '51.498',
        longitude: '-0.177',
        fullness: 95
    },
    {
        id: '4',
        name: 'Fry',
        latitude: '51.470',
        longitude: '-0.454',
        fullness: 85
    },
    {
        id: '5',
        name: 'Gilbert',
        latitude: '51.162',
        longitude: '-0.177',
        fullness: 99
    },
    //    <div style={{ width: 100, marginLeft: 100}}>
    //        <CircularProgressBar upper_value={55}/>
    //    </div>
    
];

function AllBinsPage() {

    return (
    <BinsList AllData={DUMMY_DATA}/>
    )
    
};

export default AllBinsPage;