import React, { Children } from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import { AbstraxionProvider } from "@burnt-labs/abstraxion";
import { BrowserRouter, Routes, Route, Link } from 'react-router-dom';
import { seatContractAddress } from "./Const.tsx";
import NavBar from "./NavBar.tsx";
import Send from "./send/page.tsx";
import Wallet from "./wallet/page.tsx";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <AbstraxionProvider
      config={{
        contracts: [seatContractAddress],
      }}
    >
    <BrowserRouter>
      <NavBar/>
      <Routes>
          <Route path = "/send" element={<Send />} />
          <Route path = "/wallet" element={<Wallet />} />
      </Routes>
      </BrowserRouter>
    </AbstraxionProvider>
  </React.StrictMode>
);
