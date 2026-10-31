import test from 'node:test';import assert from 'node:assert/strict';import fs from 'node:fs';import vm from 'node:vm';import ts from 'typescript';
test('Playground selects without side effects and starts a copied model only on explicit action',async()=>{
 const calls=[];const active=[{id:'already-running',running:true}];
 const vue={ref:value=>({value}),computed:fn=>({get value(){return fn();}}),onMounted(){},onActivated(){},onDeactivated(){},onBeforeUnmount(){}};
 const services={startScaleSimulator:async config=>{calls.push(JSON.parse(JSON.stringify(config)));return{id:'new'};},listScaleSimulators:async()=>active};
 const models={listModels:async()=>[],modelForm:model=>({...JSON.parse(JSON.stringify(model.config)),id:null})};
 const script=fs.readFileSync('src/pages/PlaygroundPage.vue','utf8').match(/<script setup lang="ts">([\s\S]*?)<\/script>/)[1];const exports={};
 vm.runInNewContext(ts.transpileModule(script+'\nexports.state={models,modelId,start,instances};',{compilerOptions:{module:ts.ModuleKind.CommonJS,target:ts.ScriptTarget.ES2020}}).outputText,{exports,require:path=>path==='vue'?vue:path.includes('savedScaleModels')?models:path.includes('scaleSimulator')?services:{},setInterval,clearInterval,Error});
 const state=exports.state;const model={id:'preset-1',name:'COM10',revision:1,config:{id:'old-runtime',driver:'toledo8540',serial_port:'COM10',baud_rate:4800}};state.models.value=[model];state.instances.value=active;state.modelId.value=model.id;assert.equal(calls.length,0);assert.equal(state.instances.value,active);
 await state.start();assert.equal(calls.length,1);assert.equal(calls[0].id,null);assert.equal(calls[0].serial_port,'COM10');assert.equal(model.config.id,'old-runtime');assert.equal(state.instances.value,active);
 model.config.serial_port='COM99';assert.equal(calls[0].serial_port,'COM10');
});
test('Web simulator bridge requires an authenticated session and explicit permissions',()=>{
 const api=fs.readFileSync('src-tauri/src/internal_api/simulator_commands.rs','utf8');assert.match(api,/authorize_api_command/);assert.match(api,/session_token/);
 const auth=fs.readFileSync('src-tauri/src/commands/auth.rs','utf8');assert.match(auth,/"scale_model_save"[\s\S]*Some\("config:manage"\)/);
 const page=fs.readFileSync('src/pages/PlaygroundPage.vue','utf8');assert.doesNotMatch(page,/stopAllScaleSimulators|reconfigure/);assert.match(page,/onDeactivated\(suspend\)/);
});
