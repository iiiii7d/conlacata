import { readable, writable } from "svelte/store";

export let name = writable("Home");
export const version = readable("v0.0.0");

export interface CharacterObj {
  char: string,
  ipa: string,
}

export let characters = writable([] as CharacterObj[]);
export let otherCharacters = writable([] as CharacterObj[]);
/*export let shiftKey = writable(false);

document.addEventListener("keydown", (e) => {
  shiftKey.set(true);
});
document.addEventListener("keyup", (e) => {
  shiftKey.set(false);
});
*/