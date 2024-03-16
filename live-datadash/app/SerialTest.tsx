'use client'

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

export default function serialTest() {

  let [serial, setVec] = useState("vec undefined");

  useEffect(() => {
    invoke<string>('get_data', {})
      .then(result => setVec(result))
      .catch(console.error)
  }, [])

  const handleClick = () =>{
    invoke<string>('get_data', {})
      .then(result => setVec(result))
      .catch(console.error)
  }

  return <button type="button" onClick={handleClick}>{serial}</button>
}