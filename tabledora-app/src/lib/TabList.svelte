<script lang="ts">
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  const tabListStore = getContext("tabListStore") as Writable<string[]>;
</script>

<div class="flex h-10 overflow-x-auto">
  {#each $tabListStore as tab}
    <div class="flex justify-center items-center h-full px-4 py-1 bg-gray-200">
      <span>{tab.substring(tab.lastIndexOf("/"))}</span>
    </div>
  {/each}
</div>
