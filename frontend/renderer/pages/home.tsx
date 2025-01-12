import React, { useState } from 'react'
import Head from 'next/head'
import Link from 'next/link'
import Image from 'next/image'
import LoadingSpinner from '../components/loadingSpinner';

export default function HomePage() {
  const [passInput, setPass] = useState("");
  const [showPass, setShowPass] = useState(false);
  const [showLoadingSpinner, setShowLoadingSpinner] = useState(false);
  const handleSubmit = () =>{
    console.log('submitted')
    setShowLoadingSpinner(true);
    setTimeout(() => {
      setShowLoadingSpinner(false)
    }, 1000);
  }

  return (
    <div className='flex flex-col justify-start items-center gap-20 w-[100vw] h-[100vh] p-20 bg-gray-50'>
      <div id='header' className='flex text-4xl font-bold'>Password Manager</div>
      {!showLoadingSpinner &&<form className='flex flex-col justify-center items-center h-[20vh] gap-5'>
        <label className='flex text-2xl font-semibold'>Password</label>
        <input className='flex w-[40vw] border-solid border-2 h-8 rounded-2xl p-2' type={showPass?'text':'password'} onChange={(e)=>setPass(e.target.value)}/>
        <div className='flex flex-row justify-center items-center gap-2'>
          {/* radio button */}
          <div className='flex justify-center items-center w-4 h-4 border-solid border-2 border-black   rounded-full' onClick={()=>setShowPass(!showPass)}>
            <div className={`flex w-[80%] h-[80%] rounded-full ${showPass && "bg-slate-800"}`} />
          </div>

          <div>Show Pass?</div>

          
        </div>
        <div className='flex bg-slate-400 rounded-full px-5 py-2' onClick={handleSubmit}>Unlock</div>
      </form>}
      {showLoadingSpinner && <div>
        <LoadingSpinner />
        </div>}
      
    </div>
  )
}
