import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"
import { useSyncExternalStore } from "react"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

// `false` during the static prerender pass (no DOM layout) and during the
// first hydration render, then `true` once mounted in the browser. Lets
// layout-dependent widgets (e.g. recharts ResponsiveContainer, which measures
// its parent and warns when it gets width/height -1) skip the server render
// without tripping a hydration mismatch.
const emptySubscribe = () => () => {}
export function useIsClient() {
  return useSyncExternalStore(
    emptySubscribe,
    () => true,
    () => false,
  )
}
