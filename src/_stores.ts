import { readable, writable } from "svelte/store";
import type {Writable} from "svelte/store";

export let pageName = writable("Home");
export const version = readable("v0.0.0");

export let conName = writable("Foobar");

export interface CharacterObj {
  char: string,
  ipa: string,
}
export let characters = writable([] as CharacterObj[]);
export let otherCharacters = writable([] as CharacterObj[]);

export interface WordObj {
  conWord: string,
  fromWord: string,
  description: string,
  pronunciation: string,
  partOfSpeech?: PartOfSpeechObj,
  derivedWords: WordObj[],
  tags: string[],
  synonyms: string[],
  antonyms: string[]
}

export interface PartOfSpeechObj {
  name: string,
  description: string,
  abbrev: string,
  conjugations: ConjugationObj[]
  conjTableView: ConjTableViewObj,
}
export interface ConjugationObj {
  name: string,
  description: string,
  multiDimensional: boolean,
  dimensions: DimensionObj[],
}
export interface DimensionObj {
  name: string,
  description: string,
  rules: RuleObj[],
}
export interface RuleObj {
  regex: RegExp,
  subst: string
}

export let lexicon = writable([] as WordObj[]);

export let partsOfSpeech = writable([] as PartOfSpeechObj[]);
export let globalPOS: Writable<PartOfSpeechObj> = writable({
  name: "",
  description: "",
  abbrev: "",
  conjugations: [],
  conjTableView: {},
});
export interface ConjTableViewObj {
  x?: ConjugationObj,
  y?: ConjugationObj,
  z?: ConjugationObj
}
/*export let shiftKey = writable(false);

document.addEventListener("keydown", (e) => {
  shiftKey.set(true);
});
document.addEventListener("keyup", (e) => {
  shiftKey.set(false);
});
*/