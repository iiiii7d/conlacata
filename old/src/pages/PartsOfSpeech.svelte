<script lang="ts">
  import {writable, type Writable} from "svelte/store";

  import Conjugation from "../comps/Conjugation.svelte";
  import { globalPOS, partsOfSpeech, type PartOfSpeechObj, type ConjugationObj, defaultPartOfSpeech, defaultConjugation} from "../_stores";

  let currentPOSIndex: number;
  let currentPOS: PartOfSpeechObj;
  $: currentPOS = $partsOfSpeech[currentPOSIndex] ?? $globalPOS;
  let conjugations: Writable<ConjugationObj[]>;
  $: conjugations = writable(currentPOS.conjugations);

  function addPOS() {
    $partsOfSpeech = [...$partsOfSpeech, defaultPartOfSpeech()];
    currentPOSIndex = $partsOfSpeech.length-1;
  }
  function deletePOS() {
    $partsOfSpeech.splice(currentPOSIndex, 1);
    $partsOfSpeech = $partsOfSpeech;
    currentPOSIndex -= 1;
  }
  function addConj() {
    currentPOS.conjugations = [...currentPOS.conjugations, defaultConjugation()]
  }
</script>
Part of speech: <select bind:value={currentPOSIndex}>
  <option value={-1}>Global</option>
  {#each $partsOfSpeech as pos, index}
    <option value={index}>{pos.name}</option>
  {/each}
</select>
<button on:click={addPOS}><i class="fas fa-plus"></i> Add Part of Speech</button>
<button on:click={deletePOS} disabled={currentPOSIndex == -1}><i class="fas fa-minus"></i> Delete Part of Speech</button><br>
<input type="text" bind:value={currentPOS.name}
  disabled={currentPOSIndex == -1} placeholder="Part of Speech name">
<input type="text" bind:value={currentPOS.abbrev}
  disabled={currentPOSIndex == -1} placeholder="Abbrev. " size="6">

<h2>Conjugations</h2>
<button on:click={addConj}><i class="fas fa-plus"></i> Add Conjugation</button><br>
{#each $conjugations as conj, i}
  <Conjugation {conj} conjList={conjugations} index={i}/>
{/each}
