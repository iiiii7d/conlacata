<script lang="ts">
import { characters, defaultDimension, globalPOS, otherCharacters,
    partsOfSpeech, type ConjTableViewObj, type ConjugationObj, type DimensionObj } from "../_stores";
import Character from "./Character.svelte";
  import ConjugationTableAxisSelector from "./ConjugationTableAxisSelector.svelte";
  import ConjugationTableGlobalConjApply from "./ConjugationTableGlobalSelector.svelte";
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
  $: conjX = conjugations.filter(conj => conj.name == conjTableView['x'])[0] as ConjugationObj | undefined;
  $: conjY = conjugations.filter(conj => conj.name == conjTableView['y'])[0] as ConjugationObj | undefined;
  $: conjZ = conjugations.filter(conj => conj.name == conjTableView['z'])[0] as ConjugationObj | undefined;
  $: xDims = getDims(conjX);
  $: yDims = getDims(conjY);
  $: zDims = getDims(conjZ);
  let dimGlobalNames: [string, string][] = [];
  $: globalDims = getGlobalDims(dimGlobalNames)
  function getGlobalDims(dg: ([string, string] | undefined)[]): [DimensionObj, ConjugationObj][] {
    return dg.map(obj => {
      if (obj === undefined) return undefined;
      let [conj, dim] = obj;
      let conjObj = conjugations.filter(c => c.name == conj)[0];
      let dimObj = !conjObj.multiDimensional ? conjObj.dimensions[0]
        : conjObj.dimensions.filter(d => d.name == dim)[0];
      return [dimObj, conjObj]
    }).filter(dim => dim !== undefined) as [DimensionObj, ConjugationObj][]
  }

  function getDims(conj: ConjugationObj | undefined): DimensionObj[] {
    if (conj === undefined) return [defaultDim];
    if (!conj?.multiDimensional) {
      if (conj?.dimensions[0])
      return [
        defaultDim,
        conj?.dimensions[0] as DimensionObj
      ]
      else return [
        defaultDim
      ]
    } else {
      return conj?.dimensions ?? [defaultDim]
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
  axis="x" {conjugations} otherSelected={[conjTableView.y, conjTableView.z, ...dimGlobalNames.map(o => o[1])]}/><br>
<ConjugationTableAxisSelector bind:value={conjTableView.y}
  axis="y" {conjugations} otherSelected={[conjTableView.x, conjTableView.z, ...dimGlobalNames.map(o => o[1])]}/><br>
<ConjugationTableAxisSelector bind:value={conjTableView.z}
  axis="z" {conjugations} otherSelected={[conjTableView.x, conjTableView.y, ...dimGlobalNames.map(o => o[1])]}/><br>
<ConjugationTableGlobalConjApply bind:values={dimGlobalNames}
 {conjugations} otherSelected={[conjTableView.x, conjTableView.y, conjTableView.z]} /><br>
{#each zDims as dimz, z}
  <table>
    <caption class="axis-name">{dimz.name || (z == 0 ? "(Default)"
      : conjZ?.multiDimensional ? "unnamed" : conjZ?.name)}</caption>
    <tbody>
      <tr>
        <th></th>
        {#each xDims as dimx, x}
          <th class="axis-name">{dimx.name || (x == 0 ? "(Default)"
            : conjX?.multiDimensional ? "unnamed" : conjX?.name)}</th>
        {:else}
          <th class="axis-name">(Default)</th>
        {/each}
      </tr>
        {#each yDims as dimy, y}
          <tr>
            <th class="axis-name">{dimy.name || (y == 0 ? "(Default)"
              : conjY?.multiDimensional ? "unnamed" : conjY?.name)}</th>
            {#each xDims as dimx}
              {@const conj = applyConjugation(word,
                [dimx, conjX],
                [dimy, conjY],
                [dimz, conjZ],
                ...globalDims)}
              <td>{conj}<br><i class="conj-ipa">{getIPA(conj, charlist)}</i>
                <button on:click={() => pronounceIPA(getIPA(conj, charlist))}><i class="fas fa-headphones"></i></button></td>
            {:else}
              <td>{applyConjugation(word,
                [dimy, conjY],
                [dimz, conjZ])}<br>
                <i class="conj-ipa">{getIPA(applyConjugation(word,
                  [dimy, conjY],
                  [dimz, conjZ],
                  ...globalDims), charlist)}</i>
                    <button on:click={() => pronounceIPA(getIPA(applyConjugation(word,
                      [dimy, conjY],
                      [dimz, conjZ],
                      ...globalDims), charlist))}><i class="fas fa-headphones"></i></button></td>
            {/each}
          </tr>
        {/each}
    </tbody>
  </table>
{/each}
{/if}