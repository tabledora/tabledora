<script lang="ts">
  import type { FileTreeNode } from "../types";
  import { invoke } from "@tauri-apps/api";

  export let source: string;

  const fileTreeNodeListPromise = invoke("list_children", {
    root: source,
  }) as Promise<FileTreeNode[]>;

  import FileTree from "./FileTree.svelte";
</script>

<div>
  <div class="bg-gray-700 px-1 py-0.5 text-xs text-white">
    <span>{source}</span>
  </div>
  {#await fileTreeNodeListPromise}
    <div>
      <span>Loading...</span>
    </div>
  {:then fileTreeNodeList}
    {#each fileTreeNodeList as fileTreeNode (fileTreeNode.path)}
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
