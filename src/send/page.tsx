import {
  Abstraxion,
  useAbstraxionAccount,
  useAbstraxionSigningClient,
} from "@burnt-labs/abstraxion";
import React, { useEffect, useState } from 'react'
import type { ExecuteResult} from "@cosmjs/cosmwasm-stargate";
import {seatContractAddress} from "../Const"
type ExecuteResultOrUndefined = ExecuteResult | undefined;

const Send = () => {
  const { data: account } = useAbstraxionAccount();
  const { client } = useAbstraxionSigningClient();
  const [amount, setAmount] = useState(0);
  const [reciever, setReciever] = useState<string|undefined>();
  const [balance, setBalance] = useState(0);
  const [loading, setLoading] = useState(false);
  const [executeResult, setExecuteResult] =
    useState<ExecuteResultOrUndefined>(undefined);
  async function ReadBalance() {
    console.log("Read Balance")
    const readBalanceMsg = {
      balance: {
        address: account.bech32Address
      }
    };
    try {
      const SendRes = await client?.queryContractSmart(
        seatContractAddress,
        readBalanceMsg,
      );
      console.log(SendRes);
      setBalance(SendRes.balance);
    } catch (error) {
      // eslint-disable-next-line no-console -- No UI exists yet to display errors
      console.log(error);
    }
  }
  async function SendToken() {
    event?.preventDefault();
    console.log("Send Token:")
    console.log("address: ", reciever)
    console.log("amount: ", amount);
    setLoading(true);
    const msg = {
      transfer :{
        recipient: reciever!,
        amount: amount.toString() 
      }
    };
    try {
      const SendRes = await client?.execute(
        account.bech32Address,
        seatContractAddress,
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
    } catch (error) {
      // eslint-disable-next-line no-console -- No UI exists yet to display errors
      console.log(error);
    } finally {
      setLoading(false);
    }
  }
  useEffect(()=>{
    ReadBalance();
  },[account]);
  return (
    <div>
      <div className='inline-flex h-20 w-full pt-3 pb-3 '>
        <a>Balance</a>
        <a>{balance}</a>
      </div>
      <form onSubmit={SendToken} className='inline-flex h-20 w-full pt-3 pb-3'>
        <input
          type="string"
          id='reciever'
          name= 'reciever'
          content={reciever}
          onChange={e=> setReciever(e.target.value)}
          className="w-full p-2  border border-gray-500 rounded  focus:outline-none"
          placeholder="Reciever Address"
        />
        <input
          type="number"
          id='amoutn'
          name= 'amount'
          content={amount.toString()}
          onChange={e=> setAmount(Number(e.target.value))}
          className="w-full p-2  border-l border-t border-b border-gray-500 rounded-tl-2xl rounded-bl-2xl  focus:outline-none"
          placeholder="amount"
        />
        <button disabled={loading} className="w-[150px] bg-sky-500 border-r border-t border-b border-gray-500 text-white pr-1  pl-1 rounded-tr-2xl rounded-br-2xl h-full text-xl" >Send</button>
      </form>
    </div>

  )
}

export default Send