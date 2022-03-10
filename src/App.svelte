<script lang="ts">
  import Header from "./Header.svelte";
  import Sidebar from "./Sidebar.svelte";
  import Home from "./pages/Home.svelte";
  import Characters from "./pages/Characters.svelte";
  import {pageName, conName, characters, otherCharacters, lexicon,
    partsOfSpeech, globalPOS, type localStorageFormat, type StoredWordObj} from "./_stores";
  import Lexicon from "./pages/Lexicon.svelte";
  import PartsOfSpeech from "./pages/PartsOfSpeech.svelte";
  import LZString from "lz-string";

  $: {
    let saved: localStorageFormat = {
      pageName: $pageName,
      conName: $conName,
      characters: $characters,
      otherCharacters: $otherCharacters,
      partsOfSpeech: $partsOfSpeech,
      globalPOS: $globalPOS,
      lexicon: $lexicon.map(word => {
        if (word.partOfSpeech !== undefined)
          (word as StoredWordObj).partOfSpeech = word.partOfSpeech.name;
        return word as StoredWordObj
      })
    };
    localStorage.conlacata = LZString.compress(JSON.stringify(saved));
  }

</script>
<style>
  main {
    margin: 15px;
    transform: translateX(50px);
    max-width: calc(100vw - 70px);
  }
</style>

<Sidebar />

<Header />
<main>
  {#if $pageName == "Home"}<Home />
  {:else if $pageName == "Characters"}<Characters />
  {:else if $pageName == "Lexicon"}<Lexicon />
  {:else if $pageName == "Parts of Speech"}<PartsOfSpeech />
  {/if}
</main>