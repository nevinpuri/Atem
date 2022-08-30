import { useParams } from "react-router-dom";
import { Buffer } from "buffer";

export default function Convert() {
  const params = useParams();
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
