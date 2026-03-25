import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import "./styles.css";

export default function App() {
  const [combo, setCombo] = useState<string>("");
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    const unlistenPromise = listen<string>("key_combo", (event) => {
      const value = event.payload;

      setCombo(value);
      setVisible(true);

      // auto hide after 1.2s
      // setTimeout(() => {
      //   setVisible(false);
      // }, 1200);
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  const keys = combo.split(" + ");

  return (
    <div className="container">
      {visible && (
        <div className="keystroke">
          {keys.map((key, index) => (
            <div key={index} className="key">
              {key}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
