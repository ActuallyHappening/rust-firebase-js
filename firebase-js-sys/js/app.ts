import { wrapper } from "./helper";

const _app: {[key: string]: (...any: any) => any} = {};

import { initializeApp } from "firebase/app";
_app.initializeApp = wrapper("initializeApp", initializeApp); 

import { FirebaseError } from "firebase/app";
// _app.FirebaseError = FirebaseError;
let g: FirebaseError;

export const app = _app;