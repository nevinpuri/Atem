import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  useEffect(() => {
    console.log("hello");
    listen("tauri://file-drop", (event) => {
      console.log(event);
    });
  }, []);

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="bg-black h-screen flex justify-center items-center">
      <h1 className="text-white text-2xl font-bold text-center">
        Drag and Drop Files to Compress
      </h1>
    </div>
  );
}

export default App;
