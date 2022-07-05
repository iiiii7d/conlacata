<script lang="ts" context="module">
  export function getIPA(word: string, charlist: CharacterObj[]): string {
    let ipa = "";
    if (!word) return "(None)";
    outer: while (true) {
      for (let char of charlist) {
        if (char.char && word.startsWith(char.char)) {
          ipa += char.ipa;
          word = word.slice(char.char.length);
          if (!word) break outer;
          continue outer;
        }
      }
      if (!word) break outer;
      ipa += word[0];
      word = word.slice(1);
    }
    ipa = ipa.replaceAll(/(.)\1/g, "$1ː");
    return ipa || "(None)";
  }

  
  meSpeak.loadConfig("https://raw.githubusercontent.com/mikolalysenko/mespeak/master/src/mespeak_config.json");
  export function pronounceIPA(ipa: string) {
    ipa = ipa.replaceAll('ˈ', '\'').replaceAll('ˌ', ',');
    ipa = ipa.replaceAll('t̠ʃ', "tS").replaceAll("tʃ", "tS");
    ipa = ipa.replaceAll('d̠ʒ', "dZ").replaceAll("dʒ", "dZ");
    ipa = ipa.replaceAll('θ', 'T').replaceAll('ð', 'D');
    ipa = ipa.replaceAll('ʃ', 'S').replaceAll('ʒ', 'Z');
    ipa = ipa.replaceAll('ŋ', 'N');
    ipa = ipa.replaceAll('ç', 'C');
    ipa = ipa.replaceAll('ɹ', 'r');
    ipa = ipa.replaceAll('ɣ', 'X');
    ipa = ipa.replaceAll('ɢ', 'Q');
    ipa = ipa.replaceAll(/[ɑäɐɜ]/g, 'a');
    ipa = ipa.replaceAll(/[əɘ]/g, '@');
    ipa = ipa.replaceAll('ɚ', '3');
    ipa = ipa.replaceAll(/[ɛæ]/g, 'E');
    ipa = ipa.replaceAll('e', 'e');
    ipa = ipa.replaceAll('ɫ', '@L');
    ipa = ipa.replaceAll('i', 'I');
    ipa = ipa.replaceAll('ɪ', 'i');
    ipa = ipa.replaceAll('o', 'O');
    ipa = ipa.replaceAll('ɔ', '0');
    ipa = ipa.replaceAll(/[ʌɤ]/g, 'V');
    ipa = ipa.replaceAll('ʊ', 'U');
    ipa = ipa.replaceAll(/[ʏ]/g, 'y');
    ipa = ipa.replaceAll(/[œø]/g, 'Y');
    ipa = ipa.replaceAll('ʰ', 'h');
    ipa = ipa.replaceAll('ʀ̥', 'x');
    ipa = ipa.replaceAll('ɸ', 'f').replaceAll('β', 'v');
    ipa = ipa.replaceAll('ː', ':');
    console.log(ipa);
    meSpeak.loadVoice("https://raw.githubusercontent.com/mikolalysenko/mespeak/master/voices/en/en.json");
    meSpeak.speak(ipa.split(" ").map(i => "[["+i+"]] ").join("")
      .replaceAll(".]]", "]].").replaceAll(",]]", "]],"), {speed: 150});
  }
</script>
<script lang="ts">
  import ContentEditable from "./ContentEditable.svelte";
  import {conName, otherCharacters, characters,
    partsOfSpeech, type CharacterObj, type WordObj} from "../_stores";
  import type { Writable } from "svelte/store";
  import ConjugationTable from "./ConjugationTable.svelte";

  $: charlist = $otherCharacters.concat($characters)

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
    <i><b>Pronunciaton:</b> <ContentEditable placeholder={getIPA(word.conWord, charlist)} bind:value={word.pronunciation}/></i>
      <button on:click={() => pronounceIPA(word.pronunciation || getIPA(word.conWord, charlist))}><i class="fas fa-headphones"></i></button><br>
    <b>Direct translation:</b> <ContentEditable placeholder="Base language translation..." bind:value={word.fromWord} /><br>
    <b>Part of speech:</b> <select bind:value={word.partOfSpeech}>
      <option value={undefined}>None</option>
      {#each $partsOfSpeech as pos}
        <option value={pos.name}>{pos.name}</option>
      {/each}
    </select><br><br>
    <b>Description:</b> <ContentEditable placeholder="Description" bind:value={word.description} /><br>
  </div>
  <div>
    <details>
      <summary>Conjugation table</summary>
      <ConjugationTable partOfSpeechName={word.partOfSpeech} word={word.conWord}/>
    </details>
  </div>
  <div><i class="fas fa-trash" on:click={deleteWord}></i></div>
</div><br>
<hr>