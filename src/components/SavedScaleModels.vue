<script setup lang="ts">
import {ref,computed,onMounted,onBeforeUnmount} from 'vue';
import type {ScaleSimulatorConfig} from '../services/scaleSimulator';
import {listModels,saveModel,deleteModel,modelForm,type SavedScaleModel} from '../services/savedScaleModels';
const props=defineProps<{config:ScaleSimulatorConfig}>();
const emit=defineEmits<{(event:'load',config:ScaleSimulatorConfig):void}>();
const models=ref<SavedScaleModel[]>([]),selectedId=ref(''),name=ref(''),busy=ref(false),error=ref(''),message=ref(''),confirmRemoval=ref(false);
const selected=computed(()=>models.value.find(x=>x.id===selectedId.value));let mounted=true;
onBeforeUnmount(()=>{mounted=false;});
async function perform(operation:()=>Promise<void>){if(busy.value)return;busy.value=true;error.value='';message.value='';try{await operation();}catch(e){if(mounted)error.value=e instanceof Error?e.message:String(e);}finally{if(mounted)busy.value=false;}}
async function reload(){await perform(async()=>{const result=await listModels();if(mounted)models.value=result;});}
function select(){name.value=selected.value?.name??'';confirmRemoval.value=false;}
function load(){if(selected.value){emit('load',modelForm(selected.value));message.value='Modelo carregado no formulário. Use Iniciar quando desejar.';}}
async function save(update=false){if(update&&!selected.value)return;await perform(async()=>{const result=await saveModel(name.value,props.config,update?selected.value:undefined);const rows=await listModels();if(!mounted)return;models.value=rows;selectedId.value=result.id;name.value=result.name;confirmRemoval.value=false;message.value='Modelo salvo. Instâncias em execução não foram alteradas.';});}
async function remove(){if(!selected.value||!confirmRemoval.value)return;const model=selected.value;await perform(async()=>{await deleteModel(model);const rows=await listModels();if(!mounted)return;models.value=rows;selectedId.value='';name.value='';confirmRemoval.value=false;message.value='Modelo removido. Nenhuma instância foi interrompida.';});}
onMounted(reload);
</script>
<template>
<section class="saved-models" aria-label="Modelos salvos">
<h4>Modelos salvos</h4><p>Guarde configurações independentes dos presets de fábrica. Carregar preenche somente o formulário.</p>
<fieldset :disabled="busy"><label>Modelo<select v-model="selectedId" @change="select"><option value="">Selecione um modelo</option><option v-for="model in models" :key="model.id" :value="model.id">{{model.name}}</option></select></label>
<label>Nome do modelo<input v-model="name" maxlength="80" placeholder="Ex.: Toledo bancada COM10" /></label>
<div class="model-actions"><button type="button" :disabled="!selected" @click="load">Carregar</button><button type="button" :disabled="!name.trim()" @click="save(false)">Salvar como novo</button><button type="button" :disabled="!selected || !name.trim()" @click="save(true)">Atualizar selecionado</button><button type="button" :disabled="!selected" @click="confirmRemoval=true">Remover</button><button type="button" @click="reload">Recarregar lista</button></div>
<div v-if="confirmRemoval"><p>Remover somente o modelo “{{selected?.name}}”?</p><button type="button" @click="remove">Confirmar remoção</button><button type="button" @click="confirmRemoval=false">Cancelar</button></div></fieldset>
<p v-if="error" role="alert">{{error}}</p><p v-if="message" role="status">{{message}}</p></section>
</template>
<style scoped>
.saved-models{padding:1rem;border:1px solid var(--border-color,#cbd5e1);border-radius:10px;margin-bottom:1rem}.saved-models h4{margin:0}.saved-models p{font-size:.85rem}.saved-models fieldset{border:0;padding:0;min-width:0;display:grid;gap:.75rem}.saved-models label{display:grid;gap:.3rem}.model-actions{display:flex;gap:.5rem;flex-wrap:wrap}.saved-models input,.saved-models select{width:100%;min-width:0}.saved-models [role=alert]{color:#b91c1c}
</style>
