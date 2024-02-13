import { PropsWithChildren } from 'react'
import { ClaimRow } from '../interfaces/types';
import AddressViewer from './AddressViewer';

type Props = PropsWithChildren<{claimObject:ClaimRow}>;
const ClaimCard = (props:Props) => {
  return (
    <div className='flex flex-col'>
        <div className='flex flex-row'>
            <a>from: </a>
            <AddressViewer address={props.claimObject.sender}></AddressViewer>
        </div>
        <div className='flex flex-row'>
            <img src={props.claimObject.logo} className='w-8 h-8'></img>
            <a>{props.claimObject.symbol}</a>
        </div>
        <div className='flex flex-row'>
            <a>{props.claimObject.amount*props.claimObject.price}$</a>
            <a>{props.claimObject.amount}</a>
        </div>
    </div>
  )
}

export default ClaimCard