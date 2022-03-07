<script lang="ts">
  import type { CharacterObj } from "src/_stores";
  import type { Writable } from "svelte/store";


  export let char: string;
  export let ipa: string;
  export const index: number = 0;
  export let charlist: Writable<CharacterObj[]>;
  
</script>
<style lang="scss">
  td {
    height: 100%;
    width: 100%;
    display: inline;
    &#delete {
      color: red;
      text-align: center;
      border-radius: 5px;
      
      &:hover {
        background-color: rgb(255, 0, 0, 0.5);
        cursor: pointer;
      }
      &:active {
        background-color: red;
        opacity: 1;
      }
    }
  }
</style>
<tr>
  <td><input type="text" bind:value={char} size="2"></td>
  <td><input type="text" bind:value={ipa} size="2"></td>
  <td id="delete" on:click={() => $charlist.splice(index, 1); charlis}>&nbsp;<i class="fas fa-times"></i>&nbsp;</td>
</tr>