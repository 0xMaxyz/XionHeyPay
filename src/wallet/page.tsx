
import {
  useAbstraxionAccount,
  useAbstraxionSigningClient,
} from "@burnt-labs/abstraxion";
import { useEffect, useState } from 'react'
import {HaypayAddress} from "../Const"
import "@burnt-labs/abstraxion/dist/index.css";
import "@burnt-labs/ui/dist/index.css";
import {useUserContext} from "../jwtContext"
import { ClaimRow } from "../interfaces/types";
import ClaimCard from "../Components/ClaimCard";
import CircularProgress from '@mui/material/CircularProgress';
import Button from "@mui/material/Button";
interface ClaimResults{
  token: string,
  amount: string,
  sender: string,
  memo: string
}
const Wallet = () => {
  const {email,jwt} = useUserContext();
  const {data: account } = useAbstraxionAccount();
  const {client } = useAbstraxionSigningClient();
  const [claimables, setClaimables] = useState<ClaimRow[]|undefined>(undefined);
  const [loading, setLoading] = useState(false);
  async function ReadClaimables() {
    console.log("Read Claimable of Email:", email)
    const claimsMsg = {
      claims: {
        email: email!
      }
    };
    try {
      const SendRes = await client?.queryContractSmart(
        HaypayAddress,
        claimsMsg,
      );
      const claims = SendRes.claims as unknown as ClaimResults[];
      setClaimables(claims.map(x=> {
        return {
          sender: x.sender,
          metadata: x.memo,
          symbol:"USDT",
          token_address: x.token,
          amount: Number(x.amount),
          decimals: 18,
          logo:"/HeyPay/USDTlogo.png",
          price: 1.1
        } as ClaimRow
      }))
    } catch (error) {
      // eslint-disable-next-line no-console -- No UI exists yet to display errors
      console.log(error);
    }
  }
  async function ClaimTokens() {
    event?.preventDefault();
    console.log("Claim Tokens")
    console.log("email ", email)
    setLoading(true);
    const msg = {
      claim :{
        msg:{
          jwt: jwt!,
          aud: "project-live-7e4a3221-79cd-4f34-ac1d-fedac4bde13e"
        }
      }
    };
    try {
      const SendRes = await client?.execute(
        account.bech32Address,
        HaypayAddress,
        msg,
        {
          amount: [{ amount: "0", denom: "uxion" }],
          gas: "500000",
        },
        "",
        []
      );
      console.log(SendRes)
      // setExecuteResult(SendRes);
      console.log(SendRes);
      
    } catch (error) {
      // eslint-disable-next-line no-console -- No UI exists yet to display errors
      console.log(error);
    } finally {
      setLoading(false);
    }
  }

  useEffect(()=>{
    if(email && account!.bech32Address)
      ReadClaimables();
  },[account,email]);
  return (
  <div>
    <div className='flex flex-col w-full pt-3 pb-3 '>
      <a>Claimables</a>
      {claimables?.map((x,index) =>(<ClaimCard key={index} claimObject={x}></ClaimCard>))}
    </div>
    <form onSubmit={ClaimTokens} className='inline-flex h-20 w-full pt-3 pb-3'>
      {!loading?<button disabled={loading|| !claimables || claimables.length<1} className="w-[150px] bg-sky-600 hover:bg-sky-500 disabled:bg-gray-500 disabled:text-slate-700  border-gray-500 text-white  rounded h-full text-xl" >Claim</button>:<CircularProgress></CircularProgress>}
    </form>
  </div>

  )
}

export default Wallet