import { PropsWithChildren } from 'react'
import { ClaimRow } from '../interfaces/types';
import AddressViewer from './AddressViewer';

type Props = PropsWithChildren<{claimObject:ClaimRow}>;
const ClaimCard = (props:Props) => {
  return (
    <div className=' p-1 flex flex-col min-w-40 max-w-80 bg-slate-200 gap-1 rounded-lg border-slate-300 border-2'>
        <div className='flex flex-row gap-2 '>
            <a>Sender: </a>
            <AddressViewer address={props.claimObject.sender}></AddressViewer>
        </div>
        <div className='flex p-2 flex-row full-width bg-slate-50 rounded-4 items-center gap-2'>
          <img src={props.claimObject.logo} className='w-12 h-12 align-middle'></img>
          <div className='flex flex-col'>
              <a>{props.claimObject.symbol}</a>
              <a className='text-slate-500'>{props.claimObject.amount}</a>
          </div>
          <div className='flex flex-row-reverse w-full'>
              <a>{props.claimObject.amount*props.claimObject.price}$</a>
          </div>
        </div>
    </div>
  )
}

export default ClaimCard