import { createApp, h, ref } from 'vue';
import SavedScaleModels from '../../src/components/SavedScaleModels.vue';
import { defaultScaleConfig } from '../../src/services/scaleSimulator';
import '../../src/styles.css';
const config=ref(defaultScaleConfig());
(window as any).__activeInstances=[{id:'existing-instance',running:true,driver:'generic_ascii'}];
createApp({setup(){return()=>h('div',{style:'box-sizing:border-box;width:min(100%,532px);padding:16px;min-width:0;background:#fff'},[h(SavedScaleModels,{config:config.value,onLoad:(value:any)=>{config.value=value;(window as any).__loadedModel=value;}})]);}}).mount('#app');
