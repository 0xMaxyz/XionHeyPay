import { PropsWithChildren } from 'react'
import { ClaimRow } from '../interfaces/types';
import AddressViewer from './AddressViewer';

type Props = PropsWithChildren<{claimObject:ClaimRow}>;
const ClaimCard = (props:Props) => {
  return (
    <div className=' flex flex-col min-w-40  bg-[#DDF7FC] gap-1 rounded-xl border-[0.1rem] shadow-[0.1rem_0.1rem_0_0_rgba(0,0,0,1)] border-black'>
        <div className='flex flex-row gap-2 p-2'>
            <a>Sender: </a>
            <AddressViewer address={props.claimObject.sender}></AddressViewer>
        </div>
        <div className="flex p-2 flex-col bg-[#C5F3FB] rounded-b-xl">


          <div className='flex p-2 flex-row full-width rounded-xl rounded-b-xl items-center gap-2'>
            <img src={props.claimObject.logo} className='w-12 h-12 align-middle'></img>
            <div className='flex flex-col'>
                <a>{props.claimObject.symbol}</a>
                <a className='text-slate-500'>{props.claimObject.amount}</a>
            </div>
            <div className='flex flex-row-reverse w-full font-bold'>
                <a>{props.claimObject.amount*props.claimObject.price}$</a>
            </div>
          </div>
          <a className='text-[#5D6D7F] bg-[#C5F3FB] px-4'> {props.claimObject.metadata}</a>
        </div>
    </div>
  )
}

export default ClaimCard