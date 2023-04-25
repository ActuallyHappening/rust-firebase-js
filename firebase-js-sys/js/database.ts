import { wrapper } from "./helper";

const _database: {[key: string]: any } = {};

// import { initializeApp as _initializeApp } from "firebase/app";
// _db.initializeApp = wrapper("initializeApp", _initializeApp); 

import { getDatabase } from "firebase/database";
_database.getDatabase = wrapper("getDatabase", getDatabase);

import { onValue } from "firebase/database";
_database.onValue = wrapper("onValue", onValue);

import { ref } from "firebase/database";
_database.ref = wrapper("ref", ref);

import { DataSnapshot } from "firebase/database";
_database.DataSnapshot = DataSnapshot;

DataSnapshot.prototype.val;




export const database = _database;