import BinsList from '../Components/BinsList.js';
import {useEffect, useState} from "react";

function AllBinsPage() {
    const [isLoading, setIsloading] = useState(true);
    const [loadedBins, setLoadedBins] = useState([]);

    useEffect(() => {
        setIsloading(true);
        fetch(
                'https://embedded-systems-test-server-default-rtdb.europe-west1.firebasedatabase.app//bins.json',
                ).then(response=> {
                    return response.json();
                }).then(data => {
                    const bins = [];
                    for(const key in data){
                        const bin = {
                            ...data[key]
                        };
                        bins.push(bin);
                    }
                    setIsloading(false);
                    setLoadedBins(bins);
                }
            );
    }, []);

    if(isLoading){
        return(
            <section>
                <p>Loading...</p>
            </section>
        );
    }

    return (
        <BinsList AllData={loadedBins}/>
    )
    
};

export default AllBinsPage;