interface ImportMetaEnv {
    readonly VITE_API: string;
    readonly VITE_APP_VERSION: string;
}

interface ImportMeta {
    readonly env: ImportMetaEnv;
}