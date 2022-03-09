
<script lang="ts">
  export let value: string = "";
  export let placeholder: string = "";
  export let placeholderColor: string = "grey";
  export let disabled: boolean = false;
</script>
<style lang="scss">
  [contenteditable] {
    &:hover {
      text-decoration: underline;
      cursor: text;
    }
    &:focus {
      text-decoration: underline;
    }
    & + .fas {
      visibility: hidden !important;
    }
    &:hover + .fas {
      visibility: visible !important;
    }
    &:empty::before {
      content: attr(placeholder);
      color: var(--placeholderColor);
    }
  }
</style>
{#if disabled}
  <span>{value || placeholder}</span>
{:else}
  <span contenteditable {placeholder} style={`--placeholderColor: ${placeholderColor}`}
  bind:innerHTML={value}></span> <i class="fas fa-pen"></i>
{/if}