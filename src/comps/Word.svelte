<script lang="ts">
  import ContentEditable from "./ContentEditable.svelte";
  import {conName, partsOfSpeech, type WordObj} from "../_stores";
  import type { Writable } from "svelte/store";
  import ConjugationTable from "./ConjugationTable.svelte";

  export let lexicon: Writable<WordObj[]>;
  export let index: number;

  export let word: WordObj;

  function deleteWord() {
    $lexicon.splice(index, 1);
    $lexicon = $lexicon;
  }

</script>
<style lang="scss">
  @import "../global";
  h2 {
    margin-bottom: 10px;
    margin-top: 0px;
  }
  details {
    border: 1px solid #ccc;
    padding: 5px;
  }
  div {
    display: inline-block;
    & > div {
      //border: 1px solid #aaa;
      //border-radius: 10px;
      padding: 5px;
      float: left;

      @include delete-button("& > i.fas.fa-trash");
    }
  }
</style>
<div>
  <div>
    <h2><ContentEditable placeholderColor="gray" placeholder={`Word in ${$conName}`} bind:value={word.conWord}/></h2>
    <i><b>Pronunciaton:</b> <ContentEditable placeholder={"todo..."} bind:value={word.pronunciation}/></i><br>
    <b>Direct translation:</b> <ContentEditable placeholder="Base language translation..." bind:value={word.fromWord} /><br>
    <b>Part of speech:</b> <select bind:value={word.partOfSpeech}>
      <option value={undefined}>None</option>
      {#each $partsOfSpeech as pos}
        <option value={pos}>{pos.name}</option>
      {/each}
    </select><br><br>
    <b>Description:</b> <ContentEditable placeholder="Description" bind:value={word.description} /><br>
  </div>
  <div>
    <details>
      <summary>Conjugation table</summary>
      <ConjugationTable partOfSpeech={word.partOfSpeech} word={word.conWord}/>
    </details>
  </div>
  <div><i class="fas fa-trash" on:click={deleteWord}></i></div>
</div><br>
<hr>