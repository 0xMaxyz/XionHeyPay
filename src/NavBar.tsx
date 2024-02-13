import {
  Abstraxion,
  useAbstraxionAccount,
  useModal
} from "@burnt-labs/abstraxion";
import { Button } from "@burnt-labs/ui";
import "./index.css";
import { useNavigate } from "react-router-dom";
import AddressViewer from "./Components/AddressViewer";
const NavBar = () => {
  const navigate = useNavigate();
  const { data: account } = useAbstraxionAccount();
  const [,setShowAbstraxion] = useModal();

  const links = [
    {href:"/front_test/send",label:"Send",img:""},
    {href:"/front_test/wallet",label:"Wallet",img:""},
  ]
  const handleCloseNavMenu = (key: string) => {
    navigate(key)
  };
  return (
    <nav className='flex space-x-6 mb-5 px-5 h-14 items-center bg-cyan-700' >
        <div className='flex flex-row items-center space-x-2'>
          <img src={""} className='w-9 h-9'/>
          <a href={"/"} className="text-black text-lg"> HayPay</a>
        </div>
        <ul className='flex space-x-6'>
            {links.map(link=>
            <div className='flex flex-row space-x-1 items-center' key={link.label} >
              <button onClick={()=>handleCloseNavMenu(link.href)}>
                <img src={link.img}/>
                <a className="text-black text-sm transition-colors" >{link.label}</a>
              </button>
            </div> 
            )}
        </ul>
        <div className=' w-full flex '></div>
        <div className='flex flex-row-reverse p-6'>
          <Button 
          fullWidth className='color-w'
          onClick={() => {
            setShowAbstraxion(true);
          }}
          structure="base"
          >
            {account.bech32Address ? <AddressViewer address={account.bech32Address}/> : (
              "CONNECT"
            )}
          </Button>
        </div>
      <Abstraxion
        onClose={() => { 
          setShowAbstraxion(false);
        }}
      />
    </nav>
  )
}

export default NavBar