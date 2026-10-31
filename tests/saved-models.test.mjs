import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import vm from 'node:vm';
import ts from 'typescript';
const read=p=>fs.readFileSync(p,'utf8');
function evaluate(source,deps={},globals={}){const exports={};vm.runInNewContext(ts.transpileModule(source,{compilerOptions:{module:ts.ModuleKind.CommonJS,target:ts.ScriptTarget.ES2020}}).outputText,{exports,require:p=>deps[p]||{},JSON,Error,...globals});return exports;}
test('models clone the complete form without runtime identity or hardware commands',async()=>{
 const calls=[];const config={id:'active',driver:'toledo8540',serial_port:'COM99',baud_rate:4800,serial_data_bits:7,serial_parity:'even',frame_prefix_hex:'02',initial_weight_kg:400};
 const service=evaluate(read('src/services/savedScaleModels.ts'),{'./tauri':{invokeCommand:(name,args)=>{calls.push({name,args});return Promise.resolve();}},'./scaleSimulator':{defaultScaleConfig:()=>({})}});
 const row={id:'saved',name:'A',revision:2,config};const loaded=service.modelForm(row);assert.equal(loaded.id,null);assert.equal(loaded.serial_port,'COM99');loaded.baud_rate=9600;assert.equal(config.baud_rate,4800);
 await service.saveModel('A',config,row);assert.equal(calls[0].args.expected_revision,2);assert.equal(calls[0].args.config.id,null);assert.equal(config.id,'active');await service.deleteModel(row);assert.equal(calls[1].name,'scale_model_delete');assert.ok(calls.every(c=>!c.name.includes('simulator_start')&&!c.name.includes('simulator_stop')));
});
test('loading form leaves the active list and selected instance unchanged',()=>{
 const calls=[];const vue={ref:v=>({value:v}),computed:f=>({get value(){return f();}}),onMounted(){},onUnmounted(){}};
 const core=evaluate(read('src/services/scaleSimulator.ts'),{'./tauri':{invokeCommand:(name)=>{calls.push(name);if(name!=='scale_simulator_preview_frame')throw Error('Unexpected runtime operation');return Promise.resolve('FRAME');}}});
 const script=read('src/pages/ScaleSimulatorPage.vue').match(/<script setup lang="ts">([\s\S]*?)<\/script>/)[1];
 const page=evaluate(script+'\nexports.state={applySavedModel,form,simulators,selectedSimulatorId};',{'vue':vue,'../services/scaleSimulator':core,'../utils/framePresentation':{}},{window:{}}).state;
 const active=[{id:'A',running:true}];page.simulators.value=active;page.selectedSimulatorId.value='A';page.applySavedModel({...core.defaultScaleConfig(),id:'forbidden',serial_port:'COM99'});assert.equal(page.form.value.id,null);assert.equal(page.simulators.value,active);assert.equal(page.selectedSimulatorId.value,'A');
 assert.ok(!read('src/pages/ScaleSimulatorPage.vue').includes('reconfigure'));
});
