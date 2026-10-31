<script setup lang="ts">
import {computed,ref} from 'vue';
import BasePage from '../components/base/BasePage.vue';
import BaseSectionCard from '../components/base/BaseSectionCard.vue';
import {simulatorGuide,guideMarkdown} from '../content/simulatorGuide';
const search=ref('');const sections=computed(()=>simulatorGuide.filter(s=>(s.title+' '+s.text).toLocaleLowerCase('pt-BR').includes(search.value.toLocaleLowerCase('pt-BR'))));
function download(){const url=URL.createObjectURL(new Blob([guideMarkdown()],{type:'text/markdown;charset=utf-8'}));const a=document.createElement('a');a.href=url;a.download='Scale-Simulator-Guia.md';a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);}
</script>
<template><BasePage title="Guia do Scale Simulator" subtitle="Orientação incorporada ao aplicativo, disponível sem depender de um site externo."><div class="guide-search"><label>Pesquisar orientação<input v-model="search" type="search" placeholder="Modelos, Playground, RS232, TCP…" /></label><button @click="download">Baixar guia em Markdown</button></div><div class="grid columns-2 mobile-columns-1"><BaseSectionCard v-for="section in sections" :key="section.title" :title="section.title"><p>{{section.text}}</p></BaseSectionCard></div><p v-if="!sections.length">Nenhum tópico encontrado.</p></BasePage></template>
<style scoped>.guide-search{display:flex;gap:1rem;align-items:end;flex-wrap:wrap;margin-bottom:1rem}.guide-search label{display:grid;gap:.5rem;flex:1;min-width:240px}p{line-height:1.7}</style>
