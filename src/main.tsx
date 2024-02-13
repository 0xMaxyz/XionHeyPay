import { BrowserRouter, Routes, Route } from 'react-router-dom';
import NavBar from "./NavBar.tsx";
import Send from "./send/page.tsx";
import Wallet from "./wallet/page.tsx";
import MainPage from "./page.tsx";

function Main() {
  return (
    <BrowserRouter>
      <NavBar/>
      <Routes >
          <Route path = "/front_test" element={<MainPage/>}/>
          <Route path = "/front_test/send" element={<Send />} />
          <Route path = "/front_test/wallet" element={<Wallet />} />
      </Routes>
    </BrowserRouter>
  );
}

export default Main;