import React from 'react';
import Main from "./main";
import {SnackbarProvider} from 'notistack';
import ReactDOM from "react-dom/client";
import "./index.css";
import { AbstraxionProvider } from "@burnt-labs/abstraxion";
import { HaypayAddress } from "./Const.tsx";
import { UserContextProvider } from "./jwtContext/index.tsx";
import {ValidCoins} from "./Const.tsx"
import CssBaseline from '@mui/material/CssBaseline';
let verifiedContracts = ValidCoins.map(x=>x.token_address);
verifiedContracts.push(HaypayAddress);

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <AbstraxionProvider
        
        config={{
            contracts: verifiedContracts,
            dashboardUrl:"http://localhost:3000"
        }}
        >
            <UserContextProvider>
                <SnackbarProvider>
                    <CssBaseline />
                    <Main />
                </SnackbarProvider>
            </UserContextProvider>
        </AbstraxionProvider>
    </React.StrictMode>
);



