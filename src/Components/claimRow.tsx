import { PropsWithChildren } from 'react'
import { ClaimRow } from '../interfaces/types';

type Props = PropsWithChildren<ClaimRow>;
const claimRow = (props:Props) => {
  return (
    <div className='flex flex-col'>
        <div className='flex flex-row'>
            <a>from: </a>
            <a>{props.sender}</a>
        </div>
        <div className='flex flex-row'>
            <img src={props.logo} className='w-8 h-8'></img>
            <a>{props.symbol}</a>
        </div>
        <div className='flex flex-row'>
            <a>{props.amount*props.price}$</a>
            <a>{props.amount}</a>
        </div>
    </div>
  )
}

export default claimRow