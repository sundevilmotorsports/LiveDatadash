'use client'

import { useEffect, useState } from 'react';
import {invoke} from "@tauri-apps/api"

export default function NumberTest() {
    const [number, setNumber] = useState( ' 42 ' );

    useEffect(() => {
        invoke<string>('numberTest', { number: '200' })
          .then(result => setNumber(result))
          .catch(console.error)
      }, [])

      return <p>{number}</p>
}