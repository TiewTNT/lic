<script>
    import { getCurrentWindow } from "@tauri-apps/api/window";

    // when using `"withGlobalTauri": true`, you may use
    // const { getCurrentWindow } = window.__TAURI__.window;

    const appWindow = getCurrentWindow();

        let { children } = $props();
</script>

<div class="titlebar h-10 backdrop-blur-md">
    <div data-tauri-drag-region></div>
    <div class="controls">
        <button id="titlebar-minimize" title="minimize" onclick={appWindow.minimize}>
            <!-- https://api.iconify.design/mdi:window-minimize.svg -->
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
            >
                <path fill="currentColor" d="M19 13H5v-2h14z" />
            </svg>
        </button>
        <button id="titlebar-maximize" title="maximize" onclick={appWindow.toggleMaximize}>
            <!-- https://api.iconify.design/mdi:window-maximize.svg -->
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
            >
                <path fill="currentColor" d="M4 4h16v16H4zm2 4v10h12V8z" />
            </svg>
        </button>
        <button id="titlebar-close" title="close" onclick={appWindow.close}>
            <!-- https://api.iconify.design/mdi:close.svg -->
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
            >
                <path
                    fill="currentColor"
                    d="M13.46 12L19 17.54V19h-1.46L12 13.46L6.46 19H5v-1.46L10.54 12L5 6.46V5h1.46L12 10.54L17.54 5H19v1.46z"
                />
            </svg>
        </button>
    </div>
</div>

<div class="mt-10">
{@render children()}
</div>