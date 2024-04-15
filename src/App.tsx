import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import kaden from "./assets/kaden.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import "./App.css";
import { invokeTauriCommand } from "@tauri-apps/api/helpers/tauri";

import GetPort from "./dropdown";

function App() {
const [greetMsg, setGreetMsg] = useState("");
const [name, setName] = useState("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  setGreetMsg(await invoke("greet", { name }));
}

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>
      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={
            (e) => {
              setName(e.currentTarget.value);
            }}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </div>
  );
}

function Serial(){

  const [isConnected, setIsConnected] = useState(false);
  const[chart,setChart] = useState(String);

  type Payload = {
    connected: string;
  };
  
  async function startSerialEventListener() {
    await listen<Payload>("isConnected", (event : any) => {
      console.log(event.payload.message);
      if(event.payload.message === "connected"){
        setIsConnected(false);
      }
    });
  }
  useEffect(() => {
    startSerialEventListener();
  }, []);

  

  function inDrop(port : String) {
    for(var i in document.getElementById('dropper')?.childNodes.values()){
      if(i.localeCompare(port.toString())){
        return true;
      }
    }
    return false;
   }

  async function get_chart(){
    setChart(await invoke("get_chart"));
    return chart
  }

  return(
    <div className= "container">
      <h1> WE CLOWN IT UP IN DAQ </h1>
      <div className= "row">
        <a href = "https://github.com/KadenGreen" target = "_blank">
          <img src= {kaden} className="logo kaden" alt = "goat (?)"></img>
        </a>
      </div>
      {chart.substring(chart.indexOf("</head>") + 8)}
      <div onLoad={get_chart} onClick={get_chart}> we are loading rn
      </div>
      <GetPort />
    </div>
  )
}


export default Serial;
