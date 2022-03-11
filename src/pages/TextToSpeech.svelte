<script lang="ts">

  import ContentEditable from "../comps/ContentEditable.svelte";
  import { pronounceIPA, getIPA } from "../comps/Word.svelte";
  import { characters, conName, otherCharacters } from "../_stores";
  let text: string;

  $: charlist = $otherCharacters.concat($characters)

</script>
Input {$conName} text here: <ContentEditable placeholder="(input)" bind:value={text} />
<button on:click={() => pronounceIPA(getIPA(text, charlist))}>Pronounce</button><br>
<i style="color: #aaa">IPA: {getIPA(text, charlist)}</i>