import test from 'node:test';
import assert from 'node:assert/strict';
import {validateCdcCheckpoint,decideCdcResume} from './cdc-admission.mjs';
const base={version:1,provider:'postgres',sourceId:'db1',sourceGeneration:'gen1',tenantId:'tenant1',databaseId:'db1',collectionId:'orders',schemaVersion:'v1',snapshotId:'s1',logicalPosition:'lsn1',opaqueProviderCursor:'ENCODED',status:'incremental',snapshotComplete:true,observedAtMs:1000};
const cmp=(a,b)=>{const x=Number(a.slice(3)),y=Number(b.slice(3));return Math.sign(x-y)};
test('strict normalized checkpoint shape and full-snapshot barrier',()=>{
 assert.equal(validateCdcCheckpoint(base).snapshotComplete,true);
 assert.throws(()=>validateCdcCheckpoint({...base,snapshotComplete:false}),/incremental_without_completed_snapshot/);
 assert.throws(()=>validateCdcCheckpoint({...base,tenantId:''}),/invalid_tenantId/);
});
test('generation, scope, schema and retention gaps cannot replay an old cursor',()=>{
 assert.equal(decideCdcResume(base,{...base,sourceGeneration:'gen2'}),'resnapshot');
 assert.equal(decideCdcResume(base,{...base,tenantId:'other'}),'deny_cross_scope');
 assert.equal(decideCdcResume(base,{...base,status:'gap'}),'resnapshot');
 assert.equal(decideCdcResume(base,{...base,schemaVersion:'v2'}),'schema_reconcile');
});
test('opaque cursor and wall-clock alone never prove progression',()=>{
 const changed={...base,opaqueProviderCursor:'AAAAAAAA',observedAtMs:2000,logicalPosition:'lsn2'};
 assert.equal(decideCdcResume(base,changed),'require_authoritative_position');
 assert.equal(decideCdcResume(base,changed,{logicalComparable:true,comparePositions:cmp}),'resume');
 assert.equal(decideCdcResume(changed,base,{logicalComparable:true,comparePositions:cmp}),'reject_regression');
});
