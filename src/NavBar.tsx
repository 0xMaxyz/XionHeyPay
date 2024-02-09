import Link from 'next/link'
import React, { useState } from 'react'
import {
  Abstraxion,
  useAbstraxionAccount,
  useAbstraxionSigningClient,
} from "@burnt-labs/abstraxion";
import { Button } from "@burnt-labs/ui";
import "./index.css";
const NavBar = () => {
  const { data: account } = useAbstraxionAccount();
  const { client } = useAbstraxionSigningClient();
  const [loading, setLoading] = useState(false);

  const [isOpen, setIsOpen] = useState(false);
  const links = [
    {href:"send",label:"Send",img:""},
    {href:"wallet",label:"Wallet",img:""},
  ]
  return (
    <nav className='flex space-x-6 mb-5 px-5 h-14 items-center bg-slate-950' >
        <div className='flex flex-row items-center space-x-2'>
          <img src={""} className='w-9 h-9'/>
          <a href={"/"} className="text-white text-lg"> HayPay</a>
        </div>
        <ul className='flex space-x-6'>
            {links.map(link=>
            <div className='flex flex-row space-x-1 items-center' key={link.label} >
              <button >
                <img src={link.img}/>
                <a href={"/"+link.href}className="text-white text-sm transition-colors" >{link.label}</a>
              </button>
            </div> 
            )}
        </ul>
        <div className=' w-full flex '></div>
        <div className='flex flex-row-reverse p-6'>
          <Button 
          fullWidth
          onClick={() => {
            setIsOpen(true);
          }}
          structure="base"
          >
            {account.bech32Address ? account.bech32Address : (
              "CONNECT"
            )}
          </Button>
        </div>

      {/* {client ? (
        <Button
          disabled={loading}
          fullWidth
          onClick={() => {
            // void claimSeat();
          }}
          structure="base"
        >
          {loading ? "LOADING..." : "CLAIM SEAT"}
        </Button>
      ) : null} */}
      <Abstraxion
        isOpen={isOpen}
        onClose={() => {
          setIsOpen(false);
        }}
      />
    </nav>
  )
}

export default NavBar