'use client'

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

export default function serialTest() {

  let [serial, setVec] = useState('200');

  useEffect(() => {
    invoke<string>('get_data', { testing : "test" })
      .then(result => setVec(result))
      .catch(console.error)
  }, [])

  return <p>{serial}</p>;
}