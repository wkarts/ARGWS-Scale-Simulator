<script setup lang="ts">
import {ref,computed,onMounted,onActivated,onDeactivated,onBeforeUnmount} from 'vue';
import BasePage from '../components/base/BasePage.vue';
import {listModels,modelForm,type SavedScaleModel} from '../services/savedScaleModels';
import {startScaleSimulator,listScaleSimulators,stopScaleSimulator,setScaleSimulatorWeight,executeScaleSimulatorAction,type ScaleSimulatorStatus} from '../services/scaleSimulator';
const models=ref<SavedScaleModel[]>([]),modelId=ref(''),instances=ref<ScaleSimulatorStatus[]>([]),weights=ref<Record<string,number>>({}),busy=ref(false),error=ref('');
const selected=computed(()=>models.value.find(m=>m.id===modelId.value));let timer:ReturnType<typeof setInterval>|null=null;let generation=0;let refreshing=false;
async function refresh(){if(refreshing)return;refreshing=true;const current=generation;try{const rows=await listScaleSimulators();if(current===generation)instances.value=rows;}catch(e){if(current===generation)error.value=String(e);}finally{refreshing=false;}}
async function reloadModels(){try{models.value=await listModels();}catch(e){error.value=String(e);}}
async function run(operation:()=>Promise<unknown>){if(busy.value)return;busy.value=true;error.value='';try{await operation();await refresh();}catch(e){error.value=e instanceof Error?e.message:String(e);}finally{busy.value=false;}}
async function start(){const model=selected.value;if(!model)return;await run(()=>startScaleSimulator(modelForm(model)));}
function begin(){if(timer!==null)return;void refresh();void reloadModels();timer=setInterval(refresh,1000);}
function suspend(){generation++;if(timer!==null)clearInterval(timer);timer=null;}
onMounted(begin);onActivated(begin);onDeactivated(suspend);onBeforeUnmount(suspend);
</script>
<template>
<BasePage class="scale-playground-page" title="Playground" subtitle="Escolha um modelo salvo e inicie uma balança pronta para testar." icon="activity">
<section class="playground-launch"><label>Modelo pronto<select v-model="modelId" :disabled="busy"><option value="">Selecione um modelo salvo</option><option v-for="model in models" :key="model.id" :value="model.id">{{model.name}}</option></select></label><button class="primary" :disabled="!selected || busy" @click="start">{{busy?'Aguarde…':'Iniciar instância'}}</button><button :disabled="busy" @click="reloadModels">Atualizar modelos</button><router-link to="/simulador-balancas">Criar ou editar modelos</router-link>
<p v-if="selected">{{selected.config.driver}} · {{selected.config.mode}} · {{selected.config.mode.startsWith('serial')?selected.config.serial_port:`${selected.config.host}:${selected.config.port}`}}</p><p>Selecionar um modelo não inicia nem altera nada. Uma porta ocupada gera aviso, sem interromper outra balança.</p></section>
<p v-if="error" class="playground-error" role="alert">{{error}}</p>
<div class="playground-grid"><article v-for="instance in instances" :key="instance.id" class="playground-scale"><h3>{{instance.name}}</h3><p>{{instance.driver}} · {{instance.mode.startsWith('serial')?instance.serial_port:`${instance.host}:${instance.port}`}}</p><output class="weight">{{instance.net_kg.toFixed(3)}} kg</output><p>Bruto {{instance.gross_kg}} kg · Tara {{instance.tare_kg}} kg · {{instance.stable?'Estável':'Oscilando'}} · {{instance.running?'Em execução':'Parada'}}</p>
<fieldset :disabled="busy || !instance.running"><label>Peso bruto (kg)<input v-model.number="weights[instance.id]" type="number" step="any" :placeholder="String(instance.gross_kg)" /></label><button class="primary" :disabled="!Number.isFinite(weights[instance.id])" @click="run(()=>setScaleSimulatorWeight(instance.id,weights[instance.id]))">Aplicar peso</button><div class="playground-controls"><button @click="run(()=>executeScaleSimulatorAction(instance.id,'zero'))">Zerar</button><button @click="run(()=>executeScaleSimulatorAction(instance.id,'tare'))">Tarar</button><button @click="run(()=>executeScaleSimulatorAction(instance.id,'clear_tare'))">Limpar tara</button><button @click="run(()=>executeScaleSimulatorAction(instance.id,'gross'))">Bruto</button><button @click="run(()=>executeScaleSimulatorAction(instance.id,'net'))">Líquido</button><button @click="run(()=>executeScaleSimulatorAction(instance.id,instance.stable?'unstable':'stable'))">{{instance.stable?'Oscilar':'Estabilizar'}}</button><button @click="run(()=>stopScaleSimulator(instance.id))">Parar esta instância</button></div></fieldset><details><summary>Último quadro transmitido</summary><pre>{{instance.last_frame}}</pre></details></article></div>
<p v-if="!instances.length">Nenhuma instância iniciada. Salve um modelo na área de simulação e selecione-o acima.</p>
</BasePage>
</template>
<style scoped>
.playground-launch,.playground-scale{padding:1.25rem;border:1px solid var(--border-color,#cbd5e1);background:var(--surface-color,#fff);border-radius:12px}.playground-launch{display:flex;align-items:end;gap:.75rem;flex-wrap:wrap;margin-bottom:1rem}.playground-launch p{width:100%;margin:.25rem 0}.playground-launch label{flex:1;min-width:180px}.playground-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,320px),1fr));gap:1rem}.playground-controls{display:flex;gap:.5rem;flex-wrap:wrap;margin-top:.75rem}label{display:grid;gap:.35rem}.weight{display:block;font-size:2.4rem;font-weight:700;color:#087f6d;font-variant-numeric:tabular-nums}fieldset{border:0;padding:0;min-width:0}pre{white-space:pre-wrap;overflow-wrap:anywhere}.playground-error{color:#b91c1c}button{cursor:pointer}button:disabled{cursor:not-allowed}select,input{max-width:100%}
</style>
