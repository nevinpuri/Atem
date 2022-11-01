import { useNavigate, useParams } from "react-router-dom";
import { useEffect, useMemo, useState } from "react";
import { message } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import debounce from "lodash.debounce";
import { fromBase64, toBase64 } from "../utils";

export default function Convert() {
  const router = useNavigate();
  const [convertStatus, setConvertStatus] = useState<string>("Compressing");
  const { filePath } = useParams();

  const convertVideo = async () => {
    try {
      if (!filePath) {
        message("No file");
        return;
      }

      const decodedFilePath = fromBase64(filePath);

      const out: any = await invoke("convert_video", {
        input: decodedFilePath,
        targetSize: 7.5,
      });

      if (!out) {
        setConvertStatus("Failed to convert. Check console for details");
        return;
      }

      setConvertStatus("Successfully converted");

      return router(`/success/${toBase64(out)}`);
    } catch (err: any) {
      setConvertStatus(
        "An unexpected error has occured. Check the console for details."
      );
      console.log(err);
    }
  };

  const debouncedEventHandler = useMemo(() => debounce(convertVideo, 300), []);
  useEffect(() => {
    debouncedEventHandler();
  }, []);

  return (
    <div className="h-screen flex justify-center items-center">
      <h1 className="text-white text-2xl font-bold text-center">
        {convertStatus}
      </h1>
      <svg
        className="text-white"
        height={42}
        width={42}
        version="1.1"
        id="L9"
        xmlns="http://www.w3.org/2000/svg"
        xmlnsXlink="http://www.w3.org/1999/xlink"
        x="0px"
        y="0px"
        viewBox="0 0 100 100"
        enableBackground="new 0 0 0 0"
        xmlSpace="preserve"
      >
        <path
          fill="#fff"
          d="M73,50c0-12.7-10.3-23-23-23S27,37.3,27,50 M30.9,50c0-10.5,8.5-19.1,19.1-19.1S69.1,39.5,69.1,50"
        >
          <animateTransform
            attributeName="transform"
            attributeType="XML"
            type="rotate"
            dur="1s"
            from="0 50 50"
            to="360 50 50"
            repeatCount="indefinite"
          />
        </path>
      </svg>
    </div>
  );
}
