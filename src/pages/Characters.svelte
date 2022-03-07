<script>
import { characters } from "../_stores";
import CharacterTable from "../comps/CharacterTable.svelte";
  
</script>
<h2>Main Alphabet</h2>
<CharacterTable charlist={$characters}/>

<h2>Other characters</h2>
...
<h2>Punctuation</h2>
...