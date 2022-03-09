<script lang="ts">
  import { globalPOS, partsOfSpeech, type PartOfSpeechObj, type Conjugation} from "../_stores";

  let currentPOSIndex: number;
  let currentPOS: PartOfSpeechObj;
  $: currentPOS = $partsOfSpeech[currentPOSIndex] ?? $globalPOS;

  function addPOS() {
    $partsOfSpeech = [...$partsOfSpeech, {
      name: "",
      description: "",
      abbrev: "",
      conjugations: []
    }];
    currentPOSIndex = $partsOfSpeech.length-1;
  }
  function deletePOS() {
    $partsOfSpeech.splice(currentPOSIndex, 1);
    $partsOfSpeech = $partsOfSpeech;
    currentPOSIndex -= 1;
  }
  function addConj() {
    currentPOS.conjugations = [...currentPOS.conjugations, {
      name: "",
      description: "",
      dimensional: true,
      dimensions: []
    }]
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
<button on:click={addConj}><i class="fas fa-plus"></i> Add Conjugation</button>
{#each currentPOS.conjugations as conj, conjIndex}
  TODO
{/each}