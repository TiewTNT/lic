<script lang="ts">
    import { Minus, Square, X } from "@lucide/svelte";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    // when using `"withGlobalTauri": true`, you may use
    // const { getCurrentWindow } = window.__TAURI__.window;

    const appWindow = getCurrentWindow();

    let { children } = $props();
</script>

<div class="titlebar h-10 backdrop-blur-md">
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_mouse_events_have_key_events -->
    <div
        class="m-1 size-8 mr-auto active:scale-90 active:rotate-360 transition-all"
        onmousedown={() => {
            const staticIcon = document.getElementById("appIcon");
            const animIcon = document.getElementById("appIconAnim");

            staticIcon?.style.setProperty("opacity", "0");
            animIcon?.style.setProperty("opacity", "1");

            const anims = document.querySelectorAll(
                "#appIconAnim animateMotion",
            ) as NodeListOf<SVGAnimationElement>;

            anims.forEach((a) => {
                a.endElement();
                a.beginElement();
            });
        }}
        onmouseleave={() => {
            document
                .getElementById("appIconAnim")
                ?.style.setProperty("opacity", "0");
            document
                .getElementById("appIcon")
                ?.style.setProperty("opacity", "1");
        }}
        onmouseup={() => {
            document
                .getElementById("appIconAnim")
                ?.style.setProperty("opacity", "0");
            document
                .getElementById("appIcon")
                ?.style.setProperty("opacity", "1");
        }}
    >
        <svg
            class="size-8 transition-colors"
            version="1.1"
            viewBox="0 0 9 9"
            xmlns="http://www.w3.org/2000/svg"
            id="appIcon"
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

        <svg
            viewBox="0 0 9 9"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            class="size-8 -translate-y-8 transition-colors"
            style="opacity: 0"
            id="appIconAnim"
        >
            <!-- Black tracks (masked) -->
            <g
                fill="none"
                stroke="currentColor"
                stroke-linecap="round"
                mask="url(#m)"
            >
                <path d="M2.5 1.5h-1v6h2v-6z" />
                <path d="M3.5 6.5v-5h-2v6h2z" />
                <path d="M7.5 4.5v-3h-2v6h2z" />
            </g>

            <defs>
                <mask id="m" maskUnits="userSpaceOnUse">
                    <!-- White background -->
                    <rect width="9" height="9" fill="white" />

                    <!-- Mask box 1 -->
                    <rect
                        x="-0.55"
                        y="-0.55"
                        width="1.1"
                        height="1.1"
                        fill="black"
                    >
                        <animateMotion
                            dur="2.5s"
                            repeatCount="indefinite"
                            path="M2.5 1.5 H1.5 V7.5 H3.5 V1.5 Z"
                            id="iconSvgAnim"
                            begin="indefinite"
                        />
                    </rect>

                    <!-- Mask box 2 -->
                    <rect
                        x="-0.55"
                        y="-0.55"
                        width="1.1"
                        height="1.1"
                        fill="black"
                    >
                        <animateMotion
                            dur="2.5s"
                            repeatCount="indefinite"
                            path="M3.5 6.5 V1.5 H1.5 V7.5 H3.5 Z"
                            id="iconSvgAnim"
                            begin="indefinite"
                        />
                    </rect>

                    <!-- Mask box 3 -->
                    <rect
                        x="-0.55"
                        y="-0.55"
                        width="1.1"
                        height="1.1"
                        fill="black"
                    >
                        <animateMotion
                            dur="2.5s"
                            repeatCount="indefinite"
                            path="M7.5 4.5 V1.5 H5.5 V7.5 H7.5 Z"
                            id="iconSvgAnim"
                            begin="indefinite"
                        />
                    </rect>
                </mask>
            </defs>
        </svg>
    </div>
    <div data-tauri-drag-region class="w-full"></div>
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
