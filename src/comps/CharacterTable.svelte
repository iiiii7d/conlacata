<script lang="ts">
  import {type CharacterObj} from "../_stores";
  import Character from "../comps/Character.svelte";
  import type { Writable } from "svelte/store";

  export let charlist: Writable<CharacterObj[]>;
  function addCharacter() {
    $charlist = [...$charlist, {char: "", ipa: ""}];
  }
</script>
<style lang="scss">
  
</style>
<table>
  <tbody>
    {#each $charlist as character, i}
      <Character {...character} index={i} charlist={charlist} />
    {/each}
  </tbody>
</table>
<button on:click={addCharacter}><i class="fas fa-plus"></i> Add character</button>