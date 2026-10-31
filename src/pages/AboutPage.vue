<script setup lang="ts">
import { onMounted, ref } from "vue";
import BasePage from "../components/base/BasePage.vue";
import BaseSectionCard from "../components/base/BaseSectionCard.vue";
import { getSystemInfo, type GenericRecord } from "../services/crud";
import { projectConfig } from "../config/projectConfig";
const info = ref<GenericRecord>({});
onMounted(async () => { try { info.value = await getSystemInfo(); } catch { info.value = {}; } });
</script>
<template>
  <BasePage title="Sobre o Scale Simulator" subtitle="Laboratório de pesagem para testes de integração.">
    <div class="grid columns-2 mobile-columns-1">
      <BaseSectionCard title="Scale Simulator">
        <p>{{ projectConfig.app.description }}</p>
        <div class="info-grid compact-info">
          <div class="info-item"><strong>Versão</strong><code>{{ projectConfig.app.version }}</code></div>
          <div class="info-item"><strong>Desenvolvedor</strong><span>{{ projectConfig.app.developer }}</span></div>
          <div v-if="info.build_hash && info.build_hash !== 'dev'" class="info-item"><strong>Build</strong><code>{{ info.build_hash }}</code></div>
        </div>
      </BaseSectionCard>
      <BaseSectionCard title="Indicadores e comunicação">
        <p>WT1000N, WT27R, WT3000IR, Toledo 8540, Toledo Prix 3, A12E, Líder LD2052, Filizola e ASCII genérico.</p>
        <p>TCP e RS232, envio contínuo ou por comando, múltiplas instâncias e inspeção de quadros no aplicativo desktop.</p>
      </BaseSectionCard>
      <BaseSectionCard title="Dados locais">
        <div class="info-grid compact-info">
          <div class="info-item"><strong>Banco local</strong><code>{{ info.db_path || 'Não informado pelo ambiente' }}</code></div>
          <div class="info-item"><strong>Diretório de dados</strong><code>{{ info.data_dir || 'Não informado pelo ambiente' }}</code></div>
        </div>
        <p class="muted-text">O identificador interno permanece compatível com instalações anteriores para preservar os dados existentes.</p>
      </BaseSectionCard>
    </div>
  </BasePage>
</template>
