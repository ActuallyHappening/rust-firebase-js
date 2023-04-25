import { wrapper } from "./helper";

const _database: {[key: string]: (...any: any) => any} = {};

// import { initializeApp as _initializeApp } from "firebase/app";
// _db.initializeApp = wrapper("initializeApp", _initializeApp); 

import { ref as _ref } from "firebase/database";
_database.ref = wrapper("ref", _ref);

export const database = _database;