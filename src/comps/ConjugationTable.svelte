<script lang="ts">
  import { defaultDimension, globalPOS, type DimensionObj, type PartOfSpeechObj } from "../_stores";
  import ConjugationTableAxisSelector from "./ConjugationTableAxisSelector.svelte";


  export let partOfSpeech: PartOfSpeechObj | undefined;
  export let word: string;
  let defaultDim = defaultDimension();
  defaultDim.name = "(Default)";
  
  function getDims(axis: "x" | "y" | "z", pos: PartOfSpeechObj | undefined): DimensionObj[] {
    if (pos == undefined) return [defaultDim];
    if (pos.conjTableView[axis] === undefined) return [defaultDim];
    if (!pos?.conjTableView[axis]?.multiDimensional) {
      if (pos?.conjTableView[axis]?.dimensions[0])
      return [
        defaultDim,
        pos?.conjTableView[axis]?.dimensions[0] as DimensionObj
      ]
      else return [
        defaultDim
      ]
    } else {
      return pos?.conjTableView[axis]?.dimensions ?? [defaultDim]
    }
  }
  $: xDims = getDims("x", partOfSpeech);
  $: yDims = getDims("y", partOfSpeech);
  $: zDims = getDims("z", partOfSpeech);
  
  function applyConjugation(value: string, ...dims: DimensionObj[]) {
    for (let dim of dims) {
      for (let rule of dim.rules) {
        value = value.replace(rule.regex, rule.subst);
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
</style>
{#if partOfSpeech === undefined}
  <i>No conjugations</i>
{:else}

<ConjugationTableAxisSelector bind:value={partOfSpeech.conjTableView.x}
  axis="x" conjugations={partOfSpeech.conjugations.concat($globalPOS.conjugations)}/><br>
<ConjugationTableAxisSelector bind:value={partOfSpeech.conjTableView.y}
  axis="y" conjugations={partOfSpeech.conjugations.concat($globalPOS.conjugations)}/><br>
<ConjugationTableAxisSelector bind:value={partOfSpeech.conjTableView.z}
  axis="z" conjugations={partOfSpeech.conjugations.concat($globalPOS.conjugations)}/>
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
              <td>{applyConjugation(word, dimx, dimy, dimz)}</td>
            {:else}
              <td>{applyConjugation(word, dimy, dimz)}</td>
            {/each}
          </tr>
        {/each}
    </tbody>
  </table>
{/each}
{/if}