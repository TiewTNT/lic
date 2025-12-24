<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { X } from "@lucide/svelte";

  let images: Set<string> = $state(new Set([]));
  $inspect(images);

  let imgFormat = $state("png");

  let dpi = $state(300)
</script>

{#each images as image}
  <div>
    <span>{image}</span>
    <button onclick={() => images.delete(image)}><X /></button>
  </div>
{/each}

<button
  onclick={async () => {
    images = images.union(
      new Set(
        await open({
          multiple: true,
          directory: false,
          filters: [
            {
              name: "Images",
              extensions: [
                "png",
                "jpg",
                "jpeg",
                "webp",
                "avif",
                "gif",
                "pdf",
                "svg",
              ],
            },
          ],
        }),
      ),
    );
  }}>d</button
>

<button
  onclick={async () => {
    images = new Set();
  }}>c</button
>

<button
  onclick={async () => {
    await invoke("convert_images", {
      strPaths: Array.from(images),
      imgFormat: imgFormat,
      dpi,
    });
  }}>x</button
>

<select bind:value={imgFormat}>
  <option value="png">PNG</option>
  <option value="jpg">JPG</option>
  <option value="gif">GIF</option>
  <option value="webp">WEBP</option>
  <option value="avif">AVIF</option>
  <option value="heic">HEIC</option>
  <option value="tiff">TIFF</option>
</select>

<input placeholder="DPI" type="number" bind:value={dpi} />
