<script lang="ts">
  import type { FileTreeNode } from "../types";

  export let fileTreeNode: FileTreeNode;
  export let depth = 0;
  async function handleClickFileTreeNode(event: MouseEvent) {
    console.log(fileTreeNode.path);
  }
</script>

<div>
  <button
    on:click={handleClickFileTreeNode}
    class="flex w-full hover:bg-gray-300 hover:cursor-pointer"
  >
    {#each Array(depth) as _}
      <div class="w-2" />
    {/each}
    <span>{fileTreeNode.name}</span>
  </button>
  {#if fileTreeNode.children}
    {#each fileTreeNode.children as child (child.path)}
      <svelte:self fileTreeNode={child} depth={depth + 1} />
    {/each}
  {/if}
</div>
