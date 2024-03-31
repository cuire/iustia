/// <reference types="vite/client" />
interface ImportMetaEnv {
  readonly VITE_IUSTIA_BACKEND_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
