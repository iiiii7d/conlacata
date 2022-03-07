<script lang="ts">
  import Header from "./Header.svelte";
  import Sidebar from "./Sidebar.svelte";
  import Home from "./pages/Home.svelte";
  import Characters from "./pages/Characters.svelte";
  import { pageName } from "./_stores";
  import Lexicon from "./pages/Lexicon.svelte";
</script>
<style>
  main {
    margin: 15px;
    transform: translateX(50px);
    max-width: calc(100vw - 70px);
  }
</style>

<Sidebar />

<Header />
<main>
  {#if $pageName == "Home"}<Home />
  {:else if $pageName == "Characters"}<Characters />
  {:else if $pageName == "Lexicon"}<Lexicon />
  {/if}
</main>