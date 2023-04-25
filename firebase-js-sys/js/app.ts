import { wrapper } from "./helper";

const _app: {[key: string]: (...any: any) => any} = {};

import { initializeApp as _initializeApp } from "firebase/app";
_app.initializeApp = wrapper("initializeApp", _initializeApp); 

export const app = _app;