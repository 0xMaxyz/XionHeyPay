import React from 'react'

const NothingMessage = () => {
  return (
    <div className='flex flex-col items-center gap-5 p-5'>
        <img src="/HeyPay/sadFace.svg" className='h-32 w-32'></img>
        <a className='font-extrabold'>Nothing to Claim </a>
    </div>
  )
}

export default NothingMessage