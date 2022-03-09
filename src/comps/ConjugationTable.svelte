<script lang="ts">
import type { PartOfSpeechObj } from "src/_stores";
import Conjugation from "./Conjugation.svelte";
import Dimension from "./Dimension.svelte";


  export let partOfSpeech: PartOfSpeechObj;
  // TODO combine with global conjugations
</script>
<label for="z">z = </label><select name="z" bind:value={partOfSpeech.conjTableView.z}>
  <option value={undefined}>(None)</option>
  {#each partOfSpeech.conjugations as conj}
    <option value={conj}>{conj.name}</option>
  {/each}
</select>
<table>
  <tbody>
    <tr>
      <th>
        <label for="x">x = </label><select name="x" bind:value={partOfSpeech.conjTableView.x}>
          <option value={undefined}>(None)</option>
          {#each partOfSpeech.conjugations as conj}
            <option value={conj}>{conj.name}</option>
          {/each}
        </select>
        <label for="y">y = </label><select name="y" bind:value={partOfSpeech.conjTableView.y}>
          <option value={undefined}>(None)</option>
          {#each partOfSpeech.conjugations as conj}
            <option value={conj}>{conj.name}</option>
          {/each}
        </select>
      </th>
      {#if partOfSpeech.conjTableView.x !== undefined}
      {#each partOfSpeech.conjTableView.x.dimensions as dimx}
        <th>{dimx.name}</th>
      {/each}
      {:else}
        <th>(Default)</th>
      {/if}
    </tr>
    {#if partOfSpeech.conjTableView.y !== undefined}
      {#each partOfSpeech.conjTableView.y.dimensions as dimy}
        <tr>
          <th>{dimy.name}</th>
          {#if partOfSpeech.conjTableView.x !== undefined}
          {#each partOfSpeech.conjTableView.x.dimensions as dimx}
            <td>TODO</td>
          {/each}
          {:else}
            <td>TODO</td>
          {/if}
        </tr>
      {/each}
      {:else}
        <tr>
          <th>(Default)</th>
          {#if partOfSpeech.conjTableView.x !== undefined}
          {#each partOfSpeech.conjTableView.x.dimensions as dimx}
            <td>TODO</td>
          {/each}
          {:else}
            <td>TODO</td>
          {/if}
        </tr>
      {/if}
  </tbody>
</table>