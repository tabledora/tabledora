<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  let SourceList = [];

  async function handleClickButtonOpenFolder(event: MouseEvent) {
    let path: string = await invoke("open_local_folder");
    if (path != null) SourceList = [path];
  }

  import Source from "./Source.svelte";
</script>

<div>
  {#each SourceList as source, i (source)}
    <Source {source} />
  {:else}
    <div class="mt-4">
      <button
        on:click={handleClickButtonOpenFolder}
        class="block mx-auto bg-gray-700 px-8 py-1 hover:bg-gray-600"
      >
        <span class="text-sm text-white">Open Folder</span>
      </button>
    </div>
  {/each}
</div>
