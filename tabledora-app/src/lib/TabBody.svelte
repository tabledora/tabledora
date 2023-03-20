<script lang="ts">
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";

  const activeTabStore = getContext("activeTabStore") as Writable<string>;
</script>

<div>
  {$activeTabStore}
</div>
