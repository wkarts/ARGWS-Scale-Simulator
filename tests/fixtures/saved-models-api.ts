import { defaultScaleConfig } from '../../src/services/scaleSimulator';
const root=window as any;root.__modelCalls=[];
let rows=[{id:'one',name:'Bancada COM10',revision:1,config:{...defaultScaleConfig(),driver:'toledo8540',serial_port:'COM10'}},{id:'two',name:'Modelo com nome longo para verificar responsividade sem expandir a barra',revision:1,config:{...defaultScaleConfig(),serial_port:'COM12'}}];
export const listModels=async()=>{root.__modelCalls.push('list');return JSON.parse(JSON.stringify(rows));};
export const modelForm=(model:any)=>({...JSON.parse(JSON.stringify(model.config)),id:null});
export const saveModel=async(name:string,config:any,selected?:any)=>{root.__modelCalls.push(selected?'update':'save');const old=rows.find(r=>r.id===selected?.id);if(selected&&old?.revision!==selected.revision)throw Error('Modelo alterado. Recarregue.');const model={id:selected?.id??'saved-'+Date.now(),name,revision:(selected?.revision??0)+1,config:{...config,id:null}};rows=[...rows.filter(r=>r.id!==model.id),model];return model;};
export const deleteModel=async(model:any)=>{root.__modelCalls.push('delete');rows=rows.filter(r=>r.id!==model.id);};
