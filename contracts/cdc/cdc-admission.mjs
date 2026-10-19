// CDC normalization policy, not a source-specific PostgreSQL/binlog adapter.
// A vendor opaque cursor is never ordered lexically or compared by timestamp.
export class CdcAdmissionError extends Error {
 constructor(code){super(code);this.code=code;this.name='CdcAdmissionError';}
}
const fail=code=>{throw new CdcAdmissionError(code)};
const identity=x=>typeof x==='string'&&/^[A-Za-z0-9][A-Za-z0-9_.:-]{0,127}$/.test(x);
export function validateCdcCheckpoint(c){
 if(!c||typeof c!=='object'||Array.isArray(c)||c.version!==1)fail('invalid_checkpoint');
 for(const k of ['provider','sourceId','sourceGeneration','tenantId','databaseId','collectionId','schemaVersion','snapshotId','logicalPosition']){
   if(!identity(c[k]))fail('invalid_'+k);
 }
 if(typeof c.opaqueProviderCursor!=='string'||c.opaqueProviderCursor.length>2048)fail('invalid_opaque_cursor');
 if(!['snapshotting','incremental','gap','expired','schema_incompatible'].includes(c.status))fail('invalid_status');
 if(typeof c.snapshotComplete!=='boolean')fail('invalid_snapshot_flag');
 if(c.status==='incremental'&&!c.snapshotComplete)fail('incremental_without_completed_snapshot');
 if(!Number.isSafeInteger(c.observedAtMs)||c.observedAtMs<0)fail('invalid_observation');
 return Object.freeze({...c});
}
export function decideCdcResume(previous,current,{logicalComparable=false,comparePositions}={}){
 const old=validateCdcCheckpoint(previous),now=validateCdcCheckpoint(current);
 const keys=['provider','sourceId','tenantId','databaseId','collectionId'];
 if(keys.some(k=>old[k]!==now[k]))return 'deny_cross_scope';
 if(old.sourceGeneration!==now.sourceGeneration||old.snapshotId!==now.snapshotId)return 'resnapshot';
 if(old.schemaVersion!==now.schemaVersion||now.status==='schema_incompatible')return 'schema_reconcile';
 if(['expired','gap'].includes(now.status))return 'resnapshot';
 if(now.status!=='incremental'||!now.snapshotComplete)return 'pending_snapshot';
 // Neither time nor arbitrary opaque provider cursor compares CDC progress.
 if(!logicalComparable||typeof comparePositions!=='function')return 'require_authoritative_position';
 const result=comparePositions(old.logicalPosition,now.logicalPosition);
 if(!Number.isInteger(result)||![-1,0,1].includes(result))fail('invalid_position_comparator');
 if(result===1)return 'reject_regression';
 return 'resume';
}
