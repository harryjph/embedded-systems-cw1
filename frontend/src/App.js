import './App.css';

//  Page Routing Support
import {BrowserRouter, Routes, Route, Link } from 'react-router-dom';

//  Import Components
import Bins from './Components/Bins.js';

//  Import Navigation
import Layout from './Components/Layout/Layout.js';

//  Import Pages
import AddBinsFormPage from './pages/AddBinsForm.js';
import AllBinsPage from './pages/AllBins.js';
import MyBinsPage from './pages/MyBins.js';

import { Switch } from 'react-native-web';

const data = {
  content: {
    body: [
      {
        _uid: "1",
        _val: "73",
      },
      {
        _uid: "2",
        _val: "25",
      },
      {
        _uid: "3",
        _val: "42",
      }
    ]
  }
};

function App() {
  return (

  <div>
    <BrowserRouter>
      <Layout />
      <Routes>
        {/* All avaliable bins (TODO: Home-page) */}
          <Route path='/' element={<AllBinsPage/>} />
        {/* My Bins */}
          <Route path='/my-bins' element={<MyBinsPage/>} />
      </Routes>
    </BrowserRouter>
  </div>
  );
}

export default App;
