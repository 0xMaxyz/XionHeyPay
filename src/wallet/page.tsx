
import {
  useAbstraxionAccount,
  useAbstraxionSigningClient,
} from "@burnt-labs/abstraxion";
import { useEffect, useState } from 'react'
import {HaypayAddress} from "../Const"
import "@burnt-labs/abstraxion/dist/index.css";
import "@burnt-labs/ui/dist/index.css";
import {useUserContext} from "../jwtContext"

const Wallet = () => {
  const {email,jwt} = useUserContext();
  const { data: account } = useAbstraxionAccount();
  const { client } = useAbstraxionSigningClient();
  const [jwtToken, setJwtToken] = useState("");
  const [claimables, setClaimables] = useState(0);
  const [loading, setLoading] = useState(false);
  async function ReadClaimables() {
    console.log("Read Claimable")
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
      console.log("All Claimables: ",SendRes);
      // setClaimable(SendRes.total_claims);
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
    ReadClaimables();
  },[account,email]);
  return (
  <div>
    <div className='inline-flex h-20 w-full pt-3 pb-3 '>
      <a>Claimables</a>
      <a>{claimables}</a>
    </div>
    <form onSubmit={ClaimTokens} className='inline-flex h-20 w-full pt-3 pb-3'>
      <button disabled={loading} className="w-[150px] bg-sky-500 border-r border-t border-b border-gray-500 text-white pr-1  pl-1 rounded-tr-2xl rounded-br-2xl h-full text-xl" >Claim</button>
    </form>
  </div>

  )
}

export default Wallet