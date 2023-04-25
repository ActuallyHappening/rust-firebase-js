import { wrapper } from "./helper";

import { getDatabase as _getDatabase } from "firebase/database";
export const getDatabase = wrapper("getDatabase", _getDatabase);

import { onValue as _onValue } from "firebase/database";
export const onValue = wrapper("onValue", _onValue);

import { ref as _ref } from "firebase/database";
export const ref = wrapper("ref", _ref);