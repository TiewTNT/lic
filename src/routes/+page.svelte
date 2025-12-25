<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { X, Plus, Trash, Repeat } from "@lucide/svelte";
  import { flip } from "svelte/animate";
  import { slide } from "svelte/transition";
  import { openPath } from "@tauri-apps/plugin-opener";
  import { getCurrentWebview } from "@tauri-apps/api/webview";
  import { onDestroy, onMount } from "svelte";
  import type { UnlistenFn } from "@tauri-apps/api/event";

  let imagesRecord: Record<string, boolean | null> = $state({});
  $inspect(imagesRecord);

  let imgFormat = $state("png");

  let dpi = $state(300);

  let inputFmtArr = ["png", "jpg", "jpeg", "webp", "avif", "gif", "pdf", "svg"];

  let unlisten: UnlistenFn;
  onMount(async () => {
    unlisten = await getCurrentWebview().onDragDropEvent((event) => {
      const type = event.payload.type;
      if (type === "drop") {
        console.log("File paths:", event.payload.paths);
        const next = { ...imagesRecord };
        for (let path of event.payload.paths) {
          if (
            inputFmtArr.includes(path.split(".").at(-1)?.toLowerCase() ?? "")
          ) {
            next[path] = next[path] ?? null;
          } else {
            console.error(path);
          }
        }
        imagesRecord = next;
      } else if (type === "over") {
        //console.log('Hovering files');
      } else if (type === "leave") {
        //console.log('Drag cancelled');
      }
    });
  });

  onDestroy(() => {
    unlisten();
  });
</script>

<div class="p-10">
  <div class="pathList">
    {#each Object.entries(imagesRecord) as [image, status], index (image)}
      <div
        class={(() => {
          switch (status) {
            case true:
              return "inline-flex gap-5 p-4 dark:bg-success-900/30 bg-success-200 border border-b-0 border-[#80808030] w-full";
            case false:
              return "inline-flex gap-5 p-4 dark:bg-error-900/30 bg-error-200 border border-b-0 border-[#80808030] w-full";
            default:
              return "inline-flex gap-5 p-4 dark:bg-surface-900 border border-b-0 border-[#80808030] w-full";
          }
        })()}
        transition:slide={{ duration: 200 }}
        animate:flip={{ duration: 200 }}
      >
        <button
          class="text-white/50 truncate rtl text-lefttruncate overflow-hidden whitespace-nowrap text-left [direction:rtl]"
          onclick={async () =>
            await openPath(image.split(/\/|\\/).slice(0, -1).join("/"))}
        >
          <span>
            {image.split(/\/|\\/).slice(0, -1).join("/") ?? ""}/
          </span>
          <strong
            class="dark:text-surface-contrast-900 text-surface-contrast-50"
          >
            {image.split(/\/|\\/).at(-1) ?? ""}
          </strong>
        </button>
        <button
          class="ml-auto"
          onclick={() => {
            const next = { ...imagesRecord };
            delete next[image];
            imagesRecord = next;
          }}
        >
          <X />
        </button>
      </div>
    {/each}
  </div>
  <div class="m-2 w-full text-white/50 text-center text-sm">
    {#if Object.keys(imagesRecord).length}Selected files has format{#if Object.keys(imagesRecord).length > 1}s{/if}{/if}
    {(() => {
      let extSet = new Set();
      for (let img of Object.keys(imagesRecord)) {
        extSet.add(img.split(".").at(-1)?.toUpperCase());
      }
      return Array.from(extSet);
    })().join(", ")}
  </div>

  {#if Object.keys(imagesRecord).length}
    <div class="h-40"></div>
  {/if}

  <div
    class="fixed bottom-10 left-1/2 -translate-x-1/2
    inline-flex gap-5 border rounded-3xl border-[#ffffff30]
    backdrop-blur-md backdrop-brightness-75 p-5"
  >
    <div class="inline-flex rounded-2xl shadow-lg">
      <button
        class="transition-transform active:translate-y-1 flex size-12 content-center align-middle items-center border-[#80808030] border dark:bg-surface-900 rounded-l-2xl"
        onclick={async () => {
          const picked = await open({
            multiple: true,
            directory: false,
            filters: [
              {
                name: "Images",
                extensions: inputFmtArr,
              },
            ],
          });
          if (!picked) return;
          const pickedArr = Array.isArray(picked) ? picked : [picked];
          const next = { ...imagesRecord };
          for (const p of pickedArr) {
            next[p] = next[p] ?? null;
          }
          imagesRecord = next;
        }}><Plus class="size-5 mx-auto" /></button
      >

      <button
        class="transition-transform active:translate-y-1 flex border-[#80808030] border border-l-0 size-12 content-center align-middle items-center dark:bg-surface-900 rounded-r-2xl"
        onclick={async () => {
          imagesRecord = {};
        }}><Trash class="size-5 mx-auto" /></button
      >
    </div>
    <div class="inline-flex rounded-2xl shadow-lg">
      <button
        class="transition-transform active:rotate-180 active:scale-90 flex size-12 content-center align-middle items-center dark:bg-primary-700 bg-primary-200 rounded-2xl border-[#80808030] border"
        onclick={async () => {
          const next = { ...imagesRecord };
          for (const image of Object.keys(imagesRecord)) {
            next[image] = await invoke("convert_image", {
              strPath: image,
              imgFormat: imgFormat,
              dpi,
            });
            console.log(next[image]);
          }
          imagesRecord = next;
        }}><Repeat class="size-5 mx-auto" /></button
      >
    </div>
    <div class="inline-flex rounded-2xl shadow-lg">
      <div
        class="flex border-[#80808030] border dark:bg-surface-900 rounded-2xl px-4 w-25 align-middle"
      >
        <select
          bind:value={imgFormat}
          class="select ring-0 focus:outline-0"
          onchange={() => {
            for (let k in imagesRecord) {
              imagesRecord[k] = null;
            }
          }}
        >
          <option value="png">PNG</option>
          <option value="jpg">JPG</option>
          <option value="gif">GIF</option>
          <option value="webp">WEBP</option>
          <option value="avif">AVIF</option>
        </select>
      </div>
    </div>

    {#if Object.keys(imagesRecord).filter((path) => path.endsWith(".pdf") || path.endsWith(".svg")).length}
      <div class="inline-flex rounded-2xl shadow-lg">
        <div
          class="flex border-[#80808030] border dark:bg-surface-900 rounded-2xl"
        >
          <input
            placeholder="DPI"
            class="focus:outline-0 px-4 align-middle w-30"
            type="number"
            bind:value={dpi}
          />
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  @reference "tailwindcss";
  .pathList > :first-child {
    @apply rounded-t-2xl;
  }
  .pathList > :last-child {
    @apply border-b rounded-b-2xl;
  }
</style>
