import { useEffect, useState } from "react";
import { initEngine } from "./index";

/** True once the WASM engine is instantiated (init is idempotent). */
export function useEngine(): boolean {
  const [ready, setReady] = useState(false);
  useEffect(() => {
    let alive = true;
    void initEngine().then(() => {
      if (alive) setReady(true);
    });
    return () => {
      alive = false;
    };
  }, []);
  return ready;
}
