<script lang="ts">
  import FileTree from "./FileTree.svelte";
  import type { FileTreeNode } from "../types";
  import { invoke } from "@tauri-apps/api";
  export let source;

  let promiseFileTreeNodes = invoke("list_children", {
    root: source,
  }).then((v) => {
    console.log(v);
    return v;
  }) as Promise<FileTreeNode[]>;
</script>

<div>
  <div class="bg-gray-700 px-1 py-0.5 text-xs text-white">
    <span>{source}</span>
  </div>
  {#await promiseFileTreeNodes}
    <div>
      <span>Loading...</span>
    </div>
  {:then fileTreeNodes}
    {#each fileTreeNodes as fileTreeNode (fileTreeNode.path)}
      <FileTree {fileTreeNode} />
    {:else}
      <div>
        <span>No files in this source.</span>
      </div>
    {/each}
  {:catch error}
    <p>Something went wrong: {error.message}</p>
  {/await}
</div>
