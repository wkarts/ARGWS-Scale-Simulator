import { useSessionStore } from '../stores/session';
function invokeCommand<T>(name:string,args:Record<string,unknown>={}) { return rawInvoke<T>(name,{...args,session_token:useSessionStore().sessionToken}); }
import { invokeCommand as rawInvoke } from './tauri';
import { defaultScaleConfig, type ScaleSimulatorConfig } from './scaleSimulator';
export interface SavedScaleModel { id:string; name:string; config:ScaleSimulatorConfig; revision:number }
export function modelForm(model:SavedScaleModel):ScaleSimulatorConfig { return {...defaultScaleConfig(),...JSON.parse(JSON.stringify(model.config)),id:null}; }
export const listModels=()=>invokeCommand<SavedScaleModel[]>('scale_models_list');
export const saveModel=(name:string,config:ScaleSimulatorConfig,selected?:SavedScaleModel)=>invokeCommand<SavedScaleModel>('scale_model_save',{id:selected?.id??null,name,config:{...config,id:null},expected_revision:selected?.revision??null});
export const deleteModel=(model:SavedScaleModel)=>invokeCommand<void>('scale_model_delete',{id:model.id,expected_revision:model.revision});
