/// <reference types="vite/client" />
interface ImportMetaEnv {
  readonly VITE_GA_ID?: string;
  readonly VITE_SITE_ORIGIN?: string;
  readonly VITE_BASE_PATH?: string;
}
interface ImportMeta {
  readonly env: ImportMetaEnv;
}
