import { useParams } from "react-router-dom";
import { Buffer } from "buffer";
import { useEffect } from "react";
import { message } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";

export default function Convert() {
  const params = useParams();
  useEffect(() => {
    const convertVideo = async () => {
      if (!params.filePath) {
        message("No file");
        return;
      }

      const filePath = Buffer.from(params.filePath, "base64").toString(
        "binary"
      );

      await invoke("convert_video", { input: filePath, target_size: 7.8 });
      message("done");
    };
  }, []);
  return (
    <div>
      <h1>hello</h1>
      <h1 className="text-white">
        {Buffer.from(
          params.filePath ? params.filePath : "hello",
          "base64"
        ).toString("binary")}
      </h1>
    </div>
  );
}
