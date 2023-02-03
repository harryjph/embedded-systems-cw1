import {useNavigate} from 'react-router-dom';
import AddNewBinsForm from "../Components/AddNewBinsForm";

function AddBinsFormPage() {
    const history = useNavigate();

    function addMeetupHandler(binsData) {
        fetch(
            'https://embedded-systems-test-server-default-rtdb.europe-west1.firebasedatabase.app//bins.json',
            {
                method: 'POST',
                body: JSON.stringify(binsData),
                headers: {
                'Content-Type': 'application/json'
                }
            }).then(()=>{console.log("posted form"); history('/')});
    }

    return <section>
        <h1>Add Bins Internal Form Page</h1>
        <AddNewBinsForm onAddBins={addMeetupHandler}/>
    </section>;
}

export default AddBinsFormPage;