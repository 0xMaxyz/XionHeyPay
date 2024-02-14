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
          <Route path = "/HeyPay" element={<MainPage/>}/>
          <Route path = "/HeyPay/send" element={<Send />} />
          <Route path = "/HeyPay/wallet" element={<Wallet />} />
      </Routes>
    </BrowserRouter>
  );
}

export default Main;