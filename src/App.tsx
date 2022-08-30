import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { BrowserRouter, Route, Routes, useNavigate } from "react-router-dom";
import Menu from "./pages/Menu";
import Convert from "./pages/Convert";
import { message } from "@tauri-apps/api/dialog";

function App() {
  const [video, setVideo] = useState<string>();
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="bg-black h-screen">
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Menu />} />
          <Route path="/convert/:filePath" element={<Convert />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;
