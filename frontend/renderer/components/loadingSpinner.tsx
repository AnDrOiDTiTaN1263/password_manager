import React from 'react';
export default function LoadingSpinner(){
    return (
        <div className='flex w-14 h-14  rounded-full justify-center items-center bg-slate-50 relative'>
            <div className='w-14 h-14 rounded-full animate-spin-slow absolute bg-gradient-to-b from-blue-600 from-[10%] to-[20%] to-white'></div>
            <div className='w-12 h-12 flex bg-inherit rounded-full z-10'></div>
        </div>
    )
}