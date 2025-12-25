<script>
    import { Minus, Square, X } from "@lucide/svelte";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    // when using `"withGlobalTauri": true`, you may use
    // const { getCurrentWindow } = window.__TAURI__.window;

    const appWindow = getCurrentWindow();

    let { children } = $props();
</script>

<div class="titlebar h-10 backdrop-blur-md">
    <div data-tauri-drag-region>
        <div class="m-2 size-8 mr-auto">
            <svg
                class="size-8"
                version="1.1"
                viewBox="0 0 9 9"
                xmlns="http://www.w3.org/2000/svg"
            >
                <g stroke-width="0">
                    <g fill="currentColor">
                        <path
                            d="m1 1v7h3v-1h-2v-6z"
                            style="font-variation-settings:'wght' 500;paint-order:stroke markers fill"
                        />
                        <path
                            d="m5 1v7h3v-3h-1v2h-1v-5h1v2h1v-3z"
                            style="font-variation-settings:'wght' 500;paint-order:stroke markers fill"
                        />
                        <path
                            d="m3 1v5h1v-5h-1"
                            style="font-variation-settings:'wght' 500;paint-order:stroke markers fill"
                        />
                    </g>
                </g>
            </svg>
        </div>
    </div>
    <div class="controls">
        <button
            id="titlebar-minimize"
            title="minimize"
            class="size-10 rounded-b-md hover:scale-95 hover:bg-surface-500/20"
            onclick={appWindow.minimize}
        >
            <Minus />
        </button>
        <button
            id="titlebar-maximize"
            title="maximize"
            class="size-10 rounded-b-md hover:scale-95 hover:bg-surface-500/20"
            onclick={appWindow.toggleMaximize}
        >
            <Square />
        </button>
        <button
            id="titlebar-close"
            title="close"
            class="size-10 rounded-bl-md hover:scale-95 hover:bg-red-500/20"
            onclick={appWindow.close}
        >
            <X />
        </button>
    </div>
</div>

<div class="mt-10">
    {@render children()}
</div>
