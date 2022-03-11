<script lang="ts">
  import { characters, defaultDimension, globalPOS, otherCharacters,
    partsOfSpeech, type ConjTableViewObj, type ConjugationObj, type DimensionObj, type PartOfSpeechObj } from "../_stores";
  import ConjugationTableAxisSelector from "./ConjugationTableAxisSelector.svelte";
  import {getIPA, pronounceIPA} from "./Word.svelte";


  export let partOfSpeechName: string | undefined;
  $: partOfSpeech = partOfSpeechName ?
    $partsOfSpeech.filter(pos => pos.name == partOfSpeechName)[0] : undefined;
  export let word: string;
  let defaultDim = defaultDimension();
  defaultDim.name = "(Default)";
  

  $: charlist = $otherCharacters.concat($characters)
  $: conjugations = partOfSpeech ? partOfSpeech?.conjugations?.concat($globalPOS.conjugations) : $globalPOS.conjugations;
  $: conjTableView = partOfSpeech ? partOfSpeech?.conjTableView : {};
  $: console.log(partOfSpeech?.conjugations?.concat($globalPOS.conjugations));
  $: console.log(conjugations);
  $: xDims = getDims("x", conjTableView);
  $: yDims = getDims("y", conjTableView);
  $: zDims = getDims("z", conjTableView);

  function getDims(axis: "x" | "y" | "z", conjTableView: ConjTableViewObj | undefined): DimensionObj[] {
    console.log(conjTableView)
    if (conjTableView === undefined) return [defaultDim];
    if (conjTableView[axis] === undefined) return [defaultDim];
    if (!conjTableView[axis]?.multiDimensional) {
      if (conjTableView[axis]?.dimensions[0])
      return [
        defaultDim,
        conjTableView[axis]?.dimensions[0] as DimensionObj
      ]
      else return [
        defaultDim
      ]
    } else {
      return conjTableView[axis]?.dimensions ?? [defaultDim]
    }
  }
  
  function applyConjugation(value: string, ...dims: [DimensionObj, ConjugationObj | undefined][]) {
    for (let [dim, _] of dims.sort(([_1, ac], [_2, bc]) =>
      partOfSpeech?.conjugations.indexOf(ac!)! - partOfSpeech?.conjugations.indexOf(bc!)!)) {
      for (let rule of dim.rules) {
        value = value.replace(new RegExp(rule.regex, "g"), rule.subst);
      }
    }
    return value;
  }
</script>
<style lang="scss">
  .axis-name {
    font-weight: bolder;
    text-decoration: underline;
  }
  table {
    border-collapse: collapse;
    border-style: hidden;
  }

  table td, table th {
      border: 1px solid #aaa;
  }
  caption {
    border-bottom: 1px solid #aaa;
  }
  i.conj-ipa {
    color: #aaa;
  }
</style>
{#if conjugations === undefined}
  <i>No conjugations</i>
{:else}

<ConjugationTableAxisSelector bind:value={conjTableView.x}
  axis="x" {conjugations}/><br>
<ConjugationTableAxisSelector bind:value={conjTableView.y}
  axis="y" {conjugations}/><br>
<ConjugationTableAxisSelector bind:value={conjTableView.z}
  axis="z" {conjugations}/>
{#each zDims as dimz, z}
  <table>
    <caption class="axis-name">{dimz.name || (z == 0 ? "(Default)"
      : conjTableView.z?.multiDimensional ? "unnamed" : conjTableView.z?.name)}</caption>
    <tbody>
      <tr>
        <th></th>
        {#each xDims as dimx, x}
          <th class="axis-name">{dimx.name || (x == 0 ? "(Default)"
            : conjTableView.x?.multiDimensional ? "unnamed" : conjTableView.x?.name)}</th>
        {:else}
          <th class="axis-name">(Default)</th>
        {/each}
      </tr>
        {#each yDims as dimy, y}
          <tr>
            <th class="axis-name">{dimy.name || (y == 0 ? "(Default)"
              : conjTableView.y?.multiDimensional ? "unnamed" : conjTableView.y?.name)}</th>
            {#each xDims as dimx}
              {@const conj = applyConjugation(word,
                [dimx, conjTableView.x],
                [dimy, conjTableView.y],
                [dimz, conjTableView.z])}
              <td>{conj}<br><i class="conj-ipa">{getIPA(conj, charlist)}</i>
                <button on:click={() => pronounceIPA(getIPA(conj, charlist))}><i class="fas fa-headphones"></i></button></td>
            {:else}
              <td>{applyConjugation(word,
                [dimy, conjTableView.y],
                [dimz, conjTableView.z])}<br>
                <i class="conj-ipa">{getIPA(applyConjugation(word,
                  [dimy, conjTableView.y],
                  [dimz, conjTableView.z]), charlist)}</i>
                    <button on:click={() => pronounceIPA(getIPA(applyConjugation(word,
                      [dimy, conjTableView.y],
                      [dimz, conjTableView.z]), charlist))}><i class="fas fa-headphones"></i></button></td>
            {/each}
          </tr>
        {/each}
    </tbody>
  </table>
{/each}
{/if}