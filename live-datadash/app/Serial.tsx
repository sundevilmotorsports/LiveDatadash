'use client'

import { useEffect, useState } from 'react';
import {tauri, invoke} from "@tauri-apps/api"


export default function NumberTest() {
    var [number, number_test] = useState(' 41 ' );

    tauri.invoke<string>('numberTest', { number: 200 })
          .then(result => number_test(result))
          .catch(console.error)
    

      return <p>{number}</p>
}