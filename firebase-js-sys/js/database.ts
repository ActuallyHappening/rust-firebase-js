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

import { push } from "firebase/database";
_database.push = wrapper("push", push);

import { onChildAdded } from "firebase/database";
_database.onChildAdded = wrapper("onChildAdded", onChildAdded);
import { onChildChanged } from "firebase/database";
_database.onChildChanged = wrapper("onChildChanged", onChildChanged);
import { onChildRemoved } from "firebase/database";
_database.onChildRemoved = wrapper("onChildRemoved", onChildRemoved);
import { onChildMoved } from "firebase/database";
_database.onChildMoved = wrapper("onChildMoved", onChildMoved);

import { orderByChild } from "firebase/database";
_database.orderByChild = wrapper("orderByChild", orderByChild);
import { orderByKey } from "firebase/database";
_database.orderByKey = wrapper("orderByKey", orderByKey);
import { orderByValue } from "firebase/database";
_database.orderByValue = wrapper("orderByValue", orderByValue);

import {Query} from "firebase/database";
let g: Query;


export const database = _database;