<script lang="ts">
  import ContentEditable from "./ContentEditable.svelte";
  import {conName, partsOfSpeech, type WordObj} from "../_stores";
  import type { Writable } from "svelte/store";

  export let lexicon: Writable<WordObj[]>;
  export let index: number;

  export let conWord: string;
  export let pronunciation: string;
  export let fromWord: string;
  export let description: string;
  export let partOfSpeech: string;
  let derivedWords: WordObj[] = [];
  let tags: string[] = [];
  let synonyms: string[] = [];
  let antonyms: string[] = [];

  function deleteWord() {
    $lexicon.splice(index, 1);
    $lexicon = $lexicon;
  }
  
  $: $lexicon[index] = {
    conWord, pronunciation, fromWord, description,
    partOfSpeech, derivedWords, tags, synonyms, antonyms
  }

</script>
<style lang="scss">
  h2 {
    margin-bottom: 10px;
    margin-top: 0px;
  }
  div {
    display: inline-block;
    & > div {
      //border: 1px solid #aaa;
      //border-radius: 10px;
      padding: 5px;
      float: left;

      & > i.fas.fa-trash {
        color: red;
        padding: 2px;
        &:hover {
          background-color: #f008;
        }
        &:active {
          opacity: 1;
          color: white;
        }
      }
    }
  }
</style>
<div>
  <div>
    <h2><ContentEditable placeholderColor="gray" placeholder={`Word in ${$conName}`} bind:value={conWord}/></h2>
    <i><b>Pronunciaton:</b> <ContentEditable placeholder={"todo..."} bind:value={pronunciation}/></i><br>
    <b>Translation:</b> <ContentEditable placeholder="Base language translation..." bind:value={fromWord} /><br>
    <b>Part of speech:</b> <select bind:value={partOfSpeech}>
      <option value="">None</option>
      {#each $partsOfSpeech as pos}
        <option value={pos.name}>{pos.name}</option>
      {/each}
    </select><br><br>
    <b>Description:</b> <ContentEditable placeholder="Description" bind:value={description} /><br>
  </div>
  <div>
    TODO conjugation table
  </div>
  <div><i class="fas fa-trash" on:click={deleteWord}></i></div>
</div><br>
<hr>