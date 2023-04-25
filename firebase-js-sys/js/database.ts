import { wrapper } from "./helper";

const _database: {[key: string]: (...any: any) => any} = {};

// import { initializeApp as _initializeApp } from "firebase/app";
// _db.initializeApp = wrapper("initializeApp", _initializeApp); 

import { getDatabase as _getDatabase } from "firebase/database";
_database.getDatabase = wrapper("getDatabase", _getDatabase);

import { onValue as _onValue } from "firebase/database";
_database.onValue = wrapper("onValue", _onValue);

import { ref as _ref } from "firebase/database";
_database.ref = wrapper("ref", _ref);



export const database = _database;