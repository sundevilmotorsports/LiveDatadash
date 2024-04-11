import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import kaden from "./assets/kaden.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import "./App.css";
import { invokeTauriCommand } from "@tauri-apps/api/helpers/tauri";

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

  type Payload = {
    connected: string;
  };

  type LeanPort = {
    port_name : String,
    port_exists : boolean;
  }
  
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

  async function get_ports(){
    var option : Array<LeanPort> = await invoke("get_ports")
    for(var i = 0; i < option.length; i++){
      if(!inDrop(option[i].port_name)){
        document.getElementById('dropper')?.append(new Option(option[i].port_name.toString()))
      }
    }
  }

  function inDrop(port : String) {
    for(var i in document.getElementById('dropper')?.childNodes.values()){
      if(i.localeCompare(port.toString())){
        return true;
      }
    }
    return false;
   }

  return(
    <div className= "container">
      <h1> WE CLOWN IT UP IN DAQ </h1>
      <div className= "row">
        <a href = "https://github.com/KadenGreen" target = "_blank">
          <img src= {kaden} className="logo kaden" alt = "goat (?)"></img>
        </a>
        <select name = "ports" onLoad={get_ports} onClick={get_ports} id = 'dropper'>test
        </select>
      </div>
    </div>
  )
}


export default Serial;
