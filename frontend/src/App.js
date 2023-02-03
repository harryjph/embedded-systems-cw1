import "./App.css";

//  Page Routing Support
import { BrowserRouter, Routes, Route } from "react-router-dom";

//  Import Navigation

//  Import Pages
import LoginPage from "./pages/LoginPage.js";
import AllOfMyBinsPage from "./pages/AllOfMyBins.js";
import UnclaimedBinsPage from "./pages/UnownedBins.js";

function App() {
  return (
    <div>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<LoginPage />} />
          {/* All avaliable bins (TODO: Home-page) */}
          <Route path="/my-bins" element={<AllOfMyBinsPage />} />
          {/* My Bins */}
          <Route path="/unowned-bins" element={<UnclaimedBinsPage />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;
