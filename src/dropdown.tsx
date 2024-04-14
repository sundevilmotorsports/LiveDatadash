import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "@/components/ui/select"
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { Button } from "./components/ui/button";
import { RefreshCw } from "lucide-react";

type LeanPort = {
    port_name : String,
    port_exists : boolean;
}

let option : Array<LeanPort> = await invoke("get_ports")
  
function GetPort(){
    const [options, setOptions] = useState(option);
    const [currOption, setCurrOption] = useState("");



    async function updateDrop() {
        console.log("updated");
        setOptions(await invoke("get_ports"))
        await invoke("update_port", {name : currOption});
    }

    return (
        <div>
            <div className="flex flex-row space-x-1 my-2">
                <Select onValueChange={(e) => {
                    setCurrOption(e)
                    }}>
                <SelectTrigger className="w-[180px]">
                    <SelectValue placeholder="Select Port"/>
                </SelectTrigger>
                <SelectContent onClick={updateDrop}>
                    {options.map( (port) => {
                        if(port.port_exists){
                            return (
                                <SelectItem value = {port.port_name.toString()}>{port.port_name}</SelectItem>
                            )
                        }
                    })}
                </SelectContent>
                </Select>
                <Button onClick={updateDrop}><RefreshCw/></Button>
                
            </div>
            <p className="flex">{!(options.length < 3) ? "" : "no ports available"}</p>
      </div>
    )
  }
export default GetPort;