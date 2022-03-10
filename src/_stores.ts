import { readable, writable } from "svelte/store";
import type {Writable} from "svelte/store";

export let pageName = writable("Home");
export const version = readable("v0.0.0");

export let conName = writable("Foobar");

export interface localStorageFormat {
  characters: CharacterObj[]
  otherCharacters: CharacterObj[]
  lexicon: StoredWordObj[] | WordObj[]
  partsOfSpeech: PartOfSpeechObj[],
  globalPOS: PartOfSpeechObj
}
export const defaultLocalStorageFormat = (): localStorageFormat => ({...{
  characters: [],
  otherCharacters: [],
  lexicon: [],
  partsOfSpeech: [],
  globalPOS: defaultPartOfSpeech()
}});
/*function load(): localStorageFormat {

}*/

export interface CharacterObj {
  char: string,
  ipa: string,
}
export const defaultCharacter = (): CharacterObj => ({...{
  char: "",
  ipa: ""
}});
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
export const defaultWord = (): WordObj => ({...{
  conWord: "",
  fromWord: "",
  description: "",
  pronunciation: "",
  partOfSpeech: undefined,
  derivedWords: [],
  tags: [],
  synonyms: [],
  antonyms: []
}});
export interface StoredWordObj {
  conWord: string,
  fromWord: string,
  description: string,
  pronunciation: string,
  partOfSpeech?: string,
  derivedWords: WordObj[],
  tags: string[],
  synonyms: string[],
  antonyms: string[]
}
export const defaultStoredWord = (): StoredWordObj => ({...{
  conWord: "",
  fromWord: "",
  description: "",
  pronunciation: "",
  partOfSpeech: undefined,
  derivedWords: [],
  tags: [],
  synonyms: [],
  antonyms: []
}});

export interface PartOfSpeechObj {
  name: string,
  description: string,
  abbrev: string,
  conjugations: ConjugationObj[]
  conjTableView: ConjTableViewObj,
}
export const defaultPartOfSpeech = (): PartOfSpeechObj => ({...{
  name: "",
  description: "",
  abbrev: "",
  conjugations: [],
  conjTableView: {},
}});
export interface ConjugationObj {
  name: string,
  description: string,
  multiDimensional: boolean,
  dimensions: DimensionObj[],
}
export const defaultConjugation = (): ConjugationObj => ({...{
  name: "",
  description: "",
  multiDimensional: true,
  dimensions: [defaultDimension()]
}});
export interface DimensionObj {
  name: string,
  description: string,
  rules: RuleObj[],
}
export const defaultDimension = (): DimensionObj => ({...{
  name: "",
  description: "",
  rules: []
}})
export interface RuleObj {
  regex: RegExp,
  subst: string
}
export const defaultRule = (): RuleObj => ({...{
  regex: new RegExp(""),
  subst: ""
}})

export let lexicon = writable([] as WordObj[]);

export let partsOfSpeech = writable([] as PartOfSpeechObj[]);
export let globalPOS: Writable<PartOfSpeechObj> = writable(defaultPartOfSpeech());
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