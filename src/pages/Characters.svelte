<script lang="ts">
  import {characters} from "../_stores";
  import Character from "../comps/Character.svelte";

  function addCharacter() {
    $characters = [...$characters, {char: "", ipa: ""}];
  }
</script>
<h2>Main Alphabet</h2>
...
{#each $characters as character, i}
  <Character {...character} index={i} /><br>
{/each}
<button on:click={addCharacter}>Add character</button>

<h2>Other characters</h2>
...
<h2>Punctuation</h2>
...