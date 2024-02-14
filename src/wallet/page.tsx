
import "@burnt-labs/abstraxion/dist/index.css";
import "@burnt-labs/ui/dist/index.css";
import SideBar from "../SideBar";
import Send from "../send/Send"
interface ClaimResults{
  token: string,
  amount: string,
  sender: string,
  memo: string
}
const Wallet = () => {
  return (
  <div className="flex flex-row-reverse h-full w-span">
    <SideBar></SideBar>
    <div></div>
    <Send></Send>
  </div>


  )
}

export default Wallet