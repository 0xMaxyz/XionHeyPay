import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import { AbstraxionProvider } from "@burnt-labs/abstraxion";
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { seatContractAddress } from "./Const.tsx";
import NavBar from "./NavBar.tsx";
import Send from "./send/page.tsx";
import Wallet from "./wallet/page.tsx";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <AbstraxionProvider
      config={{
        contracts: [seatContractAddress],
        dashboardUrl:"http://localhost:3000/"
      }}
    >
    <BrowserRouter>
      <NavBar/>
      <Routes>
          <Route path = "/front_test/send" element={<Send />} />
          <Route path = "/front_test/wallet" element={<Wallet />} />
      </Routes>
      </BrowserRouter>
    </AbstraxionProvider>
  </React.StrictMode>
);
