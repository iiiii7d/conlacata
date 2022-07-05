<script>
import { characters, otherCharacters } from "../_stores";
import CharacterTable from "../comps/CharacterTable.svelte";
  
</script>
<style lang="scss">
  div {
    float: left;
    padding: 0 10px;
  }
</style>
<div>
  <h2>Main Alphabet</h2>
  <CharacterTable charlist={characters}/><br>
  {$characters.filter(c => c.char.trim() != "").length} letters
</div><div>
  <h2>Other characters<br>& combinations</h2>
  <CharacterTable charlist={otherCharacters}/>
</div><div>
  <h2>Punctuation</h2>
</div>