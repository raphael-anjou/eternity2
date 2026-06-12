import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { HashRouter } from "react-router-dom";
import "./index.css";
import App from "./App";
import { LangProvider } from "./i18n";

// HashRouter: zero-config static hosting (GitHub/GitLab Pages): no server
// rewrites needed, and board-sharing links survive copy/paste.
createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <HashRouter>
      <LangProvider>
        <App />
      </LangProvider>
    </HashRouter>
  </StrictMode>,
);
