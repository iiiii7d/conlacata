<script lang="ts">
  import { characters, defaultDimension, globalPOS, otherCharacters, type ConjTableViewObj, type DimensionObj, type PartOfSpeechObj } from "../_stores";
  import ConjugationTableAxisSelector from "./ConjugationTableAxisSelector.svelte";
  import {getIPA} from "./Word.svelte";


  export let partOfSpeech: PartOfSpeechObj | undefined;
  export let word: string;
  let defaultDim = defaultDimension();
  defaultDim.name = "(Default)";
  

  let charlist = $otherCharacters.concat($characters)
  let conjugations = partOfSpeech ? partOfSpeech?.conjugations?.concat($globalPOS.conjugations) : $globalPOS.conjugations;
  let conjTableView = partOfSpeech ? partOfSpeech?.conjTableView : {};
  console.log(partOfSpeech);
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
  
  function applyConjugation(value: string, ...dims: DimensionObj[]) {
    for (let dim of dims) {
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
    <caption class="axis-name">{dimz.name || (z == 0 ? "(Default)" : "unnamed")}</caption>
    <tbody>
      <tr>
        <th></th>
        {#each xDims as dimx, x}
          <th class="axis-name">{dimx.name || (x == 0 ? "(Default)" : "unnamed")}</th>
        {:else}
          <th class="axis-name">(Default)</th>
        {/each}
      </tr>
        {#each yDims as dimy, y}
          <tr>
            <th class="axis-name">{dimy.name || (y == 0 ? "(Default)" : "unnamed")}</th>
            {#each xDims as dimx}
              {@const conj = applyConjugation(word, dimx, dimy, dimz)}
              <td>{conj}<br><i class="conj-ipa">{getIPA(conj, charlist)}</i></td>
            {:else}
              <td>{applyConjugation(word, dimy, dimz)}<br>
                <i class="conj-ipa">{getIPA(applyConjugation(word, dimy, dimz), charlist)}</i></td>
            {/each}
          </tr>
        {/each}
    </tbody>
  </table>
{/each}
{/if}