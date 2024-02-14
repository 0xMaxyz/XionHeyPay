import {
  useAbstraxionAccount,
  useAbstraxionSigningClient,
} from "@burnt-labs/abstraxion";
import { useEffect, useState } from 'react'
import type { ExecuteResult} from "@cosmjs/cosmwasm-stargate";
import {HaypayAddress} from "../Const"
type ExecuteResultOrUndefined = ExecuteResult | undefined;
import "@burnt-labs/abstraxion/dist/index.css";
import "@burnt-labs/ui/dist/index.css";
import {ValidCoins} from "../Const";
import { CircularProgress } from "@mui/material";
import useNotification from "../Components/SnackBar";

const Send = () => {
  const sendNotification = useNotification();
  const { data: account } = useAbstraxionAccount();
  const { client } = useAbstraxionSigningClient();
  const [amount, setAmount] = useState(0);
  const [reciever, setReciever] = useState<string|undefined>();
  const [balance, setBalance] = useState<number|undefined>(undefined);
  // const [contractBalance, setContractBalance] = useState(0);
  const [transactionMessage, setTransactionMessage] = useState("dinner");
  const [loading, setLoading] = useState(false);
  const [balanceLoding, setBalanceLoading] = useState(false);
  const [selectedtoken, setSelectedToken] = useState<string>(ValidCoins[0].token_address);
  const [executeResult, setExecuteResult] =
    useState<ExecuteResultOrUndefined>(undefined);
  async function ReadBalance() {
    setBalanceLoading(true);
    const readBalanceMsg = {
      balance: {
        address: account.bech32Address
      }
    };
    try {
      const SendRes = await client?.queryContractSmart(
        selectedtoken,
        readBalanceMsg,
      );
      console.log(SendRes);
      setBalance(SendRes!.balance);
    } catch (error) {
      console.log(error);
    }finally{
      setBalanceLoading(false);

    }
  }
  async function Pay() {
    event?.preventDefault();
    setLoading(true);
    const msg = {
      send :{
        contract:HaypayAddress,
        amount: amount.toString(),
        msg:btoa(`{"email":"${reciever!}","memo":"${transactionMessage}"}`)
      }
    };
    try {
      const SendRes = await client?.execute(
        account.bech32Address,
        selectedtoken,
        msg,
        {
          amount: [{ amount: "0", denom: "uxion" }],
          gas: "500000",
        },
        "",
        []
      );
      console.log(SendRes)
      setExecuteResult(SendRes);
      console.log(executeResult);
      sendNotification({msg:"Hey!! Token Sent successfully",variant:"success"});
      
    } catch (error) {
      // eslint-disable-next-line no-console -- No UI exists yet to display errors
      console.log(error);
      sendNotification({msg:`Error sending token${error}`,variant:"error"});
    } finally {
      setLoading(false);
    }
  }
  // ReadBalance();
  useEffect(()=>{
    if(account.bech32Address && selectedtoken && client)
      ReadBalance();
  },[account.bech32Address,selectedtoken,client]);
  return (
    <div className="flex flex-col bg-[#D0F6FF] w-full pt-20 items-center ">
      <div className="min-w-[40rem]  bg-[#81D6E3] rounded-xl max-w-80 border-[0.1rem] shadow-[0.1rem_0.1rem_0_0_rgba(0,0,0,1)] border-black">
        <div className="flex h-16 bg-[#ADE8F2] rounded-t-xl border-b-[0.11rem] border-black pl-5 items-center">
          <a className="font-bold text-lg">Send Money</a>
        </div>
        <form onSubmit={Pay}>
          <div className="flex flex-col p-7 gap-4">
              <div className="flex flex-row gap-5 items-center">
                <a className="w-20 font-bold">Token</a>
                <div>   
                  <select id="tokens" value={selectedtoken} onChange={e=>{setSelectedToken(e.target.value)}} className="bg-[#81D6E3] border-[0.2rem] rounded-lg h-12 w-32 border-[#29A8BB] " name="tokens">
                    {ValidCoins.map((token,index)=>(<option key={index} value={token.token_address}>
                      <div className="flex flex-row">
                        <img src={token.logo} className="w-4 h-4"></img>
                        <a>{token.symbol}</a>
                      </div>
                      
                      </option>))}
                  </select>           
                  <div className="">
                    <a className=" font-light">Balance: </a>
                    {(balanceLoding || balance==undefined)?<CircularProgress size={12} />:
                      <a className="px-1">{balance}</a>
                    }
                  </div>

                </div>        
              </div>
            <div className="flex flex-row gap-5 items-center">
              <a className="w-20 font-bold">Amount</a>
              <input className="bg-[#81D6E3] px-2 border-[0.2rem] rounded-lg h-10 w-32 border-[#29A8BB] "
                        type="number"
                        id='amount'
                        name= 'amount'
                        content={amount.toString()}
                        onChange={e=> setAmount(Number(e.target.value))}
                        placeholder="amount"/>
            </div>
            <div className="flex flex-row gap-5 items-center">
              <a className="w-20 font-bold">To</a>
              <input className="bg-[#81D6E3] border-[0.2rem] rounded-lg h-12 w-64 px-2 fon border-[#29A8BB] "
                        type="email"
                        id='reciever'
                        name= 'reciever'
                        content={reciever}
                        onChange={e=> setReciever(e.target.value)}
                        placeholder="eg. burntbanksy@gmail.com"
              />
            </div>
            <div className="flex flex-row gap-5 items-center">
              <a className="w-20 font-bold">Description</a>
              <input className="bg-[#81D6E3] focus:bg-black px-2 border-[0.2rem] rounded-lg h-12 w-64 border-[#29A8BB] "
                        type="string"
                        id='details'
                        name= 'details'
                        content={transactionMessage}
                        onChange={e=> setTransactionMessage(" "+e.target.value)}
                        placeholder="Description  (Optional)"
              />
            </div>
            <div className="flex flex-row-reverse w-full ">
            {!loading?<button disabled={!reciever|| !amount || !selectedtoken} className="w-[10rem] h-[3rem] bg-sky-600 hover:bg-sky-500 disabled:bg-gray-500 disabled:text-slate-700  border-gray-500 text-white  rounded text-xl font-bold" >Send</button>:<CircularProgress></CircularProgress>}
              {/* <button className="w-[10rem] h-[3rem] bg-sky-600 hover:bg-sky-500 disabled:bg-gray-500 disabled:text-slate-700  border-gray-500 text-white  rounded text-xl">Transfer</button> */}
            </div>
          </div>
        </form>
      </div>
    </div>
  )
}

export default Send