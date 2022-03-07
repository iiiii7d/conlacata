<script lang="ts">
  import {type CharacterObj} from "../_stores";
  import Character from "../comps/Character.svelte";
  export let charlist: CharacterObj[] = [];
  function addCharacter() {
    charlist = [...charlist, {char: "", ipa: ""}];
  }
</script>
<style lang="scss">
  table {
    width: 30%;
  }
</style>
<table>
  <tbody>
    {#each charlist as character, i}
      <Character {...character} index={i} charlist={charlist} />
    {/each}
  </tbody>
</table>
<button on:click={addCharacter}>Add character</button>