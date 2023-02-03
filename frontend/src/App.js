import './App.css';

//  Page Routing Support
import {BrowserRouter, Routes, Route, Link } from 'react-router-dom';

//  Import Components
import Bins from './Components/Bins/Bins.js';

//  Import Navigation
import Layout from './Components/Layout/Layout.js';

//  Import Pages
import AllOfMyBinsPage from './pages/AllOfMyBins.js';
import UnclaimedBinsPage from './pages/UnownedBins.js';

import { Switch } from 'react-native-web';

function App() {
  return (
  <div>
    <BrowserRouter>
      <Layout />
      <Routes>
        {/* All avaliable bins (TODO: Home-page) */}
          <Route path='/' element={<AllOfMyBinsPage/>} />
        {/* My Bins */}
          <Route path='/my-bins' element={<UnclaimedBinsPage/>} />
      </Routes>
    </BrowserRouter>
  </div>
  );
}

export default App;
