<script lang="ts">
  import Word from "../comps/Word.svelte";
  import {lexicon} from "../_stores";

  function addWord() {
    $lexicon = [...$lexicon, {
      conWord: "",
      fromWord: "",
      description: "",
      pronunciation: "",
      partOfSpeech: undefined,
      derivedWords: [],
      tags: [],
      synonyms: [],
      antonyms: []
    }]
  }
</script>
<style lang="scss">
  div {
    float: left;
  }
</style>
<div>
  <button on:click={addWord}><i class="fas fa-plus"></i> Add word</button><br>
  {#each $lexicon as word, i}
    <Word {word} index={i} lexicon={lexicon}/>
  {/each}
</div>