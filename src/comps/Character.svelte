<script lang="ts">
  import type { CharacterObj } from "../_stores";
  import type { Writable } from "svelte/store";


  export let index: number;
  export let character: CharacterObj;
  export let charlist: Writable<CharacterObj[]>;
  
  function deleteChar() {
    $charlist.splice(index, 1);
    $charlist = $charlist;
  }

  function moveUpChar() {
    if (index != 0) {
      let prevVal = $charlist[index-1];
      $charlist[index-1] = $charlist[index];
      $charlist[index] = prevVal;
    }
  }

  function moveDownChar() {
    if (index != $charlist.length-1) {
      let prevVal = $charlist[index+1];
      $charlist[index+1] = $charlist[index];
      $charlist[index] = prevVal;
    }
  }
</script>
<style lang="scss">
  @import "../_global";
  tr {
    display: block;
  }
  td {
    height: 100%;
    width: 100%;
    display: inline;

    &#delete {
      color: red;
      text-align: center;
      border-radius: 5px;
      @include unclickable;

      
      &:hover {
        background-color: #f008;
        cursor: pointer;
      }
      &:active {
        background-color: red;
        opacity: 1;
      }
    }
    &#move {
      color: #bbb;
      text-align: center;
      border-radius: 5px;
      @include unclickable;

      
      &:hover {
        background-color: #aaa;
        cursor: pointer;
      }
      &:active {
        background-color: #bbb;
      }
    }
  }
</style>
<tr>
  <td><input type="text" bind:value={character.char} size="3"
    on:change={() => $charlist = $charlist} placeholder="char"></td>
  <td><input type="text" bind:value={character.ipa} size="3"
    on:change={() => $charlist = $charlist} placeholder="ipa"></td>
  <td id="move" on:click={moveUpChar}>&nbsp;<i class="fas fa-chevron-up"></i>&nbsp;</td>
  <td id="move" on:click={moveDownChar}>&nbsp;<i class="fas fa-chevron-down"></i>&nbsp;</td>
  <td id="delete" on:click={deleteChar}>&nbsp;<i class="fas fa-times"></i>&nbsp;</td>
</tr>