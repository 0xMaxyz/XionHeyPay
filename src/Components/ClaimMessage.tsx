import React from 'react'

const ClaimMessage = () => {
  return (
    <div className='flex flex-col'>
        <div className='flex flex-col bg-[#2D6CDF] border-yellow-200 border-2 rounded-2xl text-white p-5'>
            <a className='font-bold'> Hey!!!</a>
            <a className=''>you have Tokens to claim!</a>
        </div>
        <div className='flex justify-center '>
            <img src='/HeyPay/arrow.svg' className='w-16 h-16'></img>
        </div>
    </div>

  )
}

export default ClaimMessage