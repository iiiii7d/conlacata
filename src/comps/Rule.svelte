<script lang="ts">
  import type { RuleObj } from "../_stores";
  import type { Writable } from "svelte/store";


  export let rule: RuleObj;
  export let rules: Writable<RuleObj[]>;
  export let index: number;
  let regexError: boolean
  let preRegex = rule.regex.toString().slice(1, rule.regex.toString().length-1);
  $: try {
    new RegExp(preRegex);
    rule.regex = preRegex;
    regexError = false;
  }
  catch (SyntaxError) {
    regexError = true;
  }

  function deleteRule() {
    $rules.splice(index, 1);
    $rules = $rules;
  }
</script>
<style lang="scss">
  @import "../global";
  @include delete-button("i.fas.fa-trash");
  input.error {
    color: red;
  }
</style>
<input type="text" bind:value={preRegex} class:error={regexError} size="5">
<i class="fas fa-chevron-right"></i>
<input type="text" bind:value={rule.subst} size="5">
<i class="fas fa-trash" on:click={deleteRule}></i><br>