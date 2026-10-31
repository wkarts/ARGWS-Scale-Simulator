<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onDeactivated, onMounted, ref, watch } from 'vue';
import AppModal from './AppModal.vue';
import type { ScaleSimulatorConfig } from '../services/scaleSimulator';
import { listModels, saveModel, deleteModel, modelForm, type SavedScaleModel } from '../services/savedScaleModels';
const props = defineProps<{ config: ScaleSimulatorConfig }>();
const emit = defineEmits<{ (event: 'load', config: ScaleSimulatorConfig): void }>();
const models = ref<SavedScaleModel[]>([]);
const selectedId = ref('');
const name = ref('');
const busy = ref(false);
const error = ref('');
const message = ref('');
const confirmRemoval = ref(false);
const dialog = ref<'save' | 'manage' | null>(null);
const nameInput = ref<HTMLInputElement | null>(null);
const selected = computed(() => models.value.find(model => model.id === selectedId.value));
const connection = (config: ScaleSimulatorConfig) => config.mode.startsWith('serial')
  ? `${config.serial_port} · ${config.baud_rate} baud` : `${config.host}:${config.port}`;
let mounted = true;
let opener: HTMLElement | null = null;
function closeDialog() {
  if (busy.value) return;
  dialog.value = null;
  confirmRemoval.value = false;
  void nextTick(() => opener?.focus());
}
function openDialog(kind: 'save' | 'manage', event: Event) {
  opener = event.currentTarget as HTMLElement;
  error.value = ''; message.value = ''; confirmRemoval.value = false;
  name.value = selected.value?.name ?? props.config.name.slice(0, 80);
  dialog.value = kind;
}
watch(dialog, async kind => { await nextTick(); if (kind === 'save') { nameInput.value?.focus(); nameInput.value?.select(); } });
onBeforeUnmount(() => { mounted = false; });
onDeactivated(() => { dialog.value = null; confirmRemoval.value = false; });
async function perform(operation: () => Promise<void>) {
  if (busy.value) return;
  busy.value = true; error.value = ''; message.value = '';
  try { await operation(); }
  catch (reason) { if (mounted) error.value = reason instanceof Error ? reason.message : String(reason); }
  finally { if (mounted) busy.value = false; }
}
async function reload() {
  await perform(async () => { const rows = await listModels(); if (mounted) models.value = rows; });
}
function select() { name.value = selected.value?.name ?? ''; confirmRemoval.value = false; error.value = ''; message.value = ''; }
function load() {
  if (busy.value || !selected.value) return;
  emit('load', modelForm(selected.value));
  message.value = 'Modelo carregado no formulário.';
  closeDialog();
}
async function save(update: boolean) {
  if (!name.value.trim() || (update && !selected.value)) return;
  const previous = update ? selected.value : undefined;
  await perform(async () => {
    const result = await saveModel(name.value, props.config, previous);
    if (!mounted) return;
    models.value = [...models.value.filter(model => model.id !== result.id), result]
      .sort((a, b) => a.name.localeCompare(b.name, 'pt-BR'));
    selectedId.value = result.id; name.value = result.name; confirmRemoval.value = false;
    dialog.value = null; message.value = 'Modelo salvo.';
    void nextTick(() => opener?.focus());
  });
}
async function remove() {
  if (!selected.value || !confirmRemoval.value) return;
  const model = selected.value;
  await perform(async () => {
    await deleteModel(model);
    if (!mounted) return;
    models.value = models.value.filter(item => item.id !== model.id);
    selectedId.value = ''; name.value = ''; confirmRemoval.value = false; message.value = 'Modelo removido.';
  });
}
onMounted(reload);
</script>
<template>
  <section class="saved-models" aria-label="Modelos salvos">
    <div class="model-toolbar">
      <select v-model="selectedId" aria-label="Modelo salvo" :disabled="busy" @change="select">
        <option value="">{{ models.length ? 'Modelos salvos…' : 'Nenhum modelo salvo' }}</option>
        <option v-for="model in models" :key="model.id" :value="model.id">{{ model.name }}</option>
      </select>
      <button type="button" class="model-button" :disabled="busy || !selected" @click="load">Carregar</button>
      <button type="button" class="model-button model-save" :disabled="busy" @click="openDialog('save', $event)">Salvar…</button>
      <button type="button" class="model-button model-more" aria-label="Gerenciar modelos salvos" title="Gerenciar modelos salvos" :disabled="busy" @click="openDialog('manage', $event)">⋯</button>
    </div>
    <p v-if="!dialog && error" class="model-feedback model-error" role="alert">{{ error }}</p>
    <p v-else-if="!dialog && message" class="model-feedback" role="status">{{ message }}</p>
    <AppModal v-if="dialog" :open="true" :title="dialog === 'save' ? 'Salvar configuração atual' : 'Gerenciar modelos'" width="sm" :close-on-backdrop="!busy" @close="closeDialog">
      <div class="model-editor">
        <template v-if="dialog === 'save'">
          <p class="model-help">Salva os parâmetros do formulário, sem alterar presets ou balanças em execução.</p>
          <p class="model-summary">{{ config.driver }} · {{ connection(config) }}</p>
          <label>Nome do modelo<input ref="nameInput" v-model="name" type="text" maxlength="80" :disabled="busy" autocomplete="off" placeholder="Ex.: Toledo bancada — COM10" @keydown.enter.prevent="save(false)" /></label>
          <p v-if="selected" class="model-help">Atualizar substitui a configuração de “{{ selected.name }}” pelos valores atuais do formulário.</p>
          <div class="model-dialog-actions">
            <button type="button" class="model-button model-save" :disabled="busy || !name.trim()" @click="save(false)">Salvar como novo</button>
            <button v-if="selected" type="button" class="model-button" :disabled="busy || !name.trim()" @click="save(true)">Atualizar selecionado</button>
          </div>
        </template>
        <template v-else>
          <div class="model-manage-row"><label>Modelo<select v-model="selectedId" :disabled="busy" @change="select"><option value="">Selecione um modelo</option><option v-for="model in models" :key="model.id" :value="model.id">{{ model.name }}</option></select></label><button type="button" class="model-button" :disabled="busy" @click="reload">Atualizar lista</button></div>
          <p v-if="selected" class="model-summary">{{ selected.config.driver }} · {{ connection(selected.config) }}</p>
          <p class="model-help">Para editar, carregue o modelo, ajuste o formulário e use Salvar → Atualizar selecionado.</p>
          <div class="model-dialog-actions"><button type="button" class="model-button" :disabled="busy || !selected" @click="load">Carregar / editar</button><button type="button" class="model-button model-danger" :disabled="busy || !selected" @click="confirmRemoval = true">Remover</button></div>
          <div v-if="confirmRemoval" class="model-confirm" role="alert"><p>Remover somente o modelo “{{ selected?.name }}”?</p><div class="model-dialog-actions"><button type="button" class="model-button model-danger" :disabled="busy" @click="remove">Confirmar remoção</button><button type="button" class="model-button" :disabled="busy" @click="confirmRemoval = false">Cancelar</button></div></div>
        </template>
        <p v-if="busy" class="model-feedback" role="status">Aguarde…</p><p v-if="error" class="model-feedback model-error" role="alert">{{ error }}</p><p v-if="message" class="model-feedback" role="status">{{ message }}</p>
      </div>
    </AppModal>
  </section>
</template>
<style scoped>
.saved-models{min-width:0;width:100%;margin:0 0 .75rem;container-type:inline-size;font:inherit}
.model-toolbar{display:grid;grid-template-columns:minmax(0,1fr) auto auto auto;align-items:center;gap:6px;min-width:0}
.saved-models select,.model-editor select,.model-editor input{box-sizing:border-box;width:100%;min-width:0;max-width:100%;min-height:36px;padding:7px 9px;border:1px solid var(--border-color,#cbd5e1);border-radius:8px;background:var(--surface-bg,#fff);color:inherit;font:inherit;font-size:.85rem}
.model-button{box-sizing:border-box;min-height:36px;padding:7px 11px;border:1px solid var(--border-color,#cbd5e1);border-radius:8px;background:var(--surface-bg,#fff);color:inherit;font:inherit;font-size:.82rem;line-height:1.25;cursor:pointer;white-space:normal}
.model-button:disabled{opacity:.5;cursor:not-allowed}.model-button:focus-visible,select:focus-visible,input:focus-visible{outline:2px solid var(--primary,#176b87);outline-offset:2px}
.model-save{background:var(--primary,#176b87);border-color:var(--primary,#176b87);color:#fff}.model-more{width:36px;padding:0;font-size:1.35rem;font-weight:700}.model-danger{color:#b42318;border-color:#f1b9b5}
.model-feedback{margin:6px 0 0;font-size:.8rem;line-height:1.4;overflow-wrap:anywhere}.model-error{color:#b42318}
.model-editor{display:grid;gap:12px;min-width:0;font:inherit}.model-editor label{display:grid;gap:6px;min-width:0;font-size:.88rem}.model-help{margin:0;font-size:.85rem;line-height:1.5;overflow-wrap:anywhere}.model-summary{margin:0;padding:8px 10px;border-radius:8px;background:var(--surface-muted-bg,#f3f6fa);font-size:.83rem;overflow-wrap:anywhere}
.model-dialog-actions{display:flex;gap:8px;flex-wrap:wrap}.model-dialog-actions>*{flex:1 1 130px}.model-manage-row{display:flex;gap:8px;align-items:end;flex-wrap:wrap}.model-manage-row label{flex:1 1 180px}.model-confirm{padding:10px;border:1px solid #f1b9b5;border-radius:8px}.model-confirm p{margin:0 0 8px;overflow-wrap:anywhere}
@container (max-width:320px){.model-toolbar{grid-template-columns:minmax(0,1fr) minmax(0,1fr) 36px}.model-toolbar>select{grid-column:1/-1}.model-button{padding-inline:8px}}
</style>
