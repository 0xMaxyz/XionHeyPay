import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import { AbstraxionProvider } from "@burnt-labs/abstraxion";
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { HPCAddress,HaypayAddress } from "./Const.tsx";
import NavBar from "./NavBar.tsx";
import Send from "./send/page.tsx";
import Wallet from "./wallet/page.tsx";
import MainPage from "./page.tsx";
import { UserContextProvider } from "./jwtContext/index.tsx";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <AbstraxionProvider
      config={{
        contracts: [HPCAddress,HaypayAddress],
        dashboardUrl:"http://localhost:3000"
      }}
    >
      <UserContextProvider>
        <BrowserRouter>
          <NavBar/>
          <Routes>
              <Route path = "/front_test" element={<MainPage/>}/>
              <Route path = "/front_test/send" element={<Send />} />
              <Route path = "/front_test/wallet" element={<Wallet />} />
          </Routes>
        </BrowserRouter>
      </UserContextProvider>
    </AbstraxionProvider>
  </React.StrictMode>
);
