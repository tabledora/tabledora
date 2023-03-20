<script lang="ts">
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import type { FileTreeNode } from "../types";

  export let fileTreeNode: FileTreeNode;
  export let depth = 0;

  const tabListStore = getContext("tabListStore") as Writable<string[]>;
  const activeTabStore = getContext("activeTabStore") as Writable<string>;

  function handleClickFileTreeNode(event: MouseEvent) {
    if (fileTreeNode.children == null) {
      tabListStore.update((tabList) => {
        if (!tabList.includes(fileTreeNode.path)) {
          return [...tabList, fileTreeNode.path];
        }
        return tabList;
      });

      activeTabStore.set(fileTreeNode.path);
    }
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
