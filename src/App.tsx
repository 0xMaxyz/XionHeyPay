import React from 'react';
import Main from "./main";
import {SnackbarProvider} from 'notistack';
import ReactDOM from "react-dom/client";
import "./index.css";
import { AbstraxionProvider } from "@burnt-labs/abstraxion";
import { HPCAddress,HaypayAddress } from "./Const.tsx";
import { UserContextProvider } from "./jwtContext/index.tsx";
import CssBaseline from '@mui/material/CssBaseline';


ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <AbstraxionProvider
        config={{
            contracts: [HPCAddress,HaypayAddress],
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



