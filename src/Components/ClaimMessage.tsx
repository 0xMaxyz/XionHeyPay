
const ClaimMessage = () => {
  return (
    <div className='flex flex-col'>
        <div className='flex flex-col bg-[#2D6CDF] border-yellow-200 border-[0.1rem] shadow-[0.2rem_0.2rem_0_0_rgba(243,241,105,1)] rounded-2xl text-white p-5'>
            <a className='font-bold'> Hey!!!</a>
            <a className=''>you have Tokens to claim!</a>
        </div>
        <div className='flex justify-center '>
            <img src='/HeyPay/arrow.svg' className='w-12 h-12'></img>
        </div>
    </div>

  )
}

export default ClaimMessage