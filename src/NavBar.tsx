import {
  Abstraxion,
  useAbstraxionAccount,
  useModal
} from "@burnt-labs/abstraxion";
import { Button } from "@burnt-labs/ui";
import "./index.css";
// import { useNavigate } from "react-router-dom";
const NavBar = () => {
  // const navigate = useNavigate();
  const { data: account } = useAbstraxionAccount();
  const [,setShowAbstraxion] = useModal();

  // const links = [
  //   // {href:"/HeyPay/send",label:"Send",img:""},
  //   // {href:"/HeyPay/wallet",label:"Wallet",img:""},
  // ]
  // const handleCloseNavMenu = (key: string) => {
  //   navigate(key)
  // };
  return (
    <nav className='flex space-x-6 px-8 h-16 items-center bg-gradient-to-l from-[#65CADA] to-[#97DFEB]/60 shadow backdrop-blur-sm' >
        <div className='flex flex-row items-center space-x-2'>
          <img src={"/HeyPay/logo.svg"} className='w-9 h-9'/>
          <a href={"/"} className="text-black text-2xl font-bold"> HeyPay</a>
        </div>
        <ul className='flex space-x-6'>
            {/* {links.map(link=>
            <div className='flex flex-row space-x-1 items-center' key={link.label} >
              <button onClick={()=>handleCloseNavMenu(link.href)}>
                <img src={link.img}/>
                <a className="text-black text-sm transition-colors" >{link.label}</a>
              </button>
            </div> 
            )} */}
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
            <div className=" pl-5 pr-5 pt-1 pb-1 hover:bg-[#ADE8F3] justify-center align-middle border-2 rounded-lg border-[#2D3D50] text-[#2D3D50]">
              {account.bech32Address ? ("Logout") : (
                "CONNECT"
              )}
            </div>
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