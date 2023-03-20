<script lang="ts">
  import { setContext } from "svelte";
  import { writable, type Writable } from "svelte/store";
  setContext("activeTabStore", writable(null) as Writable<string>);
  setContext("tabListStore", writable([]) as Writable<string[]>);

  import Editor from "./Editor.svelte";
  import Explorer from "./Explorer.svelte";
</script>

<div class="flex">
  <Explorer />
  <Editor />
</div>
