<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Source from "./Source.svelte";
  let sourceList = [];

  async function handleClickButtonOpenFolder(event: MouseEvent) {
    let path: string = await invoke("open_local_folder");
    if (path != null) sourceList = [path];
  }
</script>

<div>
  {#each sourceList as source, i (source)}
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
