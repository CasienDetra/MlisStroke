import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { Key } from "./components/Key";
import "./styles.css";

export default function App() {
  const [combo, setCombo] = useState<string>("");

  useEffect(() => {
    const unlistenPromise = listen<string>("key_combo", (event) => {
      const value = event.payload;

      setCombo(value);
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

    // this not working at all
  const keys = combo.split(" + ");

  return (
<div className="drag container">
  <div className="keystroke">
    {keys.map((k, i) => (
      <Key key={i} value={k} />
    ))}
  </div>
</div>
  );
}
