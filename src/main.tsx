import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import { AbstraxionProvider } from "@burnt-labs/abstraxion";
import { seatContractAddress } from "./Const.tsx";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <AbstraxionProvider
      config={{
        contracts: [seatContractAddress],
      }}
    >
      <App />
    </AbstraxionProvider>
  </React.StrictMode>
);
