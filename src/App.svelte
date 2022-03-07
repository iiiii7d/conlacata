<script lang="ts">
  import Header from "./Header.svelte";
  import Sidebar from "./Sidebar.svelte";
  import Home from "./pages/Home.svelte";
  import Characters from "./pages/Characters.svelte";
  import { name } from "./_stores";
</script>
<style>
  main {
    margin: 15px;
    transform: translateX(50px);
    max-width: calc(100vw - 70px);
  }

  /*main * {
    max-width: calc(100vw - 70px);
  }*/
</style>

<Sidebar />

<Header />
<main>
  {#if $name == "Home"}<Home />
  {:else if $name == "Characters"}<Characters />
  {/if}
</main>