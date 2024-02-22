'use client'

import { useEffect, useState } from 'react';
import {invoke} from "@tauri-apps/api";

export default function NumberTest() {
    var [number, setNumber] = useState(" 500 ");
    
    useEffect(() => {
        invoke<string>('number_test', { number: 200 })
          .then(result => setNumber(result))
          .then(response => console.log(response))
          .catch(console.error)
      }, [])

      //number = "testing";

      return <p>{number}</p>
}