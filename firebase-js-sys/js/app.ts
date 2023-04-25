import { wrapper } from "./helper";

import { initializeApp as _initializeApp } from "firebase/app";
export const initializeApp = wrapper("initializeApp", _initializeApp); 