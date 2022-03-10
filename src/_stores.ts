import { readable, writable } from "svelte/store";
import type {Writable} from "svelte/store";
import LZString from "lz-string";

export interface localStorageFormat {
  pageName: string,
  conName: string,
  characters: CharacterObj[]
  otherCharacters: CharacterObj[]
  lexicon: StoredWordObj[]
  partsOfSpeech: PartOfSpeechObj[],
  globalPOS: PartOfSpeechObj
}
export const defaultLocalStorageFormat = (): localStorageFormat => ({...{
  pageName: "Home",
  conName: "Foobar",
  characters: [],
  otherCharacters: [],
  lexicon: [],
  partsOfSpeech: [],
  globalPOS: defaultPartOfSpeech()
}});

export interface CharacterObj {
  char: string,
  ipa: string,
}
export const defaultCharacter = (): CharacterObj => ({...{
  char: "",
  ipa: ""
}});

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
let loadedLocalStorage: localStorageFormat = JSON.parse(
  LZString.decompress(localStorage.conlacata ?? LZString.compress("{}"))
   ?? "{}");
// @ts-ignore
if (loadedLocalStorage == {}) loadedLocalStorage = defaultLocalStorageFormat();

export let pageName = writable(loadedLocalStorage.pageName);
export const version = readable("v0.0.0");

export let conName = writable(loadedLocalStorage.conName);

export let characters = writable(loadedLocalStorage.characters);
export let otherCharacters = writable(loadedLocalStorage.otherCharacters);

export let lexicon = writable(loadedLocalStorage.lexicon.map((w): WordObj => {
  if (w.partOfSpeech !== undefined) w.partOfSpeech = 
  return w
}));

export let partsOfSpeech = writable(loadedLocalStorage.partsOfSpeech);
export let globalPOS: Writable<PartOfSpeechObj> = writable(loadedLocalStorage.globalPOS);