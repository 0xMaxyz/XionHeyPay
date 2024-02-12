import { PropsWithChildren } from 'react'
import { BalanceRow } from '../interfaces/types';

type Props = PropsWithChildren<BalanceRow>;
const balanceRow = (props:Props) => {
  return (
    <div className='flex flex-col'>
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

export default balanceRow