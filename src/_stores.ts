import { writable } from "svelte/store";

export const name = writable("Home");
export const version = writable("v0.0.0");

export interface Character {
  char: string,
  ipa: string,
}

export let characters = writable([] as Character[]);