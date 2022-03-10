<script lang="ts">
  import {type CharacterObj, defaultCharacter} from "../_stores";
  import Character from "../comps/Character.svelte";
  import type { Writable } from "svelte/store";

  export let charlist: Writable<CharacterObj[]>;
  function addCharacter() {
    $charlist = [...$charlist, defaultCharacter()];
  }
</script>
<style lang="scss">
  
</style>
<table>
  <tbody>
    {#each $charlist as character, i}
      <Character {character} {charlist} index={i}/>
    {/each}
  </tbody>
</table>
<button on:click={addCharacter}><i class="fas fa-plus"></i> Add character</button>