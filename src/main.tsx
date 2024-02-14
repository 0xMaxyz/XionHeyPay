import { BrowserRouter, Routes, Route } from 'react-router-dom';
import NavBar from "./NavBar.tsx";
import Wallet from "./wallet/page.tsx";

function Main() {
  return (
    <BrowserRouter>
      <NavBar/>
      <Routes >
          <Route path = "/HeyPay" element={<Wallet/>}/>
      </Routes>
    </BrowserRouter>
  );
}

export default Main;